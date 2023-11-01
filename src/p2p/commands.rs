use std::net::SocketAddr;

use clap::Subcommand;
use reth_primitives::NodeRecord;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Eth {
        nodes_addrs: Vec<NodeRecord>,
    },
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
