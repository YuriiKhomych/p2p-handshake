use bitcoin::p2p::message::NetworkMessage;
use futures::SinkExt;
use std::{fmt::Debug, io, net::SocketAddr};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Decoder;
use tracing::{instrument, trace};

use crate::p2p::btc::codec::{NetworkMessageType, RawNetworkMessageCodec};

/// Bitcoin Message handshake over TCP exchanging raw bytes
#[derive(Debug)]
pub struct MessageStream {
    node_address: SocketAddr,
    user_agent: String,
}

impl MessageStream {
    pub fn new(node_address: SocketAddr, user_agent: String) -> Self {
        Self {
            node_address,
            user_agent,
        }
    }

    /// Perform an initial handshake with a peer
    #[instrument(skip_all, fields(peer=&*format!("{:?}", self.node_address)))]
    pub async fn handshake(&self, stream: TcpStream) -> Result<(), io::Error> {
        let codec_client =
            RawNetworkMessageCodec::new_client(self.node_address, self.user_agent.clone())
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "invalid handshake"))?;

        let mut transport = codec_client.framed(stream);

        trace!("sending version message ...");
        transport.send(NetworkMessageType::Version).await?;

        while let Some(msg) = transport.try_next().await? {
            match msg.payload() {
                NetworkMessage::Verack => {
                    trace!("received verack message ...");
                    // Verack received, handshake is complete
                    break;
                }
                NetworkMessage::Version(_) => {
                    trace!("received version message");
                    trace!("sending verack ...");
                    // Received another Version message, send a Verack in response
                    transport.send(NetworkMessageType::Verack).await?;
                }
                _ => {
                    trace!("received unexpected for handshake other message");
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr};

    use super::*;
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_successful_handshake() {
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
        let listener = TcpListener::bind(addr).await.unwrap();
        let addr = listener.local_addr().unwrap(); // let node_address = "127.0.0.1:0".to_string();

        let handle = tokio::spawn(async move {
            let (incoming, _) = listener.accept().await.unwrap();
            let res = MessageStream::new(addr, "/Satoshi:25.0.0/".to_string())
                .handshake(incoming)
                .await;

            // Verify that the handshake was successful and the stream was created
            assert!(res.is_ok());
        });

        let outgoing = TcpStream::connect(addr).await.unwrap();
        let res = MessageStream::new(addr, "/Satoshi:25.0.0/".to_string())
            .handshake(outgoing)
            .await;

        // Verify that the handshake was successful and the stream was created
        assert!(res.is_ok());

        // make sure the server receives the message and asserts before ending the test
        handle.await.unwrap();
    }
}
