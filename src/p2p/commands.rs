use std::net::SocketAddr;

use clap::Subcommand;
use reth_primitives::NodeRecord;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Perform a P2P handshake with the ethereum network nodes
    Eth { nodes_addrs: Vec<NodeRecord> },
    /// Perform a P2P handshake with the bitcoin network nodes
    Btc {
        nodes_addrs: Vec<SocketAddr>,
        #[arg(
            long,
            short,
            help = "the user agent to be used during handshake operation",
            default_value = "/Satoshi:25.0.0/"
        )]
        user_agent: String,
    },
}
