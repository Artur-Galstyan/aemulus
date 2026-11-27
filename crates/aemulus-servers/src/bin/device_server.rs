use aemulus_servers::serve_sim;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,

    #[arg(short, long)]
    device_name: String,

    #[arg(short, long)]
    device_id: String,

    #[arg(short, long)]
    port: u16,
}

#[derive(Subcommand)]
enum Mode {
    Sim,
    Real,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing_subscriber::fmt::try_init();

    let cli = Cli::parse();

    match cli.mode {
        Mode::Sim => serve_sim(cli.device_name, cli.device_id, cli.port).await,
        Mode::Real => todo!(),
    }
}
