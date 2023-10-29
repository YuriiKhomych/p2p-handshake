use clap::Parser;
use p2p_handshake::{
    p2p::{config::Config, handshake},
    telemetry::init_tracing,
};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Initialize tracing
    init_tracing();

    // Parse the CLI arguments and perform the P2P handshake for corresponding network
    let config = Config::parse();
    handshake(config).await?;

    Ok(())
}
