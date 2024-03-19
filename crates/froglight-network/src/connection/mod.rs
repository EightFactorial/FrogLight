use std::{collections::VecDeque, marker::PhantomData, net::SocketAddr};

use async_std::{io::BufReader, net::TcpStream};
use bevy_log::debug;
use froglight_protocol::{
    states::Handshaking,
    traits::{State, Version},
};

mod conversion;

mod direction;
pub use direction::{Clientbound, NetworkDirection, Serverbound};

mod error;
pub use error::ConnectionError;

mod info;
use info::ConnectionInfo;

mod send_recv;

use crate::resolver::Resolver;

/// A connection to a server or client.
#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>, D: NetworkDirection<V, S> = Serverbound> {
    pub(crate) stream: TcpStream,
    pub(crate) buffer: BufReader<TcpStream>,
    pub(crate) bundle: VecDeque<D::Recv>,
    pub(crate) compression: Option<i32>,
    /// Optional information about the connection.
    pub info: Option<ConnectionInfo>,
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
        let socket = resolver.url_lookup(address.as_ref()).await?;

        debug!("Connecting to `{}` ({socket})", address.as_ref());

        let mut connection = Self::connect(socket).await?;
        if let Some(info) = &mut connection.info {
            info.address = Some(address.as_ref().into());
        }

        Ok(connection)
    }

    /// Connect to a server at the given address.
    ///
    /// Because there is no address to resolve, [`Connection::info`]'s `address`
    /// will not be set.
    ///
    /// # Errors
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    #[inline]
    pub async fn connect(socket: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(socket).await?;

        let mut connection = Self::from_stream(stream)?;
        connection.info = Some(ConnectionInfo { address: None, socket });

        Ok(connection)
    }
}

impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> Connection<V, S, D> {
    /// Set the compression threshold.
    ///
    /// If the threshold is `None` or less than 0, compression will be disabled.
    #[inline]
    pub fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold; }

    /// Get the compression threshold.
    #[inline]
    #[must_use]
    pub fn get_compression(&self) -> Option<i32> { self.compression }

    /// Create a new connection from a TCP stream.
    ///
    /// [`Connection::info`]'s `socket` will be set if the peer's
    /// address can be retrieved.
    ///
    /// # Errors
    /// The stream cannot get or set nodelay.
    pub fn from_stream(stream: TcpStream) -> Result<Self, std::io::Error> {
        // Set the stream to nodelay
        if !stream.nodelay()? {
            stream.set_nodelay(true)?;
        }

        // Get the peer address of the stream
        let info = match stream.peer_addr() {
            Ok(socket) => Some(ConnectionInfo { address: None, socket }),
            Err(_) => None,
        };

        // Create a buffer to read from the stream
        let buffer = BufReader::new(stream.clone());

        // Create a buffer to store received bundled packets
        let bundle = VecDeque::with_capacity(8);

        Ok(Self {
            stream,
            buffer,
            bundle,
            info,
            compression: None,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        })
    }
}

impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> TryFrom<TcpStream>
    for Connection<V, S, D>
{
    type Error = std::io::Error;

    fn try_from(value: TcpStream) -> Result<Self, Self::Error> { Self::from_stream(value) }
}
