use measure_time::{debug_time, info_time};
use reth_ecies::stream::ECIESStream;
use reth_primitives::NodeRecord;
use secp256k1::SecretKey;
use std::{net::IpAddr, time::Duration};
use tokio::net::TcpStream;
use tracing::instrument;

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
#[instrument(level = "trace", skip_all, fields(peer=&*format!("{:?}", config.peer.address)))]
pub async fn handshake(config: Config) -> eyre::Result<IpAddr> {
    info_time!("[{:?}] Perform a P2P handshake", config.peer.address);

    let key = SecretKey::new(&mut rand::thread_rng());
    let ecies_stream = {
        debug_time!(
            "[{:?}] Send and Parse the ECIES auth message",
            config.peer.address
        );

        // Connect to the peer and perform the ECIES handshake
        let outgoing = tokio::time::timeout(
            Duration::from_millis(config.timeout),
            TcpStream::connect((config.peer.address, config.peer.tcp_port)),
        )
        .await??;
        ECIESStream::connect(outgoing, key, config.peer.id).await?
    };
    {
        // Send, Parse the P2P Hello message and perform the initial handshake
        debug_time!(
            "[{:?}] Send, Parse the P2P Hello message and perform the initial handshake",
            config.peer.address
        );
        let hello_msg = create_hello_msg(key);
        stream::P2PStream::new(ecies_stream)
            .handshake(hello_msg, config.timeout)
            .await?;
    }

    Ok(config.peer.address)
}
