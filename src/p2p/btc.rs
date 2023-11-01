use measure_time::info_time;
use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};
use tokio::net::TcpStream;
use tracing::instrument;

use self::stream::MessageStream;
use crate::p2p::error::P2PError;

pub mod codec;
pub mod stream;

#[derive(Debug)]
pub struct Config {
    pub node_address: SocketAddr,
    pub timeout: u64,
    pub user_agent: String,
}

/// Perform a P2P handshake with a peer
#[instrument(level = "trace", skip_all, fields(peer=&*format!("{:?}", config.node_address)))]
pub async fn handshake(config: Config) -> Result<IpAddr, P2PError> {
    info_time!("[{:?}] Perform a P2P handshake", config.node_address);

    // Connect to the peer and perform the bitcoin network handshake
    let transport = tokio::time::timeout(
        Duration::from_millis(config.timeout),
        TcpStream::connect(config.node_address),
    )
    .await??;

    MessageStream::new(config.node_address, config.user_agent)
        .handshake(transport)
        .await?;

    Ok(config.node_address.ip())
}
