use bitcoin::{
    consensus::{deserialize_partial, serialize},
    p2p::{
        message::{NetworkMessage, RawNetworkMessage},
        message_network::VersionMessage,
        Address, ServiceFlags,
    },
    Network,
};
use bytes::{Buf, BytesMut};
use std::{
    fmt::Debug,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio_util::codec::{Decoder, Encoder};
use tracing::{instrument, trace};

/// Tokio codec for RawNetworkMessage
#[derive(Debug)]
pub(crate) struct RawNetworkMessageCodec {
    node_address: SocketAddr,
    user_agent: String,
}

/// Message types that can be sent over the stream
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetworkMessageType {
    Version,
    Verack,
}

impl RawNetworkMessageCodec {
    /// Create a new client codec to encode/decode messages for initiating a connection
    pub(crate) fn new_client(
        node_address: SocketAddr,
        user_agent: String,
    ) -> Result<Self, io::Error> {
        Ok(Self {
            node_address,
            user_agent,
        })
    }

    fn version_message(&self) -> RawNetworkMessage {
        trace!("creating version message ...");
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let sender = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);

        let btc_version = VersionMessage::new(
            ServiceFlags::NONE,
            now,
            Address::new(&self.node_address, ServiceFlags::NONE),
            Address::new(&sender, ServiceFlags::NONE),
            now as u64,
            self.user_agent.clone(),
            0,
        );

        RawNetworkMessage::new(
            Network::Bitcoin.magic(),
            NetworkMessage::Version(btc_version),
        )
    }

    pub fn verack_message(&self) -> RawNetworkMessage {
        trace!("creating verack message ...");
        RawNetworkMessage::new(Network::Bitcoin.magic(), NetworkMessage::Verack)
    }
}

impl Decoder for RawNetworkMessageCodec {
    type Item = RawNetworkMessage;
    type Error = io::Error;

    #[instrument(level = "trace", skip_all, fields(peer=&*format!("{:?}", self.node_address)))]
    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Ok((message, count)) = deserialize_partial::<RawNetworkMessage>(buf) {
            trace!("decoding message ...");

            buf.advance(count);
            return Ok(Some(message));
        }
        Ok(None)
    }
}

impl Encoder<NetworkMessageType> for RawNetworkMessageCodec {
    type Error = io::Error;

    #[instrument(level = "trace", skip_all, fields(peer=&*format!("{:?}", self.node_address)))]
    fn encode(&mut self, item: NetworkMessageType, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = match item {
            NetworkMessageType::Version => {
                trace!("encoding version message ...");
                self.version_message()
            }
            NetworkMessageType::Verack => {
                trace!("encoding verack message ...");
                self.verack_message()
            }
        };

        // Serialize the message and write it to the buffer
        let data = serialize(&msg);
        buf.extend_from_slice(&data);
        Ok(())
    }
}
