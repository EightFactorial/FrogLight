//! A connection with a [`Version`] and [`State`].
//!
//! This is a wrapper around a [`RawConnection`] that
//! guarantees the correct packets are being sent and received.

use std::{fmt::Debug, future::Future, hash::Hash, marker::PhantomData, net::SocketAddr};

use froglight_io::prelude::{FrogRead, FrogWrite, ReadError, WriteError};

use crate::{
    connection::raw::{RawConnection, RawReadConnection, RawWriteConnection},
    version::state::{Config, Handshake, Login, Play, State, Status, ValidState},
};

/// A connection from a client to a server.
pub type ClientConnection<V, S> = StateConnection<V, S, Client>;
/// A connection from a server to a client.
pub type ServerConnection<V, S> = StateConnection<V, S, Server>;

/// A connection type, either [`Client`] or a [`Server`].
pub trait ConnectionType<V, S>: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {
    /// The type of data sent on this connection.
    type Send: FrogWrite + Send + Sync + 'static;
    /// The type of data received on this connection.
    type Recv: FrogRead + Send + Sync + 'static;
}

/// A connection from a client to a server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Client;
impl<V: ValidState<S>, S: State> ConnectionType<V, S> for Client {
    /// Send serverbound data.
    type Send = <V as ValidState<S>>::Serverbound;
    /// Receive clientbound data.
    type Recv = <V as ValidState<S>>::Clientbound;
}

/// A connection from a server to a client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Server;
impl<V: ValidState<S>, S: State> ConnectionType<V, S> for Server {
    /// Send clientbound data.
    type Send = <V as ValidState<S>>::Clientbound;
    /// Receive serverbound data.
    type Recv = <V as ValidState<S>>::Serverbound;
}

/// A connection with a [`Version`] and [`State`].
///
/// This is a wrapper around a [`RawConnection`] that
/// guarantees the correct packets are being sent and received.
#[derive(Debug)]
pub struct StateConnection<V: ValidState<S>, S: State, T: ConnectionType<V, S>>(
    RawConnection,
    PhantomData<(V, S, T)>,
);

impl<V: ValidState<S>, S: State, T: ConnectionType<V, S>> StateConnection<V, S, T> {
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

    /// Read a packet from the connection.
    ///
    /// # Errors
    /// Returns an error if the packet could not be read.
    #[inline]
    pub fn read(&mut self) -> impl Future<Output = Result<T::Recv, ReadError>> + Send + Sync + '_ {
        self.0.read()
    }

    /// Write a packet to the connection.
    ///
    /// # Errors
    /// Returns an error if the packet could not be written.
    #[inline]
    pub async fn write(&mut self, packet: impl Into<T::Send>) -> Result<(), WriteError> {
        self.0.write(&packet.into()).await
    }

    /// Write a packet to the connection.
    ///
    /// # Errors
    /// Returns an error if the packet could not be written.
    #[inline]
    pub fn write_ref<'a>(
        &'a mut self,
        packet: &'a impl AsRef<T::Send>,
    ) -> impl Future<Output = Result<(), WriteError>> + Send + Sync + 'a {
        self.0.write(packet.as_ref())
    }

    /// Split the [`StateConnection`] into a
    /// [`StateReadConnection`] and a [`StateWriteConnection`].
    ///
    /// These can be recombined using [`StateConnection::from_split`].
    #[must_use]
    pub fn into_split(self) -> (StateReadConnection<V, S, T>, StateWriteConnection<V, S, T>) {
        let (read, write) = self.0.into_split();
        (StateReadConnection(read, PhantomData), StateWriteConnection(write, PhantomData))
    }

    /// Recombine a [`StateReadConnection`] and a [`StateWriteConnection`]
    /// into a [`StateConnection`].
    ///
    /// Both parts must be from the same connection, otherwise this will panic.
    ///
    /// # Panics
    /// Panics if the two connection halves are from different connections.
    #[must_use]
    pub fn from_split(
        read: StateReadConnection<V, S, T>,
        write: StateWriteConnection<V, S, T>,
    ) -> Self {
        Self(RawConnection::from_split(read.0, write.0), PhantomData)
    }

    /// Get a mutable reference to the inner [`RawConnection`].
    #[inline]
    #[must_use]
    pub fn as_raw(&mut self) -> &mut RawConnection { &mut self.0 }

    /// Create a new [`StateConnection`] from a [`RawConnection`].
    #[inline]
    #[must_use]
    pub fn from_raw(raw: RawConnection) -> Self { Self(raw, PhantomData) }

    /// Get the inner [`RawConnection`].
    #[inline]
    #[must_use]
    pub fn into_raw(self) -> RawConnection { self.0 }
}

/// A read-only connection with a [`Version`] and [`State`].
///
/// This is a wrapper around a [`RawReadConnection`] that
/// guarantees the correct packets are being received.
#[derive(Debug)]
pub struct StateReadConnection<V: ValidState<S>, S: State, T: ConnectionType<V, S>>(
    RawReadConnection,
    PhantomData<(V, S, T)>,
);

impl<V: ValidState<S>, S: State, T: ConnectionType<V, S>> StateReadConnection<V, S, T> {
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

    /// Read a packet from the connection.
    #[inline]
    pub fn read(&mut self) -> impl Future<Output = Result<T::Recv, ReadError>> + Send + Sync + '_ {
        self.0.read()
    }

    /// Get a mutable reference to the inner [`RawReadConnection`].
    #[inline]
    #[must_use]
    pub fn as_raw(&mut self) -> &mut RawReadConnection { &mut self.0 }
}

/// A write-only connection with a [`Version`] and [`State`].
///
/// This is a wrapper around a [`RawWriteConnection`] that
/// guarantees the correct packets are being sent.
#[derive(Debug)]
pub struct StateWriteConnection<V: ValidState<S>, S: State, T: ConnectionType<V, S>>(
    RawWriteConnection,
    PhantomData<(V, S, T)>,
);

impl<V: ValidState<S>, S: State, T: ConnectionType<V, S>> StateWriteConnection<V, S, T> {
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

    /// Write a packet to the connection.
    #[inline]
    pub fn write<'a>(
        &'a mut self,
        packet: &'a T::Send,
    ) -> impl Future<Output = Result<(), WriteError>> + Send + Sync + 'a {
        self.0.write(packet)
    }

    /// Get a mutable reference to the inner [`RawWriteConnection`].
    #[inline]
    #[must_use]
    pub fn as_raw(&mut self) -> &mut RawWriteConnection { &mut self.0 }
}

impl<V: ValidState<Handshake>> StateConnection<V, Handshake, Client> {
    /// Connect to a server by resolving the address.
    ///
    /// # Errors
    /// Returns an error if the connection could not be established,
    /// or if the stream could not set `nodelay` to `true`.
    #[inline]
    pub async fn connect(address: &(impl AsRef<str> + ?Sized)) -> Result<Self, std::io::Error> {
        Ok(Self(RawConnection::connect(address).await?, PhantomData))
    }

    /// Connect to a server at the given socket address.
    ///
    /// # Errors
    /// Returns an error if the connection could not be established,
    /// or if the stream could not set `nodelay` to `true`.
    #[inline]
    pub async fn connect_to(
        address: &(impl AsRef<str> + ?Sized),
        socket: SocketAddr,
    ) -> Result<Self, std::io::Error> {
        Ok(Self(RawConnection::connect_to(address, socket).await?, PhantomData))
    }
}

impl<V: ValidState<Handshake>, T: ConnectionType<V, Handshake>> StateConnection<V, Handshake, T> {
    /// Set the state of the connection to [`Query`].
    #[inline]
    #[must_use]
    pub fn query(self) -> StateConnection<V, Status, T>
    where
        V: ValidState<Status>,
        T: ConnectionType<V, Status>,
    {
        StateConnection(self.0, PhantomData)
    }

    /// Set the state of the connection to [`Login`].
    #[inline]
    #[must_use]
    pub fn login(self) -> StateConnection<V, Login, T>
    where
        V: ValidState<Login>,
        T: ConnectionType<V, Login>,
    {
        StateConnection(self.0, PhantomData)
    }
}

impl<V: ValidState<Login>, T: ConnectionType<V, Login>> StateConnection<V, Login, T> {
    /// Set the state of the connection to [`Config`].
    #[inline]
    #[must_use]
    pub fn config(self) -> StateConnection<V, Config, T>
    where
        V: ValidState<Config>,
        T: ConnectionType<V, Config>,
    {
        StateConnection(self.0, PhantomData)
    }
}

impl<V: ValidState<Config>, T: ConnectionType<V, Config>> StateConnection<V, Config, T> {
    /// Set the state of the connection to [`Play`].
    #[inline]
    #[must_use]
    pub fn play(self) -> StateConnection<V, Play, T>
    where
        V: ValidState<Play>,
        T: ConnectionType<V, Play>,
    {
        StateConnection(self.0, PhantomData)
    }
}

impl<V: ValidState<Play>, T: ConnectionType<V, Play>> StateConnection<V, Play, T> {
    /// Set the state of the connection to [`Config`].
    #[inline]
    #[must_use]
    pub fn config(self) -> StateConnection<V, Config, T>
    where
        V: ValidState<Config>,
        T: ConnectionType<V, Config>,
    {
        StateConnection(self.0, PhantomData)
    }
}
