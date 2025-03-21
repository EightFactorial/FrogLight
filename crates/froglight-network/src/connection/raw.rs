//! A raw connection to a client or server.
#![allow(clippy::manual_async_fn)]

use std::{
    future::Future,
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicI32, Ordering},
    },
};

use async_net::TcpStream;
use froglight_common::version::Version;
use froglight_io::{
    prelude::*,
    version::{FrogReadVersion, FrogWriteVersion},
};
use smol_str::SmolStr;

use super::io::{read_type, read_type_version, write_type, write_type_version};

/// A low-level connection to a server.
///
/// Can be used to read and write data directly.
#[derive(Debug)]
pub struct RawConnection {
    pub(super) address: SmolStr,
    pub(super) stream: TcpStream,
    pub(super) compression: Arc<AtomicI32>,
}

impl RawConnection {
    /// Get the address of the connection.
    #[inline]
    #[must_use]
    pub fn address(&self) -> &str { &self.address }

    /// Get the remote address the stream is connected to.
    #[inline]
    #[expect(clippy::missing_errors_doc)]
    pub fn peer_addr(&self) -> Result<SocketAddr, std::io::Error> { self.stream.peer_addr() }

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

    /// Get the underlying stream of the connection.
    #[inline]
    #[must_use]
    pub fn as_stream(&mut self) -> &mut TcpStream { &mut self.stream }

    /// Get the underlying [`TcpStream`] of the connection.
    #[inline]
    #[must_use]
    pub fn into_stream(self) -> TcpStream { self.stream }

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

    /// Read a value from the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being read is the same
    /// as the type being written!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn read_version<'a, T: FrogReadVersion<V> + Send + Sync + 'a, V: Version>(
        &'a mut self,
    ) -> impl Future<Output = Result<T, ReadError>> + Send + Sync + 'a {
        read_type_version(self)
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

    /// Write a value to the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being written is the same
    /// as the type being read!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn write_version<'a, T: FrogWriteVersion<V> + Send + Sync, V: Version>(
        &'a mut self,
        val: &'a T,
    ) -> impl Future<Output = Result<(), WriteError>> + Send + Sync + 'a {
        write_type_version(self, val)
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
    /// into a [`RawConnection`].
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

// -------------------------------------------------------------------------------------------------

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

    /// Get the remote address the stream is connected to.
    #[inline]
    #[expect(clippy::missing_errors_doc)]
    pub fn peer_addr(&self) -> Result<SocketAddr, std::io::Error> { self.0.peer_addr() }

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

    /// Read a value from the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being read is the same
    /// as the type being written!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn read_version<'a, T: FrogReadVersion<V> + Send + Sync + 'a, V: Version>(
        &'a mut self,
    ) -> impl Future<Output = Result<T, ReadError>> + Send + Sync + 'a {
        read_type_version(&mut self.0)
    }
}

// -------------------------------------------------------------------------------------------------

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

    /// Get the remote address the stream is connected to.
    #[inline]
    #[expect(clippy::missing_errors_doc)]
    pub fn peer_addr(&self) -> Result<SocketAddr, std::io::Error> { self.0.peer_addr() }

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

    /// Write a value to the connection.
    ///
    /// # Warning
    /// There are is no guarantee that type being written is the same
    /// as the type being read!
    ///
    /// This is a low-level API.
    #[inline]
    pub fn write_version<'a, T: FrogWriteVersion<V> + Send + Sync, V: Version>(
        &'a mut self,
        val: &'a T,
    ) -> impl Future<Output = Result<(), WriteError>> + Send + Sync + 'a {
        write_type_version(&mut self.0, val)
    }
}
