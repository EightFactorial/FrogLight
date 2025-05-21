//! TODO

use core::net::SocketAddr;
use std::io::Cursor;

#[cfg(feature = "crypto")]
use aes::{
    Aes128,
    cipher::{BlockDecryptMut, BlockEncryptMut, inout::InOutBuf},
};
use async_net::TcpStream;
use async_trait::async_trait;
#[cfg(feature = "crypto")]
use cfb8::{Decryptor, Encryptor};
use froglight_common::version::Version;
use froglight_io::{
    prelude::{FrogVarRead, FrogVarWrite},
    version::{FrogReadVersion, FrogWriteVersion},
};
use froglight_packet::state::{Handshake, ValidState};
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use crate::{
    connection::{RawConnection, raw::RawPacketVersion, state::ConnectionError},
    prelude::ClientConnection,
};

impl<V: ValidState<Handshake>> ClientConnection<V, Handshake> {
    /// Connect to a server at the given address,
    /// resolved using the provided
    /// [`FroglightResolver`](crate::prelude::FroglightResolver).
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    #[cfg(feature = "resolver")]
    pub async fn connect(
        addr: &str,
        resolver: &crate::prelude::FroglightResolver,
    ) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect(addr, resolver).await.map(Self::from_raw)
    }

    /// Connect to a server at the given address,
    /// resolved using the default system resolver.
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    pub async fn connect_system(addr: &str) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect_system(addr).await.map(Self::from_raw)
    }

    /// Connect to a server at the given [`SocketAddr`].
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    pub async fn connect_to(socket: SocketAddr) -> Result<Self, std::io::Error> {
        IoTransport::<TcpStream>::connect_to(socket).await.map(Self::from_raw)
    }
}

// -------------------------------------------------------------------------------------------------

/// A marker struct for [`RawPacketVersion`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IoPacket;

impl<V: Version, T: FrogReadVersion<V> + FrogWriteVersion<V> + Send + Sync + 'static>
    RawPacketVersion<V, IoPacket> for T
{
    async fn read_packet<'a, C: RawConnection + ?Sized>(
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> Result<Self, ConnectionError> {
        buf.clear();

        conn.read_packet(buf).await?;
        let mut cursor = Cursor::new(buf.as_mut_slice());
        let result = T::frog_read(&mut cursor);
        result.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))
    }

    async fn write_packet<'a, C: RawConnection + ?Sized>(
        &'a self,
        conn: &'a mut C,
        buf: &'a mut Vec<u8>,
    ) -> Result<(), ConnectionError> {
        buf.clear();

        let result = self.frog_write(buf);
        result.map_err(|err| ConnectionError::WriteRawPacket(Box::new(err)))?;
        conn.write_packet(buf).await
    }
}

// -------------------------------------------------------------------------------------------------

/// The default [`RawConnection`] implementation.
pub struct IoTransport<S> {
    stream: S,
    peer: SocketAddr,
    compression: Option<i32>,
    #[allow(dead_code)]
    scratch: Vec<u8>,

    #[cfg(feature = "crypto")]
    cipher: Option<IoCipher>,
}

/// The cipher used for encrypting and decrypting packets.
#[cfg(feature = "crypto")]
pub struct IoCipher {
    /// The cipher is used to encrypt packets.
    pub encryptor: Encryptor<Aes128>,
    /// The decryptor is used to decrypt packets.
    pub decryptor: Decryptor<Aes128>,
}

impl<S> IoTransport<S> {
    /// Creates a new [`IoTransport`] instance.
    #[must_use]
    pub const fn wrap(stream: S, peer: SocketAddr) -> Self {
        #[cfg(feature = "crypto")]
        {
            Self { stream, peer, compression: None, scratch: Vec::new(), cipher: None }
        }

        #[cfg(not(feature = "crypto"))]
        {
            Self { stream, peer, compression: None, scratch: Vec::new() }
        }
    }

    /// Get a mutable reference to the inner [`IoCipher`].
    #[inline]
    #[must_use]
    #[cfg(feature = "crypto")]
    pub fn cipher_mut(&mut self) -> &mut Option<IoCipher> { &mut self.cipher }
}

#[async_trait]
impl<S: AsyncReadExt + AsyncWriteExt + Send + Sync + Unpin + 'static> RawConnection
    for IoTransport<S>
{
    #[inline]
    async fn peer_addr(&self) -> Result<SocketAddr, ConnectionError> { Ok(self.peer) }

    #[inline]
    async fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    #[inline]
    async fn get_compression(&self) -> Option<i32> { self.compression }

    // TODO: Decompression
    async fn read_packet(&mut self, buf: &mut Vec<u8>) -> Result<(), ConnectionError> {
        // Read the length bytes of the packet
        let mut len_buf = Cursor::new([0u8; 5]);
        self.read_raw(len_buf.get_mut().as_mut_slice()).await?;

        // Read the length of the packet and count the number of bytes
        let len = <u32 as FrogVarRead>::frog_var_read(&mut len_buf);
        let len = len.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;
        #[expect(clippy::cast_possible_truncation)]
        let len_size = len_buf.position() as usize;

        // Make sure the buffer can hold the packet
        buf.resize(buf.len().max(len + len_size), 0u8);
        // Copy the non-length bytes into the buffer
        buf[..5usize - len_size].copy_from_slice(&len_buf.get_ref()[len_size..5usize]);
        // Read the rest of the packet into the buffer
        let pbuf = &mut buf[len_size + 1..len];
        self.read_raw(pbuf).await?;

        // Create a cursor to read from
        let mut cursor = Cursor::new(pbuf);

        // Check whether the packet is compressed
        if self.compression.is_some_and(|c| c >= 0) {
            let size = <u32 as FrogVarRead>::frog_var_read(&mut cursor);
            let size = size.map_err(|err| ConnectionError::ReadRawPacket(Box::new(err)))? as usize;

            if size != 0 {
                todo!("Packet Decompression")
            }
        }

        // Move the packet to the beginning of the buffer and set the buffer size.
        #[expect(clippy::cast_possible_truncation)]
        let position = cursor.position() as usize;
        buf.copy_within(position.., 0);
        buf.truncate(buf.len() - position);

        // Log the ID and length of the packet
        #[cfg(feature = "trace")]
        if let Some(id) = buf.first() {
            tracing::trace!("Reading Packet: {id} ({})", buf.len());
        }

        Ok(())
    }

    // TODO: Compression
    async fn write_packet(&mut self, buf: &[u8]) -> Result<(), ConnectionError> {
        // Log the ID and length of the packet
        #[cfg(feature = "trace")]
        if let Some(id) = buf.first() {
            tracing::trace!("Writing Packet: {id} ({})", buf.len());
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
        if let Some(cipher) = &mut self.cipher {
            // Decrypt the packet using the cipher
            let inout = InOutBuf::from(buf);
            let (inout, tail) = inout.into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            cipher.decryptor.decrypt_blocks_inout_mut(inout);
        }

        Ok(())
    }

    // TODO: Test encryption
    #[allow(unused_mut)]
    async fn write_raw(&mut self, mut buf: &[u8]) -> Result<(), ConnectionError> {
        #[cfg(feature = "crypto")]
        if let Some(cipher) = &mut self.cipher {
            // Clear and resize the scratch buffer to fit the packet
            self.scratch.clear();
            self.scratch.resize(buf.len(), 0u8);

            // Encrypt the packet using the cipher
            let inout = InOutBuf::new(buf, self.scratch.as_mut_slice()).unwrap();
            let (inout, tail) = inout.into_chunks();
            debug_assert!(tail.is_empty(), "InOutBuf tail should be empty!");
            cipher.encryptor.encrypt_blocks_inout_mut(inout);

            // Use the scratch buffer as the input buffer
            buf = self.scratch.as_slice();
        }

        let result = self.stream.write_all(buf).await;
        result.map_err(|err| ConnectionError::WriteRawConnection(Box::new(err)))
    }
}

// -------------------------------------------------------------------------------------------------

impl IoTransport<TcpStream> {
    /// Create an [`IoTransport`] from an address resolved
    /// using the provided
    /// [`FroglightResolver`](crate::prelude::FroglightResolver).
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    #[inline]
    #[cfg(feature = "resolver")]
    pub async fn connect(
        addr: &str,
        resolver: &crate::prelude::FroglightResolver,
    ) -> Result<Self, std::io::Error> {
        Self::connect_to(resolver.lookup_minecraft(addr).await?).await
    }

    /// Create an [`IoTransport`] from an address resolved
    /// using the system's default resolver.
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    pub async fn connect_system(addr: &str) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr).await?;
        let socket = stream.peer_addr()?;
        stream.set_nodelay(true)?;
        Ok(Self::wrap(stream, socket))
    }

    /// Create an [`IoTransport`] from a [`SocketAddr`].
    ///
    /// # Errors
    /// Returns an error if the connection cannot be established,
    /// or if `TCP_NODELAY` cannot be set.
    pub async fn connect_to(socket: SocketAddr) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(socket).await?;
        stream.set_nodelay(true)?;
        Ok(Self::wrap(stream, socket))
    }
}
