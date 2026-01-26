//! TODO

use core::marker::PhantomData;

#[cfg(feature = "bevy")]
use bevy_tasks::{IoTaskPool, Task};
use froglight_packet::version::{Clientbound, PacketVersion, Serverbound, VersionPacket};
#[cfg(feature = "futures-lite")]
use futures_lite::{AsyncRead as FAsyncRead, AsyncWrite as FAsyncWrite};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead as TAsyncRead, AsyncWrite as TAsyncWrite};

use crate::connection::{Encrypted, channel::Channel as InnerChannel};

/// A [`Version`]'ed connection that uses a specific [`Runtime`].
#[derive(Clone)]
pub struct AsyncConnection<R: Runtime<C>, C, V: PacketVersion> {
    connection: Encrypted<R, C>,
    channel: Channel<V>,
    _phantom: PhantomData<(R, V)>,
}

type Channel<V> = InnerChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>;

impl<R: Runtime<C>, C, V: PacketVersion> AsyncConnection<R, C, V> {
    /// Create a new [`AsyncConnection`].
    #[inline]
    #[must_use]
    pub const fn new(connection: C, channel: Channel<V>) -> Self {
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
impl<C, V: PacketVersion> AsyncConnection<FuturesLite, C, V>
where
    FuturesLite: Runtime<C>,
{
    /// Create a new [`AsyncConnection`] using the [`futures_lite`] runtime.
    #[inline]
    #[must_use]
    pub const fn new_async(connection: C, channel: Channel<V>) -> Self {
        Self::new(connection, channel)
    }
}

#[cfg(feature = "tokio")]
impl<C, V: PacketVersion> AsyncConnection<Tokio, C, V>
where
    Tokio: Runtime<C>,
{
    /// Create a new [`AsyncConnection`] using the [`tokio`] runtime.
    #[inline]
    #[must_use]
    pub const fn new_tokio(connection: C, channel: Channel<V>) -> Self {
        Self::new(connection, channel)
    }
}

// -------------------------------------------------------------------------------------------------

/// A marker trait for different async runtimes' connection implementations.
pub trait Runtime<C> {
    /// Reads the exact number of bytes required to fill `buf`.
    fn read_exact<'a>(
        conn: &'a mut C,
        buf: &'a mut [u8],
    ) -> impl Future<Output = std::io::Result<()>> + 'a;

    /// Spawn a task on the [`IoTaskPool`].
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret>;
}

/// Marker type for the [`futures_lite`] runtime.
#[cfg(feature = "futures-lite")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuturesLite;

#[cfg(feature = "futures-lite")]
impl<C: FAsyncRead + FAsyncWrite + Unpin> Runtime<C> for FuturesLite {
    #[inline]
    fn read_exact<'a>(
        conn: &'a mut C,
        buf: &'a mut [u8],
    ) -> impl Future<Output = std::io::Result<()>> + 'a {
        futures_lite::AsyncReadExt::read_exact(conn, buf)
    }

    #[inline]
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret> {
        IoTaskPool::get().spawn(future)
    }
}

/// Marker type for the [`tokio`] runtime.
#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tokio;

#[cfg(feature = "tokio")]
impl<C: TAsyncRead + TAsyncWrite + Unpin> Runtime<C> for Tokio {
    #[inline]
    #[allow(clippy::manual_async_fn, reason = "Control")]
    fn read_exact<'a>(
        conn: &'a mut C,
        buf: &'a mut [u8],
    ) -> impl Future<Output = std::io::Result<()>> + 'a {
        async { tokio::io::AsyncReadExt::read_exact(conn, buf).await.map(|_| ()) }
    }

    #[inline]
    #[cfg(feature = "bevy")]
    fn spawn_task<Fut: Future<Output = Ret> + Send + 'static, Ret: Send + 'static>(
        future: Fut,
    ) -> Task<Ret> {
        IoTaskPool::get().spawn(async_compat::Compat::new(future))
    }
}
