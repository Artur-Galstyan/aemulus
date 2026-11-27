use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use aemulus_proto::{
    sila2::{
        dev::arturgalstyan::robots::pf400::v1::{
            MoveZParameters, MoveZResponses, SubscribePositionZParameters,
            SubscribePositionZResponses, pf400_server::Pf400,
        },
        org::silastandard::{
            CommandConfirmation, CommandExecutionUuid, ExecutionInfo, Real,
            execution_info::CommandStatus,
        },
    },
    sim::v1::{GetJointRequest, SetActuatorRequest, sim_service_client::SimServiceClient},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Response, transport::Channel};
use tracing::info;
use uuid::Uuid;

pub(crate) struct Pf400Sim {
    pub sim: SimServiceClient<Channel>,
    pub moves: Arc<Mutex<HashMap<String, (CommandStatus, f64)>>>,
    pub device_name: String,
}

#[tonic::async_trait]
impl Pf400 for Pf400Sim {
    type MoveZ_InfoStream = tonic::codegen::BoxStream<ExecutionInfo>;
    type Subscribe_PositionZStream = tonic::codegen::BoxStream<SubscribePositionZResponses>;

    async fn move_z(
        &self,
        request: tonic::Request<MoveZParameters>,
    ) -> Result<tonic::Response<CommandConfirmation>, tonic::Status> {
        info!("Got move_z request");
        let Some(height) = request.into_inner().height else {
            return Err(tonic::Status::invalid_argument("height is required"));
        };

        let mut sim = self.sim.clone();

        let get_start_pos_request = GetJointRequest {
            joint: format!("{}/j1_z", self.device_name),
        };
        let get_start_pos_response = sim.get_joint(get_start_pos_request).await?;
        let start_pos = get_start_pos_response.into_inner().position;

        info!("Request height: {}", height.value);
        let set_actuator_request = SetActuatorRequest {
            actuator: format!("{}/j1_z", self.device_name),
            value: height.value,
        };
        sim.set_actuator(set_actuator_request).await?;

        let move_uuid = Uuid::new_v4();
        let Ok(mut locked_hashmap) = self.moves.lock() else {
            return Err(tonic::Status::internal("failed to lock moves"));
        };
        locked_hashmap.insert(move_uuid.to_string(), (CommandStatus::Running, 0.0));

        let moves = Arc::clone(&self.moves);
        let id = move_uuid.to_string();
        let device_name = self.device_name.clone();

        tokio::spawn(async move {
            loop {
                let get_joint_request = GetJointRequest {
                    joint: format!("{}/j1_z", device_name),
                };

                let Ok(response) = sim.get_joint(get_joint_request).await else {
                    if let Ok(mut moves) = moves.lock() {
                        moves.insert(id.clone(), (CommandStatus::FinishedWithError, 0.0));
                    }

                    break;
                };
                let joint_response = response.into_inner();
                let mut status = CommandStatus::Running;
                if (joint_response.position - height.value).abs() < 0.001 {
                    status = CommandStatus::FinishedSuccessfully;
                }

                let current = joint_response.position;
                let target = height.value;
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

    async fn move_z_info(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<Self::MoveZ_InfoStream>, tonic::Status> {
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

    async fn move_z_result(
        &self,
        request: tonic::Request<CommandExecutionUuid>,
    ) -> Result<tonic::Response<MoveZResponses>, tonic::Status> {
        let id = request.into_inner().value;
        let Ok(moves) = self.moves.lock() else {
            return Err(tonic::Status::internal("failed to lock moves"));
        };
        let Some((state, _)) = moves.get(&id) else {
            return Err(tonic::Status::not_found("move not found"));
        };

        match state {
            CommandStatus::FinishedSuccessfully => Ok(Response::new(MoveZResponses {})),
            CommandStatus::FinishedWithError => Err(tonic::Status::aborted("move failed")),
            _ => Err(tonic::Status::aborted("move is still running")),
        }
    }

    async fn subscribe_position_z(
        &self,
        _request: tonic::Request<SubscribePositionZParameters>,
    ) -> Result<tonic::Response<Self::Subscribe_PositionZStream>, tonic::Status> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let mut sim = self.sim.clone();
        let device_name = self.device_name.clone();

        tokio::spawn(async move {
            loop {
                let get_joint_request = GetJointRequest {
                    joint: format!("{}/j1_z", device_name),
                };

                let response = match sim.get_joint(get_joint_request).await {
                    Ok(response) => response,
                    Err(status) => {
                        let _ = tx.send(Err(status)).await;
                        break;
                    }
                };
                let joint_response = response.into_inner();

                let message = SubscribePositionZResponses {
                    position_z: Some(Real {
                        value: joint_response.position,
                    }),
                };

                if tx.send(Ok(message)).await.is_err() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }
}
