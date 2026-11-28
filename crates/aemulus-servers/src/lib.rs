use std::sync::Arc;

use aemulus_proto::{
    sila2::dev::arturgalstyan::{
        robots::pf400::v1::pf400_server::Pf400Server,
        shakers::shakercontroller::v1::shaker_controller_server::ShakerControllerServer,
    },
    sim::v1::sim_service_client::SimServiceClient,
};
use tonic::{
    codec::CompressionEncoding,
    transport::{Channel, Server},
};

use crate::{
    devices::{pf400::sim::Pf400Sim, shaker::sim::ShakerSim},
    models::Kind,
};

pub mod devices;
pub mod models;

pub async fn serve_sim(
    device_name: String,
    model: models::ModelIdentifier,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let sim = SimServiceClient::new(Channel::from_static("http://127.0.0.1:50051").connect_lazy());
    let router = match model.kind() {
        Kind::Pf400 => Server::builder().add_service(
            Pf400Server::new(Pf400Sim {
                sim,
                moves: Arc::default(),
                device_name,
            })
            .accept_compressed(CompressionEncoding::Gzip),
        ),
        Kind::Shaker(spec) => Server::builder().add_service(
            ShakerControllerServer::new(ShakerSim {
                sim,
                device_name,
                spec,
            })
            .accept_compressed(CompressionEncoding::Gzip),
        ),
    };

    router.serve(format!("127.0.0.1:{port}").parse()?).await?;
    Ok(())
}

pub fn start_sim_server(
    device_name: String,
    model: models::ModelIdentifier,
    port: u16,
) -> std::io::Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;

    std::thread::spawn(move || {
        let result: Result<(), Box<dyn std::error::Error>> =
            runtime.block_on(async { serve_sim(device_name, model, port).await });
        if let Err(e) = result {
            tracing::error!("sim server error: {e}");
        }
    });

    Ok(())
}
