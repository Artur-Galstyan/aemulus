use aemulus_proto::aemulus::sim::v1::{
    GetJointRequest, SetActuatorRequest, sim_service_client::SimServiceClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://127.0.0.1:5000")
        .connect()
        .await?;

    let mut client = SimServiceClient::new(channel);

    let get_joint_request = GetJointRequest {
        joint: "j1_z".to_string(),
    };

    let response = client.get_joint(get_joint_request).await?;

    println!("{}", response.into_inner().position);

    let set_joint_request = SetActuatorRequest {
        actuator: "j1_z".to_string(),
        value: 0.2,
    };

    let response = client.set_actuator(set_joint_request).await?;

    println!("{:?}", response);

    println!("sleeping for 2 seconds");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let get_joint_request = GetJointRequest {
        joint: "j1_z".to_string(),
    };

    let response = client.get_joint(get_joint_request).await?;
    println!("{}", response.into_inner().position);

    Ok(())
}
