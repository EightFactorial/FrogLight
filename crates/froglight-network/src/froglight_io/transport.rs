use core::net::SocketAddr;
use std::io::Cursor;

use async_trait::async_trait;
use froglight_io::prelude::{FrogVarRead, FrogVarWrite};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

#[cfg(feature = "crypto")]
use crate::connection::ConnectionCrypto;
use crate::connection::{RawConnection, state::ConnectionError};

/// The default [`RawConnection`] implementation.
///
/// Wraps any [`AsyncReadExt`] and [`AsyncWriteExt`] stream.
pub struct IoTransport<S> {
    stream: S,
    peer: SocketAddr,
    compression: Option<i32>,
    scratch: Vec<u8>,

    #[cfg(feature = "crypto")]
    crypto: Option<ConnectionCrypto>,
}

impl<S> IoTransport<S> {
    /// Creates a new [`IoTransport`] instance.
    #[must_use]
    pub const fn wrap(stream: S, peer: SocketAddr) -> Self {
        #[cfg(feature = "crypto")]
        {
            Self { stream, peer, compression: None, scratch: Vec::new(), crypto: None }
        }

        #[cfg(not(feature = "crypto"))]
        {
            Self { stream, peer, compression: None, scratch: Vec::new() }
        }
    }
}

#[async_trait]
impl<S: AsyncReadExt + AsyncWriteExt + Send + Sync + Unpin + 'static> RawConnection
    for IoTransport<S>
{
    #[inline]
    async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> { Ok(self.peer) }

    #[inline]
    async fn get_compression(&self) -> Option<i32> { self.compression }

    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    #[inline]
    #[cfg(feature = "crypto")]
    async fn get_crypto(&self) -> Option<&ConnectionCrypto> { self.crypto.as_ref() }

    #[inline]
    #[cfg(feature = "crypto")]
    async fn get_crypto_mut(&mut self) -> Option<&mut ConnectionCrypto> { self.crypto.as_mut() }

    // TODO: Decompression
    async fn read_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        // Make sure the buffer is large enough to hold the packet length
        if buf.len() < 5 {
            buf.resize(5, 0u8);
        }

        // Read the length bytes of the packet
        let (len_buf, _) = buf.split_first_chunk_mut::<5>().unwrap();
        self.read_raw(len_buf.as_mut_slice()).await?;

        // Read the length of the packet and count the number of bytes
        let mut len_cursor = Cursor::new(len_buf);
        let len = <u32 as FrogVarRead>::frog_var_read(&mut len_cursor);
        let len = len.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;
        #[expect(clippy::cast_possible_truncation)]
        let len_size = len_cursor.position() as usize;

        // Resize the buffer to hold the packet data
        buf.resize(len + len_size, 0u8);
        // Create a slice positioned to read the rest of the packet
        let pbuf = &mut buf[5usize..];
        self.read_raw(pbuf).await?;
        // Create a cursor to read packet bytes from the buffer
        let mut cursor = Cursor::new(&buf[len_size..]);

        // Check whether the packet is compressed
        if self.compression.is_some_and(|c| c >= 0) {
            let size = <u32 as FrogVarRead>::frog_var_read(&mut cursor);
            let size = size.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;

            if size != 0 {
                todo!("Packet Decompression")
            }
        }

        // Move the packet to the beginning of the buffer and set the buffer size
        #[expect(clippy::cast_possible_truncation)]
        let position = cursor.position() as usize;
        buf.copy_within(len_size + position.., 0);
        buf.truncate(len);

        // Log the ID and length of the packet
        #[cfg(feature = "trace")]
        if let Some(id) = buf.first() {
            tracing::trace!(target: "froglight_network::froglight_io", "Reading Packet ID {id} ({})", buf.len());
        }

        Ok(())
    }

    // TODO: Compression
    async fn write_packet(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        // Log the ID and length of the packet
        #[cfg(feature = "trace")]
        if let Some(id) = buf.first() {
            tracing::trace!(target: "froglight_network::froglight_io", "Writing Packet ID {id} ({})", buf.len());
        }

        // Get the packet length and prefix length
        let packet_len = buf.len();
        let prefix_len = packet_len + packet_len.frog_var_len();

        if self.compression.is_some_and(|c| c <= prefix_len.try_into().unwrap_or_default()) {
            todo!("Packet Compression")
        } else {
            let mut prefix_buf = Vec::<u8>::with_capacity(8);

            if self.compression.is_some() {
                // Write the packet length and a `0` byte
                let result = FrogVarWrite::frog_var_write(&(packet_len + 1), &mut prefix_buf);
                result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
                prefix_buf.push(0);
            } else {
                // Write the packet length
                let result = FrogVarWrite::frog_var_write(&packet_len, &mut prefix_buf);
                result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
            }

            self.write_raw(&prefix_buf).await?;
            self.write_raw(buf).await
        }
    }

    // TODO: Test decryption
    async fn read_raw(&mut self, buf: &mut [u8]) -> Result<(), ConnectionError> {
        let result = self.stream.read_exact(buf).await;
        result.map_err(|err| ConnectionError::ReadRawConnection(Box::new(err)))?;

        #[cfg(feature = "crypto")]
        if let Some(crypto) = &mut self.crypto {
            crypto.decrypt_inplace(buf);
        }

        Ok(())
    }

    // TODO: Test encryption
    #[allow(unused_mut)]
    async fn write_raw(&mut self, mut buf: &[u8]) -> Result<(), ConnectionError> {
        #[cfg(feature = "crypto")]
        if let Some(crypto) = &mut self.crypto {
            // Clear and resize the scratch buffer to fit the packet
            self.scratch.clear();
            self.scratch.resize(buf.len(), 0u8);
            // Encrypt the packet using the crypto
            crypto.encrypt_into(buf, &mut self.scratch);
            // Use the scratch buffer as the input buffer
            buf = self.scratch.as_slice();
        }

        let result = self.stream.write_all(buf).await;
        result.map_err(|err| ConnectionError::WriteRawConnection(Box::new(err)))
    }
}
