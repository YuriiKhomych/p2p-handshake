use clap::Subcommand;
use reth_primitives::NodeRecord;

#[derive(Subcommand, Debug)]
pub enum Commands {
    Eth { nodes_addrs: Vec<NodeRecord> },
}
