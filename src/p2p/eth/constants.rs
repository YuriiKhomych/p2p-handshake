/// [`MAX_PAYLOAD_SIZE`] is the maximum size of an uncompressed message payload.
/// This is defined in [EIP-706](https://eips.ethereum.org/EIPS/eip-706).
pub(crate) const MAX_PAYLOAD_SIZE: usize = 16 * 1024 * 1024;

/// [`HANDSHAKE_TIMEOUT`] determines the amount of time to wait before determining that a `p2p`
/// handshake has timed out.
pub const HANDSHAKE_TIMEOUT: u64 = 500;
