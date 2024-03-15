use std::{marker::PhantomData, net::SocketAddr};

use async_std::net::TcpStream;
use async_std_resolver::{lookup::Lookup, proto::rr::RData};
use bevy_app::App;
use bevy_log::{debug, error};
use froglight_protocol::{
    states::Handshaking,
    traits::{State, Version},
};

mod conversion;

mod direction;
pub use direction::{Clientbound, NetworkDirection, Serverbound};

mod error;
pub use error::ConnectionError;

#[doc(hidden)]
pub(super) fn build(_app: &mut App) {}

/// A connection to a server or client.
#[derive(Debug)]
pub struct Connection<V: Version, S: State<V>, D: NetworkDirection<V, S> = Serverbound> {
    pub(crate) _stream: TcpStream,
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    _direction: PhantomData<D>,
}

impl<V: Version, D: NetworkDirection<V, Handshaking>> Connection<V, Handshaking, D>
where
    Handshaking: State<V>,
{
    /// Connect to a server at the given address.
    ///
    /// # Errors
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    #[inline]
    pub async fn connect(address: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(address).await?;
        Self::from_stream(stream).await
    }

    /// Connect to a server using a DNS lookup, using the first valid address
    /// record.
    ///
    /// If no `port` is given, the default port `25565` is used.
    ///
    /// # Errors
    /// If there are no `A` or `AAAA` address records.
    /// If the connection cannot be established.
    /// If the connection cannot be set to nodelay.
    pub async fn from_lookup(lookup: &Lookup, port: Option<u16>) -> Result<Self, ConnectionError> {
        let port = port.unwrap_or(25565);

        // Keep track of whether a connection was attempted
        let mut attempted_connection = false;

        for record in lookup.record_iter() {
            match record.data() {
                Some(RData::A(data)) => {
                    debug!("Attempting to connect to {}", data.0);
                    attempted_connection = true;

                    match Self::connect(SocketAddr::new(data.0.into(), port)).await {
                        Ok(connection) => return Ok(connection),
                        Err(err) => {
                            error!("Failed to connect to {}: {err}", data.0);
                        }
                    }
                }
                Some(RData::AAAA(data)) => {
                    debug!("Attempting to connect to {}", data.0);
                    attempted_connection = true;

                    match Self::connect(SocketAddr::new(data.0.into(), port)).await {
                        Ok(connection) => return Ok(connection),
                        Err(err) => {
                            error!("Failed to connect to {}: {err}", data.0);
                        }
                    }
                }
                _ => {}
            }
        }

        // Return the appropriate error
        if attempted_connection {
            Err(ConnectionError::NoConnection)
        } else {
            Err(ConnectionError::NoAddressRecords)
        }
    }
}

#[allow(clippy::unused_async)]
impl<V: Version, S: State<V>, D: NetworkDirection<V, S>> Connection<V, S, D> {
    /// Send a packet to the other side of the connection.
    ///
    /// # Errors
    /// If a packet cannot be sent.
    pub async fn send(&mut self, _packet: impl Into<D::Send>) -> Result<(), ConnectionError> {
        todo!()
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

        debug!("Connected to {}", stream.peer_addr()?);
        Ok(Self {
            _stream: stream,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        })
    }
}
