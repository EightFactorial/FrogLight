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
#[expect(dead_code, reason = "WIP")]
pub struct AsyncConnection<R: Runtime<C>, C, V: PacketVersion> {
    connection: C,
    channel: ConnectionChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>,
    _phantom: PhantomData<(R, V)>,
}

impl<R: Runtime<C>, C, V: PacketVersion> AsyncConnection<R, C, V> {
    /// Create a new [`Connection`].
    #[must_use]
    pub const fn new(
        connection: C,
        channel: ConnectionChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>,
    ) -> Self {
        Self { connection, channel, _phantom: PhantomData }
    }
}

#[cfg(feature = "futures-lite")]
impl<C, V: PacketVersion> AsyncConnection<FuturesLite, C, V>
where
    FuturesLite: Runtime<C>,
{
    /// Create a new [`Connection`] using the [`futures_lite`] runtime.
    #[inline]
    #[must_use]
    pub const fn new_async(
        connection: C,
        channel: ConnectionChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>,
    ) -> Self {
        Self::new(connection, channel)
    }
}

#[cfg(feature = "tokio")]
impl<C, V: PacketVersion> AsyncConnection<Tokio, C, V>
where
    Tokio: Runtime<C>,
{
    /// Create a new [`Connection`] using the [`tokio`] runtime.
    #[inline]
    #[must_use]
    pub const fn new_tokio(
        connection: C,
        channel: ConnectionChannel<VersionPacket<V, Serverbound>, VersionPacket<V, Clientbound>>,
    ) -> Self {
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
