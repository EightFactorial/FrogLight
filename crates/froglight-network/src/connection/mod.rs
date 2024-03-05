use std::{marker::PhantomData, net::TcpStream};

use bevy_app::App;
use froglight_protocol::{
    states::Handshaking,
    traits::{State, Version},
};

mod conversion;

mod direction;
pub use direction::{Clientbound, Direction, Serverbound};

mod error;
pub use error::ConnectionError;

#[doc(hidden)]
pub(super) fn build(_app: &mut App) {}

/// A connection to a server or client.
#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>, D: Direction<V, S> = Serverbound> {
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    _direction: PhantomData<D>,
}

#[allow(clippy::unused_async)]
impl<V: Version, D: Direction<V, Handshaking>> Connection<V, Handshaking, D>
where
    Handshaking: State<V>,
{
    /// Connect to a server at the given address.
    #[must_use]
    pub async fn connect(_address: &impl AsRef<str>) -> Self { todo!() }
}

#[allow(clippy::unused_async)]
impl<V: Version, S: State<V>, D: Direction<V, S>> Connection<V, S, D> {
    /// Send a packet to the other side of the connection.
    ///
    /// # Errors
    /// Errors if the packet cannot be sent.
    pub async fn send(&mut self, _packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        todo!()
    }

    /// Receive a packet from the other side of the connection.
    ///
    /// # Errors
    /// Errors if the packet cannot be received.
    pub async fn recv(&mut self) -> Result<D::Recv, ConnectionError> { todo!() }

    /// Create a new connection from a TCP stream.
    ///
    /// # Errors
    /// Errors if the stream cannot get or set nodelay.
    pub async fn from_tcp(stream: TcpStream) -> Result<Self, ConnectionError> {
        if !stream.nodelay()? {
            stream.set_nodelay(true)?;
        }

        todo!()
    }
}
