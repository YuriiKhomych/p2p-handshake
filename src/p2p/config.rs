use clap::{command, Parser};

use super::{commands::Commands, eth::HANDSHAKE_TIMEOUT};

#[derive(Parser, Debug)]
#[command(version)]
#[command(propagate_version = true)]
pub struct Config {
    #[arg(
        long,
        short,
        default_value_t = HANDSHAKE_TIMEOUT,
        help = "handshake operation maximum time (in ms)"
    )]
    pub timeout: u64,
    #[command(subcommand)]
    pub commands: Commands,
}
