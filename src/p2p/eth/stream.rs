use alloy_rlp::{Decodable, Encodable};
use futures::{Sink, SinkExt, StreamExt};
use pin_project::pin_project;
use reth_eth_wire::{
    errors::{P2PHandshakeError, P2PStreamError},
    DisconnectReason, HelloMessage, P2PMessage,
};
use reth_primitives::{
    bytes::{Bytes, BytesMut},
    hex,
};
use std::{io, time::Duration};
use tokio_stream::Stream;

use crate::p2p::eth::constants::MAX_PAYLOAD_SIZE;

/// The `P2PStream` is consumed the ecies stream and returns an ok result if
/// `Hello` handshake is completed.
#[pin_project]
#[derive(Debug)]
pub struct P2PStream<S> {
    #[pin]
    stream: S,
}

impl<S> P2PStream<S> {
    /// Create a new `P2PStream` from a type `S` which implements `Stream` and `Sink`.
    pub fn new(stream: S) -> Self {
        Self { stream }
    }
}

/// Implement `Stream` for `P2PStream` by delegating to the underlying stream.
impl<S> P2PStream<S>
where
    S: Stream<Item = io::Result<BytesMut>> + Sink<Bytes, Error = io::Error> + Unpin,
{
    /// Consumes the `P2PStream` and performs a handshake with the peer.
    pub async fn handshake(
        mut self,
        hello: HelloMessage,
        timeout: u64,
    ) -> Result<(), P2PStreamError> {
        tracing::trace!(?hello, "sending p2p hello to peer");

        // Send our hello message with the Sink
        let mut raw_hello_bytes = BytesMut::new();
        P2PMessage::Hello(hello.clone()).encode(&mut raw_hello_bytes);
        self.stream.send(raw_hello_bytes.into()).await?;

        // Receive the first message from the peer
        tracing::trace!("waiting for first message from peer");
        let first_message_bytes =
            tokio::time::timeout(Duration::from_millis(timeout), self.stream.next())
                .await
                .or(Err(P2PStreamError::HandshakeError(
                    P2PHandshakeError::Timeout,
                )))?
                .ok_or(P2PStreamError::HandshakeError(
                    P2PHandshakeError::NoResponse,
                ))??;

        // let's check the compressed length first, we will need to check again once confirming
        // that it contains snappy-compressed data (this will be the case for all non-p2p messages).
        if first_message_bytes.len() > MAX_PAYLOAD_SIZE {
            return Err(P2PStreamError::MessageTooBig {
                message_size: first_message_bytes.len(),
                max_size: MAX_PAYLOAD_SIZE,
            });
        }

        // The first message sent MUST be a hello OR disconnect message
        // to finalize the handshake.
        tracing::trace!(?first_message_bytes, "received first message from peer");
        match P2PMessage::decode(&mut &first_message_bytes[..]) {
            Ok(P2PMessage::Hello(hello)) => Ok(hello),
            Ok(P2PMessage::Disconnect(reason)) => {
                tracing::debug!("Disconnected by peer during handshake: {}", reason);
                Err(P2PStreamError::HandshakeError(
                    P2PHandshakeError::Disconnected(reason),
                ))
            }
            Err(err) => {
                tracing::debug!(?err, msg=%hex::encode(&first_message_bytes), "Failed to decode first message from peer");
                Err(P2PStreamError::HandshakeError(err.into()))
            }
            Ok(msg) => {
                tracing::debug!("expected hello message but received: {:?}", msg);
                Err(P2PStreamError::HandshakeError(
                    P2PHandshakeError::NonHelloMessageInHandshake,
                ))
            }
        }?;

        // Send disconnect message to avoid keeping the connection alive with peer
        tracing::trace!("sending disconnect message to peer");
        self.send_disconnect(DisconnectReason::ClientQuitting)
            .await?;

        Ok(())
    }
}

impl<S> P2PStream<S>
where
    S: Sink<Bytes, Error = io::Error> + Unpin,
{
    /// Send a disconnect message during the handshake.
    pub async fn send_disconnect(
        &mut self,
        reason: DisconnectReason,
    ) -> Result<(), P2PStreamError> {
        let mut buf = BytesMut::new();
        P2PMessage::Disconnect(reason).encode(&mut buf);
        tracing::trace!(
            %reason,
            "Sending disconnect message during the handshake",
        );
        self.stream
            .send(buf.freeze())
            .await
            .map_err(P2PStreamError::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::p2p::eth::utils::create_hello_msg;
    use reth_eth_wire::DisconnectReason;
    use secp256k1::SecretKey;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_util::codec::{Decoder, LengthDelimitedCodec};

    #[tokio::test]
    async fn test_handshake_passthrough() {
        // Create a p2p stream and server and confirm that the two go through the handshake process successfully
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = listener.local_addr().unwrap();

        let handle = tokio::spawn(async move {
            // Create a p2p incoming stream and send a hello message
            let (incoming, _) = listener.accept().await.unwrap();
            let stream = LengthDelimitedCodec::default().framed(incoming);

            let server_hello = create_hello_msg(SecretKey::new(&mut rand::thread_rng()));

            // Confirm that the handshake is successful
            let p2p_stream = P2PStream::new(stream);
            match p2p_stream.handshake(server_hello, 10).await {
                Ok(_) => (),
                Err(e) => panic!("unexpected err: {e}"),
            }
        });

        // Create a p2p outgoing stream and send a hello message
        let outgoing = TcpStream::connect(local_addr).await.unwrap();
        let sink = LengthDelimitedCodec::default().framed(outgoing);

        let client_hello = create_hello_msg(SecretKey::new(&mut rand::thread_rng()));

        // Confirm that the handshake is successful
        let p2p_stream = P2PStream::new(sink);
        match p2p_stream.handshake(client_hello, 10).await {
            Ok(_) => (),
            Err(e) => panic!("unexpected err: {e}"),
        }

        // Make sure the server receives the message successfully before ending the test
        handle.await.unwrap();
    }

    #[tokio::test]
    async fn test_handshake_timeout_err() {
        // Create a p2p stream and server and confirm that we get a timeout error
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = listener.local_addr().unwrap();

        // Create a p2p outgoing stream and send a hello message
        let outgoing = TcpStream::connect(local_addr).await.unwrap();
        let sink = LengthDelimitedCodec::default().framed(outgoing);

        let client_hello = create_hello_msg(SecretKey::new(&mut rand::thread_rng()));

        // Confirm that the handshake times out
        let p2p_stream = P2PStream::new(sink);
        match p2p_stream.handshake(client_hello, 10).await {
            Ok(_) => panic!("expected handshake to fail, instead got a success"),
            Err(P2PStreamError::HandshakeError(P2PHandshakeError::Timeout)) => (),
            Err(other_err) => panic!("expected timeout error, got {other_err:?}"),
        }
    }

    #[tokio::test]
    async fn test_handshake_disconnect() {
        // Create a p2p stream and server, then confirm that the two are authed
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = listener.local_addr().unwrap();

        let handle = tokio::spawn(async move {
            // Create a p2p incoming stream and send a disconnect message
            let (incoming, _) = listener.accept().await.unwrap();
            let stream = LengthDelimitedCodec::default().framed(incoming);

            // Send a disconnect message
            let mut p2p_stream = P2PStream::new(stream);
            p2p_stream
                .send_disconnect(DisconnectReason::UselessPeer)
                .await
                .unwrap();
        });

        let outgoing = TcpStream::connect(local_addr).await.unwrap();
        let sink = LengthDelimitedCodec::default().framed(outgoing);

        let client_hello = create_hello_msg(SecretKey::new(&mut rand::thread_rng()));

        // Confirm that the handshake fails
        let p2p_stream = P2PStream::new(sink);
        match p2p_stream.handshake(client_hello.clone(), 10).await {
            Ok(_) => panic!("expected handshake to fail, instead got a success"),
            Err(P2PStreamError::HandshakeError(P2PHandshakeError::Disconnected(reason))) => {
                assert_eq!(reason, DisconnectReason::UselessPeer);
            }
            Err(e) => panic!("unexpected err: {e}"),
        }

        // Make sure the server sends the disconnect message before ending the test
        handle.await.unwrap();
    }
}
