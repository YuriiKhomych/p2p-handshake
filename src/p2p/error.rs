use reth_ecies::ECIESError;
use reth_eth_wire::errors::P2PStreamError;
use std::fmt::Display;

#[derive(thiserror::Error, Debug)]
pub enum P2PError {
    #[error("{0}: P2P handshake error")]
    P2PHandshakeError(P2PHandshake),
    #[error("{0}: ECIES error make a retry in couple of minutes")]
    ECIESError(#[from] ECIESError),
    #[error("{0}: IO error")]
    IOError(#[from] std::io::Error),
    #[error("{0}: Tokio elapsed error")]
    TokioElapsedError(#[from] tokio::time::error::Elapsed),
    #[error("{0}: P2P stream error")]
    P2PStreamError(#[from] P2PStreamError),
}

#[derive(Debug)]
pub struct P2PHandshake {
    message: String,
    address: String,
}

impl P2PHandshake {
    pub fn new(err: P2PError, address: String) -> Self {
        Self {
            message: err.to_string(),
            address,
        }
    }
}

impl Display for P2PHandshake {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[failed] [{}] error: {}", self.address, self.message)
    }
}
