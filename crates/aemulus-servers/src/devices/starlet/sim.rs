use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use aemulus_proto::{
    sila2::{
        dev::arturgalstyan::robots::starlet::v1::{
            MoveEjectParameters, MoveEjectResponses, MovePlungerParameters, MovePlungerResponses,
            MoveXParameters, MoveXResponses, MoveYParameters, MoveYResponses, MoveZParameters,
            MoveZResponses,
            starlet_server::{Starlet, StarletServer},
        },
        org::silastandard::{
            CommandConfirmation, CommandExecutionUuid, ExecutionInfo, Integer, Real,
            execution_info::CommandStatus,
        },
    },
    sim::v1::{GetJointRequest, SetActuatorRequest, sim_service_client::SimServiceClient},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{
    Response,
    codec::CompressionEncoding,
    transport::{Channel, Server},
};
use tracing::info;
use uuid::Uuid;

struct StarletSim {
    sim: SimServiceClient<Channel>,
    moves: Arc<Mutex<HashMap<String, (CommandStatus, f64)>>>,
    device_name: String,
}

fn channel_index(channel: Option<Integer>) -> Result<i64, tonic::Status> {
    let Some(channel) = channel else {
        return Err(tonic::Status::invalid_argument("channel is required"));
    };
    if !(0..8).contains(&channel.value) {
        return Err(tonic::Status::invalid_argument(
            "channel must be between 0 and 7",
        ));
    }
    Ok(channel.value)
}

impl StarletSim {
    async fn start_move(
        &self,
        axis: String,
        target: f64,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        let mut sim = self.sim.clone();
        let joint_name = format!("{}/{}", self.device_name, axis);

        let get_start_pos_request = GetJointRequest {
            joint: joint_name.clone(),
        };
        let get_start_pos_response = sim.get_joint(get_start_pos_request).await?;
        let start_pos = get_start_pos_response.into_inner().position;

        info!("Request {axis} position: {target}");
        let set_actuator_request = SetActuatorRequest {
            actuator: joint_name.clone(),
            value: target,
        };
        sim.set_actuator(set_actuator_request).await?;

        let move_uuid = Uuid::new_v4();
        let Ok(mut locked_hashmap) = self.moves.lock() else {
            return Err(tonic::Status::internal("failed to lock moves"));
        };
        locked_hashmap.insert(move_uuid.to_string(), (CommandStatus::Running, 0.0));

        let moves = Arc::clone(&self.moves);
        let id = move_uuid.to_string();

        tokio::spawn(async move {
            loop {
                let get_joint_request = GetJointRequest {
                    joint: joint_name.clone(),
                };

                let Ok(response) = sim.get_joint(get_joint_request).await else {
                    if let Ok(mut moves) = moves.lock() {
                        moves.insert(id.clone(), (CommandStatus::FinishedWithError, 0.0));
                    }

                    break;
                };
                let joint_response = response.into_inner();
                let mut status = CommandStatus::Running;
                if (joint_response.position - target).abs() < 0.001 {
                    status = CommandStatus::FinishedSuccessfully;
                }

                let current = joint_response.position;
                let total = (target - start_pos).abs();
                let progress = if total > 0.0 {
                    (1.0 - (target - current).abs() / total).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                if let Ok(mut moves) = moves.lock() {
                    moves.insert(id.clone(), (status, progress));
                }

                if status == CommandStatus::FinishedSuccessfully {
                    break;
                }

                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        Ok(tonic::Response::new(CommandConfirmation {
            command_execution_uuid: Some(CommandExecutionUuid {
                value: move_uuid.to_string(),
            }),
            lifetime_of_execution: None,
        }))
    }

    fn move_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<tonic::codegen::BoxStream<ExecutionInfo>>, tonic::Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let moves = Arc::clone(&self.moves);
        let id = request.into_inner().value;

        if let Ok(moves) = moves.lock()
            && !moves.contains_key(&id)
        {
            return Err(tonic::Status::not_found("move not found"));
        }

        tokio::spawn(async move {
            loop {
                let (state, progress) = {
                    let Ok(moves) = moves.lock() else { break };
                    let Some(row) = moves.get(&id) else { break };
                    (row.0, row.1)
                };

                let message = ExecutionInfo {
                    command_status: state as i32,
                    estimated_remaining_time: None,
                    updated_lifetime_of_execution: None,
                    progress_info: Some(Real { value: progress }),
                };

                if tx.send(Ok(message)).await.is_err() {
                    break;
                }
                if state == CommandStatus::FinishedSuccessfully
                    || state == CommandStatus::FinishedWithError
                {
                    break;
                }

                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    fn move_result<R: Default>(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<R>, tonic::Status> {
        let id = request.into_inner().value;
        let Ok(moves) = self.moves.lock() else {
            return Err(tonic::Status::internal("failed to lock moves"));
        };
        let Some((state, _)) = moves.get(&id) else {
            return Err(tonic::Status::not_found("move not found"));
        };

        match state {
            CommandStatus::FinishedSuccessfully => Ok(Response::new(R::default())),
            CommandStatus::FinishedWithError => Err(tonic::Status::aborted("move failed")),
            _ => Err(tonic::Status::aborted("move is still running")),
        }
    }
}

#[tonic::async_trait]
impl Starlet for StarletSim {
    type MoveX_InfoStream = tonic::codegen::BoxStream<ExecutionInfo>;
    type MoveY_InfoStream = tonic::codegen::BoxStream<ExecutionInfo>;
    type MoveZ_InfoStream = tonic::codegen::BoxStream<ExecutionInfo>;
    type MovePlunger_InfoStream = tonic::codegen::BoxStream<ExecutionInfo>;
    type MoveEject_InfoStream = tonic::codegen::BoxStream<ExecutionInfo>;

    async fn move_x(
        &self,
        request: tonic::Request<MoveXParameters>,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        info!("Got move_x request");
        let Some(position) = request.into_inner().position else {
            return Err(tonic::Status::invalid_argument("position is required"));
        };
        self.start_move("x".to_string(), position.value).await
    }

    async fn move_x_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<Self::MoveX_InfoStream>, tonic::Status> {
        self.move_info(request)
    }

    async fn move_x_result(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<MoveXResponses>, tonic::Status> {
        self.move_result(request)
    }

    async fn move_y(
        &self,
        request: tonic::Request<MoveYParameters>,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        info!("Got move_y request");
        let params = request.into_inner();
        let channel = channel_index(params.channel)?;
        let Some(position) = params.position else {
            return Err(tonic::Status::invalid_argument("position is required"));
        };
        self.start_move(format!("y{channel}"), position.value).await
    }

    async fn move_y_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<Self::MoveY_InfoStream>, tonic::Status> {
        self.move_info(request)
    }

    async fn move_y_result(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<MoveYResponses>, tonic::Status> {
        self.move_result(request)
    }

    async fn move_z(
        &self,
        request: tonic::Request<MoveZParameters>,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        info!("Got move_z request");
        let params = request.into_inner();
        let channel = channel_index(params.channel)?;
        let Some(position) = params.position else {
            return Err(tonic::Status::invalid_argument("position is required"));
        };
        self.start_move(format!("z{channel}"), position.value).await
    }

    async fn move_z_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<Self::MoveZ_InfoStream>, tonic::Status> {
        self.move_info(request)
    }

    async fn move_z_result(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<MoveZResponses>, tonic::Status> {
        self.move_result(request)
    }

    async fn move_plunger(
        &self,
        request: tonic::Request<MovePlungerParameters>,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        info!("Got move_plunger request");
        let params = request.into_inner();
        let channel = channel_index(params.channel)?;
        let Some(position) = params.position else {
            return Err(tonic::Status::invalid_argument("position is required"));
        };
        self.start_move(format!("plunger{channel}"), position.value)
            .await
    }

    async fn move_plunger_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<Self::MovePlunger_InfoStream>, tonic::Status> {
        self.move_info(request)
    }

    async fn move_plunger_result(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<MovePlungerResponses>, tonic::Status> {
        self.move_result(request)
    }

    async fn move_eject(
        &self,
        request: tonic::Request<MoveEjectParameters>,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        info!("Got move_eject request");
        let params = request.into_inner();
        let channel = channel_index(params.channel)?;
        let Some(position) = params.position else {
            return Err(tonic::Status::invalid_argument("position is required"));
        };
        self.start_move(format!("eject{channel}"), position.value)
            .await
    }

    async fn move_eject_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<Self::MoveEject_InfoStream>, tonic::Status> {
        self.move_info(request)
    }

    async fn move_eject_result(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<MoveEjectResponses>, tonic::Status> {
        self.move_result(request)
    }
}

pub async fn serve_starlet_sim(device_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50051").connect_lazy();
    let starlet = StarletSim {
        sim: SimServiceClient::new(channel),
        moves: Arc::default(),
        device_name,
    };

    Server::builder()
        .add_service(StarletServer::new(starlet).accept_compressed(CompressionEncoding::Gzip))
        .serve("127.0.0.1:50053".parse()?)
        .await?;

    Ok(())
}

pub fn start_sim_server(device_name: String) -> std::io::Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;

    std::thread::spawn(move || {
        let result: Result<(), Box<dyn std::error::Error>> =
            runtime.block_on(async { serve_starlet_sim(device_name).await });
        if let Err(e) = result {
            tracing::error!("sim server error: {e}");
        }
    });

    Ok(())
}
