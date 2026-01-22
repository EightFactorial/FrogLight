//! TODO

use core::marker::PhantomData;

use facet::Facet;
use facet_format::{DeserializeError as FDError, SerializeError as FSError};
use facet_minecraft::{deserialize::DeserializeError, serialize::SerializeError};
use froglight_packet::version::{Clientbound, PacketVersion, Serverbound, VersionPacket};
#[cfg(feature = "futures-lite")]
use futures_lite::{AsyncRead as FAsyncRead, AsyncWrite as FAsyncWrite};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead as TAsyncRead, AsyncWrite as TAsyncWrite};

use crate::connection::channel::ConnectionChannel;

/// A [`Version`]'ed connection that uses a specific [`Runtime`].
#[derive(Clone)]
pub struct AsyncConnection<R: Runtime<C>, C, V: PacketVersion> {
    connection: C,
    channel: Channel<V>,
    _phantom: PhantomData<(R, V)>,
}

type Channel<V> = ConnectionChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>;

impl<R: Runtime<C>, C, V: PacketVersion> AsyncConnection<R, C, V> {
    /// Create a new [`AsyncConnection`].
    #[inline]
    #[must_use]
    pub const fn new(connection: C, channel: Channel<V>) -> Self {
        Self { connection, channel, _phantom: PhantomData }
    }

    /// Change the [`Runtime`] of this [`AsyncConnection`].
    #[inline]
    #[must_use]
    pub fn with_runtime<R2: Runtime<C>>(self) -> AsyncConnection<R2, C, V> {
        AsyncConnection {
            connection: self.connection,
            channel: self.channel,
            _phantom: PhantomData,
        }
    }

    /// Get a reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn connection(&self) -> &C { &self.connection }

    /// Get a mutable reference to the underlying connection.
    #[inline]
    #[must_use]
    pub const fn connection_mut(&mut self) -> &mut C { &mut self.connection }

    /// Get a reference to the connection's channel.
    #[inline]
    #[must_use]
    pub const fn channel(&self) -> &Channel<V> { &self.channel }

    /// Read a type from the connection.
    #[inline]
    pub fn read_type<T: Facet<'static>>(
        &mut self,
    ) -> impl core::future::Future<Output = Result<T, FDError<DeserializeError>>> + '_ {
        R::read_type(&mut self.connection)
    }

    /// Write a type to the connection.
    #[inline]
    pub fn write_type<'a, T: Facet<'static>>(
        &'a mut self,
        value: &'a T,
    ) -> impl core::future::Future<Output = Result<(), FSError<SerializeError>>> + 'a {
        R::write_type(&mut self.connection, value)
    }

    /// Separate the [`AsyncConnection`] into its parts.
    #[inline]
    #[must_use]
    pub fn into_parts(self) -> (C, Channel<V>) { (self.connection, self.channel) }
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
    /// Read a type from the given reader.
    fn read_type<T: Facet<'static>>(
        reader: &mut C,
    ) -> impl Future<Output = Result<T, FDError<DeserializeError>>>;

    /// Write a type to the given writer.
    fn write_type<T: Facet<'static>>(
        writer: &mut C,
        value: &T,
    ) -> impl Future<Output = Result<(), FSError<SerializeError>>>;
}

/// Marker type for the [`futures_lite`] runtime.
#[cfg(feature = "futures-lite")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FuturesLite;

#[cfg(feature = "futures-lite")]
impl<C: FAsyncRead + FAsyncWrite + Unpin> Runtime<C> for FuturesLite {
    #[inline]
    fn read_type<T: Facet<'static>>(
        reader: &mut C,
    ) -> impl Future<Output = Result<T, FDError<DeserializeError>>> {
        facet_minecraft::from_async_reader::<T, C>(reader)
    }

    #[inline]
    fn write_type<T: Facet<'static>>(
        writer: &mut C,
        value: &T,
    ) -> impl Future<Output = Result<(), FSError<SerializeError>>> {
        facet_minecraft::to_async_writer::<T, C>(value, writer)
    }
}

/// Marker type for the [`tokio`] runtime.
#[cfg(feature = "tokio")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tokio;

#[cfg(feature = "tokio")]
impl<C: TAsyncRead + TAsyncWrite + Unpin> Runtime<C> for Tokio {
    #[inline]
    fn read_type<T: Facet<'static>>(
        reader: &mut C,
    ) -> impl Future<Output = Result<T, FDError<DeserializeError>>> {
        facet_minecraft::from_tokio_reader::<T, C>(reader)
    }

    #[inline]
    fn write_type<T: Facet<'static>>(
        writer: &mut C,
        value: &T,
    ) -> impl Future<Output = Result<(), FSError<SerializeError>>> {
        facet_minecraft::to_tokio_writer::<T, C>(value, writer)
    }
}
