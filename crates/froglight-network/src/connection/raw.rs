use std::{
    future::Future,
    io::Cursor,
    net::SocketAddr,
    sync::{
        atomic::{AtomicI32, Ordering},
        Arc,
    },
};

use async_net::TcpStream;
use froglight_io::{
    standard::{FrogRead, FrogWrite, ReadError, WriteError},
    variable::{FrogVarRead, FrogVarWrite},
};
use futures_lite::{AsyncReadExt, AsyncWriteExt};
use smol_str::SmolStr;

/// A low-level connection to a server.
///
/// Can be used to read and write data directly.
#[derive(Debug)]
pub struct RawConnection {
    address: SmolStr,
    stream: TcpStream,
    compression: Arc<AtomicI32>,
}

impl RawConnection {
    /// Get the address of the connection.
    #[inline]
    #[must_use]
    pub fn address(&self) -> &str { &self.address }

    /// Get the compression level of the connection.
    #[must_use]
    pub fn compression(&self) -> Option<i32> {
        match self.compression.load(Ordering::Relaxed) {
            x if x == i32::MIN => None,
            x => Some(x),
        }
    }

    /// Set the compression level of the connection.
    pub fn set_compression(&self, level: Option<i32>) {
        self.compression.store(level.unwrap_or(i32::MIN), Ordering::Relaxed);
    }

    /// Create a [`RawConnection`] by connecting to the given address.
    ///
    /// The address is resolved into a [`SocketAddr`] using the system resolver.
    ///
    /// # Errors
    /// Returns an error if the connection could not be established,
    /// or if the stream could not set `nodelay` to `true`.
    pub async fn connect(address: &(impl AsRef<str> + ?Sized)) -> Result<Self, std::io::Error> {
        TcpStream::connect(address.as_ref())
            .await
            .and_then(|stream| Self::from_stream(address, stream))
    }

    /// Create a [`RawConnection`] by connecting to the given address and port.
    ///
    /// The address is cosmetic, only the socket is used for the connection.
    ///
    /// # Errors
    /// Returns an error if the connection could not be established,
    /// or if the stream could not set `nodelay` to `true`.
    pub async fn connect_to(
        address: &(impl AsRef<str> + ?Sized),
        socket: SocketAddr,
    ) -> Result<Self, std::io::Error> {
        TcpStream::connect(socket).await.and_then(|stream| Self::from_stream(address, stream))
    }

    /// Create a [`RawConnection`] from an address and existing stream.
    ///
    /// The address is cosmetic, only the stream is used for the connection.
    ///
    /// # Errors
    /// Returns an error if the stream could not set `nodelay` to `true`.
    pub fn from_stream(
        address: &(impl AsRef<str> + ?Sized),
        stream: TcpStream,
    ) -> Result<Self, std::io::Error> {
        stream.set_nodelay(true)?;
        Ok(Self {
            address: SmolStr::new(address),
            stream,
            compression: Arc::new(AtomicI32::new(i32::MIN)),
        })
    }

    /// Read a value from the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being read is the same
    /// as the type being written!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn read<'a, T: FrogRead + Send + Sync + 'a>(
        &'a mut self,
    ) -> impl Future<Output = Result<T, ReadError>> + Send + Sync + 'a {
        read_type(self)
    }

    /// Write a value to the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being written is the same
    /// as the type being read!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn write<'a, T: FrogWrite + Send + Sync>(
        &'a mut self,
        val: &'a T,
    ) -> impl Future<Output = Result<(), WriteError>> + Send + Sync + 'a {
        write_type(self, val)
    }

    /// Split the [`RawConnection`] into a
    /// [`RawReadConnection`] and a [`RawWriteConnection`] pair.
    ///
    /// These can be recombined using [`RawConnection::from_split`].
    #[must_use]
    pub fn into_split(self) -> (RawReadConnection, RawWriteConnection) {
        (
            RawReadConnection(Self {
                address: self.address.clone(),
                stream: self.stream.clone(),
                compression: self.compression.clone(),
            }),
            RawWriteConnection(Self {
                address: self.address,
                stream: self.stream,
                compression: self.compression,
            }),
        )
    }

    /// Recombine a [`RawReadConnection`] and a [`RawWriteConnection`]
    /// into a single [`RawConnection`].
    ///
    /// Both parts must be from the same connection, otherwise this will panic.
    ///
    /// # Panics
    /// Panics if the two connection halves are from different connections.
    #[must_use]
    pub fn from_split(read: RawReadConnection, write: RawWriteConnection) -> Self {
        assert!(
            Arc::ptr_eq(&read.0.compression, &write.0.compression),
            "Connection halves are from different connections"
        );
        Self { address: write.0.address, stream: write.0.stream, compression: write.0.compression }
    }
}

/// A low-level read-only connection to a server.
///
/// Can be used to read data from the connection.
#[derive(Debug)]
pub struct RawReadConnection(RawConnection);

impl RawReadConnection {
    /// Get the address of the connection.
    #[inline]
    #[must_use]
    pub fn address(&self) -> &str { self.0.address() }

    /// Get the compression level of the connection.
    #[inline]
    #[must_use]
    pub fn compression(&self) -> Option<i32> { self.0.compression() }

    /// Set the compression level of the connection.
    #[inline]
    pub fn set_compression(&self, level: Option<i32>) { self.0.set_compression(level) }

    /// Read a value from the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being read is the same
    /// as the type being written!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn read<'a, T: FrogRead + Send + Sync + 'a>(
        &'a mut self,
    ) -> impl Future<Output = Result<T, ReadError>> + Send + Sync + 'a {
        read_type(&mut self.0)
    }
}

/// A low-level write-only connection to a server.
///
/// Can be used to write data to the connection.
#[derive(Debug)]
pub struct RawWriteConnection(RawConnection);

impl RawWriteConnection {
    /// Get the address of the connection.
    #[inline]
    #[must_use]
    pub fn address(&self) -> &str { self.0.address() }

    /// Get the compression level of the connection.
    #[inline]
    #[must_use]
    pub fn compression(&self) -> Option<i32> { self.0.compression() }

    /// Set the compression level of the connection.
    #[inline]
    pub fn set_compression(&self, level: Option<i32>) { self.0.set_compression(level) }

    /// Write a value to the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being written is the same
    /// as the type being read!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn write<'a, T: FrogWrite + Send + Sync>(
        &'a mut self,
        val: &'a T,
    ) -> impl Future<Output = Result<(), WriteError>> + Send + Sync + 'a {
        write_type(&mut self.0, val)
    }
}

/// TODO: Decryption
async fn read_type<T: FrogRead>(raw: &mut RawConnection) -> Result<T, ReadError> {
    let mut len_buf = [0u8; 5];
    raw.stream.peek(&mut len_buf).await?;
    let mut len_cursor = Cursor::new(len_buf.as_slice());

    #[expect(clippy::cast_possible_truncation)]
    let mut packet_buf: Vec<u8> =
        Vec::with_capacity(usize::frog_var_read(&mut len_cursor)? + len_cursor.position() as usize);
    raw.stream.read_exact(packet_buf.as_mut_slice()).await?;

    #[expect(clippy::cast_possible_truncation)]
    let mut packet_cursor = Cursor::new(&packet_buf[len_cursor.position() as usize..]);
    if raw.compression().is_some_and(|c| c >= 0) && 0 != u32::frog_var_read(&mut packet_cursor)? {
        unimplemented!("Packet Decompression")
    } else {
        T::frog_read(&mut packet_cursor)
    }
}

/// TODO: Encryption
async fn write_type<T: FrogWrite + Send + Sync>(
    raw: &mut RawConnection,
    val: &T,
) -> Result<(), WriteError> {
    let packet_len = val.frog_len();
    let prefixed_len = packet_len + packet_len.frog_var_len();

    let compression = raw.compression();
    if compression.is_some_and(|c| i32::try_from(prefixed_len).unwrap_or_default() >= c) {
        unimplemented!("Packet Compression");
    } else {
        let mut buf = Vec::with_capacity(prefixed_len);

        if compression.is_some() {
            (packet_len + 1).frog_var_write(&mut buf)?;
            u8::frog_write(&0, &mut buf)?;
        } else {
            packet_len.frog_var_write(&mut buf)?;
        }

        val.frog_write(&mut buf)?;
        raw.stream.write_all(&buf).await.map_err(WriteError::Io)
    }
}
