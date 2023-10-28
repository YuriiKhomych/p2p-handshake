use p2p_handshake::p2p::eth::{handshake, Config, HANDSHAKE_TIMEOUT};
use reth_primitives::holesky_nodes;

#[tokio::test]
async fn test_eth_handshake() {
    // Use the holesky nodes to avoid bothering the Ethereum mainnet
    let nodes_addrs = holesky_nodes();

    // Iterate over the nodes and perform the P2P handshake
    for peer in nodes_addrs {
        let res = handshake(Config {
            timeout: HANDSHAKE_TIMEOUT,
            peer,
        })
        .await;

        assert!(res.is_ok());
    }
}
