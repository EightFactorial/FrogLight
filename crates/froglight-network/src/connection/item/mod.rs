use std::{collections::VecDeque, marker::PhantomData, net::SocketAddr};

use async_std::{io::BufReader, net::TcpStream};
use froglight_protocol::{
    states::Handshaking,
    traits::{State, Version},
};

mod account;
pub use account::AccountInformation;

mod conversions;
mod packets;

mod directions;
pub use directions::{Clientbound, NetworkDirection, Serverbound};

mod error;
pub use error::ConnectionError;

mod information;
pub use information::ConnectionInformation;

#[cfg(test)]
mod compile_test;

/// A connection to a remote host.
#[derive(Debug)]
pub struct Connection<V, S, D = Serverbound>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    pub(crate) stream: TcpStream,
    pub(crate) buffer: BufReader<TcpStream>,
    pub(crate) bundle: VecDeque<D::Recv>,
    pub(crate) compression: Option<i32>,
    /// Information about the connection.
    pub info: ConnectionInformation,
    /// Information about the account.
    pub account: AccountInformation,
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    _direction: PhantomData<D>,
}

impl<V> Connection<V, Handshaking, Serverbound>
where
    V: Version,
    Handshaking: State<V>,
    Serverbound: NetworkDirection<V, Handshaking>,
{
    /// Connect to a remote host using a
    /// [`Resolver`](crate::resolver::Resolver).
    ///
    /// # Errors
    /// If the address cannot be resolved.
    /// If the connection cannot be established.
    #[cfg(feature = "resolver")]
    pub async fn connect_to(
        address: &str,
        resolver: &crate::resolver::Resolver,
    ) -> Result<Self, ConnectionError> {
        use compact_str::ToCompactString;

        let address = address.to_compact_string();
        bevy_log::debug!("Resolving address: `{address}`");

        if let Some(socket) = resolver.lookup_mc(&address).await? {
            bevy_log::debug!("Connecting to `{address}`: {socket}");
            let mut connection = Self::connect(socket).await?;
            connection.info.address = Some(address);
            Ok(connection)
        } else {
            bevy_log::debug!("No records found for `{address}`");
            Err(ConnectionError::NoRecords(address))
        }
    }

    /// Connect to a remote host.
    ///
    /// # Errors
    /// If the connection cannot be established.
    pub async fn connect(socket: SocketAddr) -> Result<Self, ConnectionError> {
        let stream = TcpStream::connect(socket).await?;
        Self::from_async_stream(stream).map_err(ConnectionError::Io)
    }
}

impl<V, S, D> Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Get the account information for the connection.
    #[inline]
    #[must_use]
    pub fn account(&self) -> &AccountInformation { &self.account }

    /// Set the account information for the connection.
    #[inline]
    pub fn set_account(&mut self, account: AccountInformation) { self.account = account; }

    /// Get the compression threshold for the connection.
    #[inline]
    #[must_use]
    pub fn compression(&self) -> Option<i32> { self.compression }

    /// Set the compression threshold for the connection.
    #[inline]
    pub fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold; }

    /// Create a new connection from a [`std::net::TcpStream`].
    ///
    /// # Errors
    /// If the stream cannot retrieve or set nodelay.
    pub fn from_stream(stream: std::net::TcpStream) -> Result<Self, std::io::Error> {
        Self::from_async_stream(TcpStream::from(stream))
    }

    /// Create a new connection from an async [`TcpStream`].
    ///
    /// # Errors
    /// If the stream cannot retrieve or set nodelay.
    pub fn from_async_stream(stream: TcpStream) -> Result<Self, std::io::Error> {
        // Set the stream to nodelay
        if !stream.nodelay()? {
            stream.set_nodelay(true)?;
        }

        Ok(Self {
            info: ConnectionInformation { address: None, socket: stream.peer_addr()? },
            account: AccountInformation::default(),
            buffer: BufReader::new(stream.clone()),
            bundle: VecDeque::with_capacity(16),
            compression: None,
            stream,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        })
    }

    #[cfg(test)]
    fn nothing() {}
}

impl<V, S, D> Clone for Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    fn clone(&self) -> Self {
        Self { info: self.info.clone(), ..Self::from_async_stream(self.stream.clone()).unwrap() }
    }
}

impl<V, S, D> TryFrom<std::net::TcpStream> for Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    type Error = std::io::Error;
    fn try_from(stream: std::net::TcpStream) -> Result<Self, Self::Error> {
        Self::from_stream(stream)
    }
}

impl<V, S, D> TryFrom<TcpStream> for Connection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    type Error = std::io::Error;
    fn try_from(stream: TcpStream) -> Result<Self, Self::Error> { Self::from_async_stream(stream) }
}
