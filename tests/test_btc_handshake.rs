use p2p_handshake::p2p::{
    btc::{handshake, Config},
    config::HANDSHAKE_TIMEOUT,
};

#[tokio::test]
async fn test_btc_handshake() {
    // Use the hardcoded nodes to perform the P2P handshake
    let nodes_addrs = vec![
        "178.238.233.75:8333",
        "108.208.224.205:8333",
        "54.228.116.186:48333",
        "96.126.123.143:8333",
    ];

    for address in nodes_addrs {
        // Iterate over the nodes and perform the P2P handshake
        let res = handshake(Config {
            node_address: address.parse().unwrap(),
            timeout: HANDSHAKE_TIMEOUT,
            user_agent: "/Satoshi:25.0.0/".to_string(),
        })
        .await;

        assert!(res.is_ok());
    }
}
