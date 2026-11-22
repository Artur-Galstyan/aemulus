use aemulus_proto::sila2::{
    dev::arturgalstyan::robots::pf400::v1::{MoveZParameters, pf400_client::Pf400Client},
    org::silastandard::Real,
};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static("http://127.0.0.1:50052").connect_lazy();
    let mut pf400_client = Pf400Client::new(channel);

    let move_z_request = MoveZParameters {
        height: Some(Real { value: 0.0 }),
    };

    let cmd_uuid = pf400_client.move_z(move_z_request).await?;

    println!("Command UUID: {:?}", cmd_uuid);
    let cmd_id = cmd_uuid.into_inner().command_execution_uuid.unwrap();
    let response = pf400_client.move_z_info(cmd_id.clone()).await?;

    let mut stream = response.into_inner();
    while let Some(info) = stream.message().await? {
        println!("Execution info: {:?}", info);
    }

    let move_z_result = pf400_client.move_z_result(cmd_id.clone()).await?;
    println!("Move Z result: {:?}", move_z_result);

    Ok(())
}
