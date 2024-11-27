//! A connection to a remote host.

use std::{collections::VecDeque, marker::PhantomData, net::SocketAddr, sync::Arc};

use async_std::{io::BufReader, net::TcpStream, sync::RwLock};
use froglight_protocol::{
    states::Handshake,
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

mod halves;
pub use halves::{ReadConnection, WriteConnection};

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
    pub(crate) stream: BufReader<TcpStream>,
    pub(crate) bundle: VecDeque<D::Recv>,
    pub(crate) compression: Option<i32>,
    /// Information about the connection.
    info: ConnectionInformation,
    /// Information about the account.
    account: AccountInformation,
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    _direction: PhantomData<D>,
}

impl<V> Connection<V, Handshake, Serverbound>
where
    V: Version,
    Handshake: State<V>,
    Serverbound: NetworkDirection<V, Handshake>,
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
        #[cfg(debug_assertions)]
        bevy_log::debug!("Resolving address: `{address}`");

        let address = address.to_string();

        if let Some(socket) = resolver.lookup_mc(&address).await? {
            #[cfg(debug_assertions)]
            bevy_log::debug!("Connecting to `{address}`: {socket}");

            let mut connection = Self::connect(socket).await?;
            connection.info.address = Some(address.into());
            Ok(connection)
        } else {
            #[cfg(debug_assertions)]
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
    /// Get a reference to the [`AccountInformation`].
    #[inline]
    #[must_use]
    pub fn account(&self) -> &AccountInformation { &self.account }

    /// Get a mutable reference to the [`AccountInformation`].
    #[inline]
    #[must_use]
    pub fn account_mut(&mut self) -> &mut AccountInformation { &mut self.account }

    /// Get a reference to the compression level.
    #[inline]
    #[must_use]
    pub fn compression(&self) -> Option<i32> { self.compression }

    /// Get a mutable reference to the compression level.
    #[inline]
    #[must_use]
    pub fn compression_mut(&mut self) -> &mut Option<i32> { &mut self.compression }

    /// Get a reference to the [`ConnectionInformation`].
    #[inline]
    #[must_use]
    pub fn info(&self) -> &ConnectionInformation { &self.info }

    /// Get a mutable reference to the [`ConnectionInformation`].
    #[inline]
    #[must_use]
    pub fn info_mut(&mut self) -> &mut ConnectionInformation { &mut self.info }

    /// Split the [`Connection`] into a
    /// [`ReadConnection`] and a [`WriteConnection`].
    #[must_use]
    pub fn into_split(self) -> (ReadConnection<V, S, D>, WriteConnection<V, S, D>) {
        let compression = Arc::new(RwLock::new(self.compression));
        let info = Arc::new(RwLock::new(self.info));
        let account = Arc::new(RwLock::new(self.account));

        let write = WriteConnection {
            stream: self.stream.get_ref().clone(),
            compression: compression.clone(),
            info: info.clone(),
            account: account.clone(),
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        };
        let read = ReadConnection {
            stream: self.stream,
            bundle: self.bundle,
            compression,
            info,
            account,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        };

        (read, write)
    }

    /// Create a new [`Connection`] from a
    /// [`ReadConnection`] and a [`WriteConnection`].
    #[must_use]
    #[expect(unused_variables)]
    pub async fn from_split(
        read: ReadConnection<V, S, D>,
        write: WriteConnection<V, S, D>,
    ) -> Self {
        Self {
            stream: read.stream,
            bundle: read.bundle,
            compression: *read.compression.read().await,
            info: read.info.read().await.clone(),
            account: read.account.read().await.clone(),
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        }
    }

    /// Create a new connection from a [`std::net::TcpStream`].
    ///
    /// # Errors
    /// If the stream cannot retrieve or set nodelay.
    pub fn from_stream(stream: std::net::TcpStream) -> Result<Self, std::io::Error> {
        Self::from_async_stream(TcpStream::from(stream))
    }

    pub(super) const BUFREADER_CAPACITY: usize = 65536;

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
            stream: BufReader::with_capacity(Self::BUFREADER_CAPACITY, stream),
            bundle: VecDeque::new(),
            compression: None,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        })
    }

    /// Returns the inner [`TcpStream`].
    #[must_use]
    pub fn into_stream(self) -> BufReader<TcpStream> { self.stream }

    #[cfg(test)]
    #[allow(dead_code)]
    const fn nothing() {}
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
