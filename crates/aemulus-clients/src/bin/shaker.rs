use aemulus_proto::sila2::{
    dev::arturgalstyan::shakers::shakercontroller::v1::{
        StartShakingParameters,
        shaker_controller_client::ShakerControllerClient,
    },
    org::silastandard::Integer,
};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50053").connect_lazy();
    let mut shaker_client = ShakerControllerClient::new(channel);

    shaker_client
        .start_shaking(StartShakingParameters {
            speed: Some(Integer { value: 2500 }),
        })
        .await?;

    // shaker_client.stop_shaking(StopShakingParameters {}).await?;

    Ok(())
}
