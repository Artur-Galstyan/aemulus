use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("TODO");
    info!("Starting pf400 real server");

    Ok(())
}
