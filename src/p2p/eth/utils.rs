use reth_ecies::util::pk2id;
use reth_eth_wire::HelloMessage;
use secp256k1::{SecretKey, SECP256K1};

/// Create a P2P Hello message
pub fn create_hello_msg(key: SecretKey) -> HelloMessage {
    let our_peer_id = pk2id(&key.public_key(SECP256K1));
    HelloMessage::builder(our_peer_id).build()
}
