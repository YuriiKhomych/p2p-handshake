use p2p_handshake::p2p::eth::{self, HANDSHAKE_TIMEOUT};
use reth_primitives::holesky_nodes;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let nodes_addrs = holesky_nodes();
    for node in nodes_addrs {
        let _ = eth::handshake(eth::Config {
            timeout: HANDSHAKE_TIMEOUT,
            peer: node,
        })
        .await;
    }
    Ok(())
}
