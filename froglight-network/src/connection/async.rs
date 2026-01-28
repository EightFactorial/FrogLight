//! TODO

use std::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy_tasks::{IoTaskPool, Task};
use froglight_packet::version::{Clientbound, PacketVersion, Serverbound, VersionPacket};
#[cfg(feature = "futures-lite")]
use futures_lite::{AsyncRead as FAsyncRead, AsyncWrite as FAsyncWrite};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead as TAsyncRead, AsyncWrite as TAsyncWrite};

use crate::connection::{Encrypted, channel::Channel as InnerChannel};

/// A [`Version`]'ed connection that uses a specific [`Runtime`].
pub struct AsyncConnection<R: Runtime<C>, C: Send, V: PacketVersion> {
    connection: Encrypted<R, C>,
    channel: Channel<V>,
    _phantom: PhantomData<(R, V)>,
}

type Channel<V> = InnerChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>;

impl<R: Runtime<C>, C: Send, V: PacketVersion> AsyncConnection<R, C, V> {
    /// Create a new [`AsyncConnection`].
    #[inline]
    #[must_use]
    pub fn new(connection: C, channel: Channel<V>) -> Self {
        Self::new_encrypted(Encrypted::new(connection), channel)
    }

    /// Create a new [`AsyncConnection`] from an already encrypted connection.
    #[inline]
    #[must_use]
    pub const fn new_encrypted(connection: Encrypted<R, C>, channel: Channel<V>) -> Self {
        Self { connection, channel, _phantom: PhantomData }
    }

    /// Change the [`Runtime`] of this [`AsyncConnection`].
    #[inline]
    #[must_use]
    pub fn with_runtime<R2: Runtime<C>>(self) -> AsyncConnection<R2, C, V> {
        AsyncConnection {
            connection: self.connection.with_runtime(),
            channel: self.channel,
            _phantom: PhantomData,
        }
    }

    /// Get a reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn connection(&self) -> &Encrypted<R, C> { &self.connection }

    /// Get a mutable reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn connection_mut(&mut self) -> &mut Encrypted<R, C> { &mut self.connection }

    /// Get a reference to the connection's channel.
    #[inline]
    #[must_use]
    pub const fn channel(&self) -> &Channel<V> { &self.channel }

    /// Separate the [`AsyncConnection`] into its parts.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (Encrypted<R, C>, Channel<V>) { (self.connection, self.channel) }
}

#[cfg(feature = "futures-lite")]
impl<C: Send, V: PacketVersion> AsyncConnection<FuturesLite, C, V>
where
    FuturesLite: Runtime<C>,
{
    /// Create a new [`AsyncConnection`] using the [`futures_lite`] runtime.
    #[inline]
    #[must_use]
    pub fn new_async(connection: C, channel: Channel<V>) -> Self { Self::new(connection, channel) }
}

#[cfg(feature = "tokio")]
impl<C: Send, V: PacketVersion> AsyncConnection<Tokio, C, V>
where
    Tokio: Runtime<C>,
{
    /// Create a new [`AsyncConnection`] using the [`tokio`] runtime.
    #[inline]
    #[must_use]
    pub fn new_tokio(connection: C, channel: Channel<V>) -> Self { Self::new(connection, channel) }
}

// -------------------------------------------------------------------------------------------------

/// A trait for runtime-specific read and write operations.
///
/// Also provides methods for splitting connections and spawning tasks.
pub trait Runtime<C>:
    RuntimeRead<C>
    + RuntimeWrite<C>
    + RuntimeRead<Self::Read>
    + RuntimeWrite<Self::Write>
    + Sized
    + Send
    + 'static
{
    /// The read half of the connection.
    type Read: Send + 'static;
    /// The write half of the connection.
    type Write: Send + 'static;

    /// Split the connection into a read and write half.
    fn into_split(conn: C) -> (Self::Read, Self::Write);

    /// Spawn a task on the [`IoTaskPool`].
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret>;
}

/// A trait for reading from a connection in a specific runtime.
pub trait RuntimeRead<C>: Sized + Send + 'static {
    /// Reads the exact number of bytes required to fill `buf`.
    fn read_exact<'a>(
        conn: &'a mut C,
        buf: &'a mut [u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'a;
}

/// A trait for writing to a connection in a specific runtime.
pub trait RuntimeWrite<C>: Sized + Send + 'static {
    /// Writes an entire buffer into the byte stream.
    fn write_all<'a>(
        conn: &'a mut C,
        buf: &'a [u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'a;
}

// ------------------------------------

/// Marker type for the [`futures_lite`] runtime.
#[cfg(feature = "futures-lite")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuturesLite;

#[cfg(feature = "futures-lite")]
impl<C: FAsyncRead + FAsyncWrite + Clone + Unpin + Send + 'static> Runtime<C> for FuturesLite {
    type Read = C;
    type Write = C;

    fn into_split(conn: C) -> (Self::Read, Self::Write) { (conn.clone(), conn) }

    #[inline]
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret> {
        IoTaskPool::get().spawn(future)
    }
}

#[cfg(feature = "futures-lite")]
impl<C: FAsyncRead + Unpin + Send> RuntimeRead<C> for FuturesLite {
    #[inline]
    fn read_exact<'a>(
        conn: &'a mut C,
        buf: &'a mut [u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'a {
        futures_lite::AsyncReadExt::read_exact(conn, buf)
    }
}

#[cfg(feature = "futures-lite")]
impl<C: FAsyncWrite + Unpin + Send> RuntimeWrite<C> for FuturesLite {
    #[inline]
    fn write_all<'a>(
        conn: &'a mut C,
        buf: &'a [u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'a {
        futures_lite::AsyncWriteExt::write_all(conn, buf)
    }
}

// ------------------------------------

/// Marker type for the [`tokio`] runtime.
#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tokio;

#[cfg(feature = "tokio")]
impl<C: TAsyncRead + TAsyncWrite + Clone + Send + Unpin + 'static> Runtime<C> for Tokio {
    type Read = C;
    type Write = C;

    fn into_split(conn: C) -> (Self::Read, Self::Write) { (conn.clone(), conn) }

    #[inline]
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret> {
        IoTaskPool::get().spawn(async_compat::Compat::new(future))
    }
}

#[cfg(feature = "tokio")]
impl<C: TAsyncRead + Send + Unpin> RuntimeRead<C> for Tokio {
    #[inline]
    #[allow(clippy::manual_async_fn, reason = "Control")]
    fn read_exact<'a>(
        conn: &'a mut C,
        buf: &'a mut [u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'a {
        async { tokio::io::AsyncReadExt::read_exact(conn, buf).await.map(|_| ()) }
    }
}

#[cfg(feature = "tokio")]
impl<C: TAsyncWrite + Send + Unpin> RuntimeWrite<C> for Tokio {
    #[inline]
    fn write_all<'a>(
        conn: &'a mut C,
        buf: &'a [u8],
    ) -> impl Future<Output = std::io::Result<()>> + Send + 'a {
        tokio::io::AsyncWriteExt::write_all(conn, buf)
    }
}

/// A wrapper around [`TcpStream`](tokio::net::TcpStream) where
/// [`Tokio`] implements [`Runtime`].
#[cfg(feature = "tokio")]
#[repr(transparent)]
pub struct TokioTcpStream(pub tokio::net::TcpStream);

#[cfg(feature = "tokio")]
impl Runtime<TokioTcpStream> for Tokio {
    type Read = tokio::net::tcp::OwnedReadHalf;
    type Write = tokio::net::tcp::OwnedWriteHalf;

    fn into_split(conn: TokioTcpStream) -> (Self::Read, Self::Write) { conn.0.into_split() }

    #[inline]
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret> {
        IoTaskPool::get().spawn(async_compat::Compat::new(future))
    }
}

#[cfg(feature = "tokio")]
impl TAsyncRead for TokioTcpStream {
    #[inline]
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        TAsyncRead::poll_read(std::pin::Pin::new(&mut self.get_mut().0), cx, buf)
    }
}

#[cfg(feature = "tokio")]
impl TAsyncWrite for TokioTcpStream {
    #[inline]
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        TAsyncWrite::poll_write(std::pin::Pin::new(&mut self.get_mut().0), cx, buf)
    }

    #[inline]
    fn poll_write_vectored(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<std::io::Result<usize>> {
        TAsyncWrite::poll_write_vectored(std::pin::Pin::new(&mut self.get_mut().0), cx, bufs)
    }

    #[inline]
    fn is_write_vectored(&self) -> bool { TAsyncWrite::is_write_vectored(&self.0) }

    #[inline]
    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        TAsyncWrite::poll_flush(std::pin::Pin::new(&mut self.get_mut().0), cx)
    }

    #[inline]
    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        TAsyncWrite::poll_shutdown(std::pin::Pin::new(&mut self.get_mut().0), cx)
    }
}

#[cfg(feature = "tokio")]
impl From<tokio::net::TcpStream> for TokioTcpStream {
    #[inline]
    fn from(stream: tokio::net::TcpStream) -> Self { Self(stream) }
}
#[cfg(feature = "tokio")]
impl From<TokioTcpStream> for tokio::net::TcpStream {
    #[inline]
    fn from(stream: TokioTcpStream) -> Self { stream.0 }
}

#[cfg(feature = "tokio")]
impl std::ops::Deref for TokioTcpStream {
    type Target = tokio::net::TcpStream;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
#[cfg(feature = "tokio")]
impl core::ops::DerefMut for TokioTcpStream {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
