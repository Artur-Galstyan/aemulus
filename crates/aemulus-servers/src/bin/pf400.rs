use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Mode,

    #[arg(short, long)]
    device_name: String,
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
        Mode::Sim => aemulus_servers::devices::pf400::sim::serve_pf400_sim(cli.device_name).await,
        Mode::Real => todo!(),
    }
}
