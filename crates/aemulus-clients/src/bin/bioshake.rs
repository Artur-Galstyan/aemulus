use aemulus_proto::sila2::{
    dev::arturgalstyan::robots::{
        bioshake::v1::{
            StartShakingParameters, StopShakingParameters, bioshake_client::BioshakeClient,
        },
        pf400::v1::{MoveZParameters, pf400_client::Pf400Client},
    },
    org::silastandard::{Integer, Real},
};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50053").connect_lazy();
    let mut bioshake_client = BioshakeClient::new(channel);

    // bioshake_client
    //     .start_shaking(StartShakingParameters {
    //         speed: Some(Integer { value: 2500 }),
    //     })
    //     .await?;

    bioshake_client
        .stop_shaking(StopShakingParameters {})
        .await?;

    Ok(())
}
