use aemulus_proto::aemulus::{
    pf400::v1::{
        MoveZRequest, MoveZResponse, ReadZRequest, ReadZResponse,
        pf400_service_server::{Pf400Service, Pf400ServiceServer},
    },
    sim::v1::{GetJointRequest, SetActuatorRequest, sim_service_client::SimServiceClient},
};
use tonic::{
    codec::CompressionEncoding,
    transport::{Channel, Server},
};
use tracing::info;

struct Pf400 {
    sim: SimServiceClient<Channel>,
}

#[tonic::async_trait]
impl Pf400Service for Pf400 {
    async fn move_z(
        &self,
        request: tonic::Request<MoveZRequest>,
    ) -> Result<tonic::Response<MoveZResponse>, tonic::Status> {
        info!("Got move_z request");
        let mut sim = self.sim.clone();
        let height = request.into_inner().height;
        info!("Request height: {}", height);
        let set_actuator_request = SetActuatorRequest {
            actuator: "j1_z".to_string(),
            value: height,
        };
        sim.set_actuator(set_actuator_request).await?;
        Ok(tonic::Response::new(MoveZResponse { position: height }))
    }

    async fn read_z(
        &self,
        _request: tonic::Request<ReadZRequest>,
    ) -> Result<tonic::Response<ReadZResponse>, tonic::Status> {
        info!("Got read_z request");
        let mut sim = self.sim.clone();
        let get_joint_request = GetJointRequest {
            joint: "j1_z".to_string(),
        };
        let position = sim.get_joint(get_joint_request).await?;
        Ok(tonic::Response::new(ReadZResponse {
            position: position.into_inner().position,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let channel = Channel::from_static("http://127.0.0.1:50051").connect_lazy();
    let pf400 = Pf400 {
        sim: SimServiceClient::new(channel),
    };

    Server::builder()
        .add_service(Pf400ServiceServer::new(pf400).accept_compressed(CompressionEncoding::Gzip))
        .serve("127.0.0.1:50052".parse()?)
        .await?;

    Ok(())
}
