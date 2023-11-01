use clap::{command, Parser};

use crate::p2p::commands::Commands;

/// [`HANDSHAKE_TIMEOUT`] determines the amount of time to wait before determining that a `p2p`
/// handshake has timed out.
pub const HANDSHAKE_TIMEOUT: u64 = 1000;

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
