use std::{collections::VecDeque, marker::PhantomData, sync::Arc};

use async_std::{io::BufReader, net::TcpStream, sync::RwLock};
use froglight_protocol::traits::{State, Version};

use super::{AccountInformation, ConnectionInformation, NetworkDirection, Serverbound};

/// The read half of a [`Connection`](super::Connection).
///
/// Can only be used to read packets from the connection.
#[derive(Debug)]
pub struct ReadConnection<V, S, D = Serverbound>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    pub(crate) stream: BufReader<TcpStream>,
    pub(crate) bundle: VecDeque<D::Recv>,
    pub(crate) compression: Arc<RwLock<Option<i32>>>,
    /// Information about the connection.
    pub info: Arc<RwLock<ConnectionInformation>>,
    /// Information about the account.
    pub account: Arc<RwLock<AccountInformation>>,
    pub(super) _version: PhantomData<V>,
    pub(super) _state: PhantomData<S>,
    pub(super) _direction: PhantomData<D>,
}

impl<V, S, D> ReadConnection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Returns the inner [`TcpStream`].
    #[must_use]
    pub fn into_stream(self) -> BufReader<TcpStream> { self.stream }

    /// Returns the compression level of the connection.
    pub fn get_compression(&self) -> &RwLock<Option<i32>> { &self.compression }

    /// Set the state of the connection.
    #[must_use]
    pub fn set_state<S2>(self) -> ReadConnection<V, S2, D>
    where
        S2: State<V>,
        D: NetworkDirection<V, S2>,
    {
        ReadConnection {
            stream: self.stream,
            compression: self.compression,
            info: self.info,
            account: self.account,
            bundle: VecDeque::new(),
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        }
    }
}

/// The write half of a [`Connection`](super::Connection).
///
/// Can only be used to write packets to the connection.
#[derive(Debug, Clone)]
pub struct WriteConnection<V, S, D = Serverbound>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    pub(crate) stream: TcpStream,
    pub(crate) compression: Arc<RwLock<Option<i32>>>,
    /// Information about the connection.
    pub info: Arc<RwLock<ConnectionInformation>>,
    /// Information about the account.
    pub account: Arc<RwLock<AccountInformation>>,
    pub(super) _version: PhantomData<V>,
    pub(super) _state: PhantomData<S>,
    pub(super) _direction: PhantomData<D>,
}

impl<V, S, D> WriteConnection<V, S, D>
where
    V: Version,
    S: State<V>,
    D: NetworkDirection<V, S>,
{
    /// Returns the inner [`TcpStream`].
    #[must_use]
    pub fn into_stream(self) -> TcpStream { self.stream }

    /// Returns the compression level of the connection.
    pub fn get_compression(&self) -> &RwLock<Option<i32>> { &self.compression }

    /// Set the state of the connection.
    #[must_use]
    pub fn set_state<S2>(self) -> WriteConnection<V, S2, D>
    where
        S2: State<V>,
        D: NetworkDirection<V, S2>,
    {
        WriteConnection {
            stream: self.stream,
            compression: self.compression,
            info: self.info,
            account: self.account,
            _version: PhantomData,
            _state: PhantomData,
            _direction: PhantomData,
        }
    }
}
