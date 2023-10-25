use reth_ecies::stream::ECIESStream;
use reth_primitives::NodeRecord;
use secp256k1::SecretKey;
use std::{net::IpAddr, time::Duration};
use tokio::net::TcpStream;

use crate::p2p::eth::utils::create_hello_msg;

mod constants;
mod stream;
mod utils;

pub use constants::HANDSHAKE_TIMEOUT;

pub struct Config {
    pub timeout: u64,
    pub peer: NodeRecord,
}

/// Perform a P2P handshake with a peer
pub async fn handshake(config: Config) -> eyre::Result<IpAddr> {
    let key = SecretKey::new(&mut rand::thread_rng());

    let outgoing = tokio::time::timeout(
        Duration::from_millis(config.timeout),
        TcpStream::connect((config.peer.address, config.peer.tcp_port)),
    )
    .await??;

    let ecies_stream = ECIESStream::connect(outgoing, key, config.peer.id).await?;

    let hello_msg = create_hello_msg(key);
    stream::P2PStream::new(ecies_stream)
        .handshake(hello_msg, config.timeout)
        .await?;

    Ok(config.peer.address)
}
