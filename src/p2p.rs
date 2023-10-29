use std::net::IpAddr;

use futures_util::TryFutureExt;
use tokio::task::JoinHandle;
use tracing::{error, info};

use self::{commands::Commands, config::Config, error::P2PError};

mod commands;
pub mod config;
pub mod error;
pub mod eth;

pub async fn handshake(config: Config) -> Result<(), eyre::ErrReport> {
    let tasks: Vec<JoinHandle<Result<IpAddr, P2PError>>> = match config.commands {
        Commands::Eth { nodes_addrs } => nodes_addrs
            .into_iter()
            .map(|node| {
                tokio::spawn(
                    eth::handshake(eth::Config {
                        timeout: config.timeout,
                        peer: node.to_owned(),
                    })
                    .map_err(move |err| {
                        P2PError::P2PHandshakeError(error::P2PHandshake::new(
                            err,
                            node.address.clone().to_string(),
                        ))
                    }),
                )
            })
            .collect(),
    };

    for task in tasks {
        match task.await? {
            Ok(addr) => info!("[successful] [{:?}] ", addr),
            Err(err) => error!("{}", err),
        }
    }
    Ok(())
}
