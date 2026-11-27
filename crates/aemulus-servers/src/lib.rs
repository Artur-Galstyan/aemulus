use std::sync::Arc;

use aemulus_proto::{
    sila2::dev::arturgalstyan::robots::{
        bioshake::v1::bioshake_server::{self, BioshakeServer},
        pf400::v1::pf400_server::{self, Pf400Server},
    },
    sim::v1::sim_service_client::SimServiceClient,
};
use tonic::{
    codec::CompressionEncoding,
    transport::{Channel, Server},
};

use crate::devices::{bioshake::sim::BioshakeSim, pf400::sim::Pf400Sim};

pub mod devices;

pub async fn serve_sim(
    device_name: String,
    device_id: String,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50051").connect_lazy();

    match device_id.as_str() {
        pf400_server::SERVICE_NAME => {
            let pf400 = Pf400Sim {
                sim: SimServiceClient::new(channel),
                moves: Arc::default(),
                device_name,
            };

            Server::builder()
                .add_service(Pf400Server::new(pf400).accept_compressed(CompressionEncoding::Gzip))
                .serve(format!("127.0.0.1:{}", port).parse()?)
                .await?;
        }
        bioshake_server::SERVICE_NAME => {
            let bioshake = BioshakeSim {
                sim: SimServiceClient::new(channel),
                device_name,
            };

            Server::builder()
                .add_service(
                    BioshakeServer::new(bioshake).accept_compressed(CompressionEncoding::Gzip),
                )
                .serve(format!("127.0.0.1:{}", port).parse()?)
                .await?;
        }
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "device not found",
            )));
        }
    }

    Ok(())
}

pub fn start_sim_server(device_name: String, device_id: String, port: u16) -> std::io::Result<()> {
    let runtime = tokio::runtime::Runtime::new()?;

    std::thread::spawn(move || {
        let result: Result<(), Box<dyn std::error::Error>> =
            runtime.block_on(async { serve_sim(device_name, device_id, port).await });
        if let Err(e) = result {
            tracing::error!("sim server error: {e}");
        }
    });

    Ok(())
}
