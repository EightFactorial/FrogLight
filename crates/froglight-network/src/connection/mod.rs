use std::{collections::VecDeque, marker::PhantomData, net::SocketAddr};

use async_std::{io::WriteExt, net::TcpStream};
use froglight_protocol::{
    io::{FrogVarWrite, FrogWrite},
    states::Handshaking,
    traits::{State, Version},
};

mod conversion;

mod direction;
pub use direction::{Clientbound, NetworkDirection, Serverbound};

mod error;
pub use error::ConnectionError;

use crate::resolver::{Resolver, ResolverServerTask};

/// A connection to a server or client.
#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>, D: NetworkDirection<V, S> = Serverbound> {
    pub(crate) stream: TcpStream,
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    _direction: PhantomData<D>,
}

impl<V: Version, D: NetworkDirection<V, Handshaking>> Connection<V, Handshaking, D>
where
    Handshaking: State<V>,
{
    /// Connect to a server at an address resolved by the given resolver.
    ///
    /// # Errors
    /// If the address cannot be resolved.
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    pub async fn connect_to(
        address: &(impl AsRef<str> + ?Sized),
        resolver: &Resolver,
    ) -> Result<Self, ConnectionError> {
        let address = ResolverServerTask::url_lookup(
            resolver.resolver.clone(),
            resolver.extractor.clone(),
            address.as_ref().to_string(),
        )
        .await?;

        Self::connect(address).await
    }

    /// Connect to a server at the given address.
    ///
    /// # Errors
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    #[inline]
    pub async fn connect(socket: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(socket).await?;
        Self::from_stream(stream).await
    }
}

#[allow(clippy::unused_async)]
impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> Connection<V, S, D> {
    /// Send a packet to the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    pub async fn send(&mut self, packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        let packet: Vec<u8> = packet.into().fg_to_bytes();
        let mut buffer = VecDeque::new();

        // Write the length of the packet
        packet.len().fg_var_write(&mut buffer)?;

        // Write the packet
        buffer.extend(packet);

        // Write the buffer to the stream
        self.stream.write_all(buffer.make_contiguous()).await.map_err(Into::into)
    }

    /// Receive a packet from the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be received.
    pub async fn recv(&mut self) -> Result<D::Recv, ConnectionError> { todo!() }

    /// Create a new connection from a TCP stream.
    ///
    /// # Errors
    /// The stream cannot get or set nodelay.
    pub async fn from_stream(stream: TcpStream) -> Result<Self, ConnectionError> {
        if !stream.nodelay()? {
            stream.set_nodelay(true)?;
        }

        Ok(Self { stream, _version: PhantomData, _state: PhantomData, _direction: PhantomData })
    }
}
