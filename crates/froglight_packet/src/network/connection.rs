//! Typed network connections using [`froglight_network`].

use alloc::{boxed::Box, vec::Vec};
use core::{error::Error, marker::PhantomData};

use facet_minecraft::AssertProtocol;
use froglight_network::connection::{
    RawConnection, RawReadConnection, RawWriteConnection, Splittable,
};

use crate::{
    network::ConnConfig,
    state::{Client, Direction, Handshake, Server, State, ValidState},
};

/// A [`Connection`] from a [`Client`] to a [`Server`].
pub type ClientConnection<V, S> = Connection<V, S, Client>;
/// A [`Connection`] from a [`Server`] to a [`Client`].
pub type ServerConnection<V, S> = Connection<V, S, Server>;

/// A typed network connection.
pub struct Connection<V: ValidState<S>, S: State, D: Direction<V, S>> {
    conn: RawConnection,
    config: ConnConfig,
    buffer: Vec<u8>,
    scratch: Vec<u8>,
    _phantom: PhantomData<(V, S, D)>,
}

impl<V: ValidState<Handshake>, D: Direction<V, Handshake>> Connection<V, Handshake, D> {
    /// Create a new [`Connection`].
    #[must_use]
    #[inline]
    pub fn new<T: Splittable>(conn: T) -> Self {
        Self::new_boxed(Box::new(conn), ConnConfig::default())
    }

    /// Create a new [`Connection`] from an already boxed type.
    #[inline]
    #[must_use]
    pub const fn new_boxed(conn: Box<dyn Splittable>, config: ConnConfig) -> Self {
        Self::new_unchecked(conn, config)
    }
}

impl<V: ValidState<S>, S: State, D: Direction<V, S>> Connection<V, S, D> {
    /// Receive a packet from the connection.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn receive<'facet>(&mut self) -> Result<D::Recv, Box<dyn Error + Send + Sync>>
    where
        D::Recv: AssertProtocol<'facet>,
    {
        super::protocol::receive_type::<D::Recv, _>(
            &mut self.buffer,
            &mut self.scratch,
            &self.config,
            &mut self.conn,
        )
        .await
    }

    /// Send a packet to the connection.
    ///
    /// # Errors
    ///
    /// TODO
    #[inline]
    pub async fn send<'facet, T: Into<D::Send>>(
        &mut self,
        packet: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        D::Send: AssertProtocol<'facet>,
    {
        self.send_ref(&packet.into()).await
    }

    /// Send a packet to the connection.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn send_ref<'facet>(
        &mut self,
        packet: &D::Send,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        D::Send: AssertProtocol<'facet>,
    {
        super::protocol::send_type::<D::Send, _>(
            packet,
            &mut self.buffer,
            &mut self.scratch,
            &self.config,
            &mut self.conn,
        )
        .await
    }

    /// Split the connection into read-only and a write-only halves.
    #[must_use]
    pub fn into_split(self) -> (ReadConnection<V, S, D>, WriteConnection<V, S, D>) {
        let (read, write) = self.conn.into_split();
        (
            ReadConnection {
                conn: read,
                config: self.config.clone(),
                buffer: self.buffer,
                scratch: self.scratch,
                _phantom: PhantomData,
            },
            WriteConnection {
                conn: write,
                config: self.config,
                buffer: Vec::new(),
                scratch: Vec::new(),
                _phantom: PhantomData,
            },
        )
    }

    /// Create a new [`Connection`] from an already boxed type,
    /// without checking the current connection's state.
    #[inline]
    #[must_use]
    pub const fn new_unchecked(conn: Box<dyn Splittable>, config: ConnConfig) -> Self {
        Self {
            config,
            conn: RawConnection::new_boxed(conn),
            buffer: Vec::new(),
            scratch: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Transform this connection's state to another state,
    /// without checking if the transition is valid.
    #[inline]
    #[must_use]
    pub fn into_state_unchecked<S2: State>(self) -> Connection<V, S2, D>
    where
        V: ValidState<S2>,
        D: Direction<V, S2>,
    {
        Connection {
            conn: self.conn,
            config: self.config,
            buffer: Vec::new(),
            scratch: Vec::new(),
            _phantom: PhantomData,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// A typed, read-only network connection.
pub struct ReadConnection<V: ValidState<S>, S: State, D: Direction<V, S>> {
    conn: RawReadConnection,
    config: ConnConfig,
    buffer: Vec<u8>,
    scratch: Vec<u8>,
    _phantom: PhantomData<(V, S, D)>,
}

impl<V: ValidState<S>, S: State, D: Direction<V, S>> ReadConnection<V, S, D> {
    /// Receive a packet from the connection.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn receive<'facet>(&mut self) -> Result<D::Recv, Box<dyn Error + Send + Sync>>
    where
        D::Recv: AssertProtocol<'facet>,
    {
        super::protocol::receive_type::<D::Recv, _>(
            &mut self.buffer,
            &mut self.scratch,
            &self.config,
            &mut self.conn,
        )
        .await
    }

    /// Attempt to recombine this [`ReadConnection`] and a [`WriteConnection`]
    /// back into a full [`Connection`].
    ///
    /// # Errors
    ///
    /// Returns an error if the split halves are not from the same original
    /// connection.
    pub fn try_combine(
        self,
        write: WriteConnection<V, S, D>,
    ) -> Result<Connection<V, S, D>, Box<dyn Error + Send + Sync>> {
        Ok(Connection {
            conn: self.conn.try_combine(write.conn)?,
            config: self.config,
            buffer: self.buffer,
            scratch: self.scratch,
            _phantom: PhantomData,
        })
    }
}

// -------------------------------------------------------------------------------------------------

/// A typed, write-only network connection.
pub struct WriteConnection<V: ValidState<S>, S: State, D: Direction<V, S>> {
    conn: RawWriteConnection,
    config: ConnConfig,
    buffer: Vec<u8>,
    scratch: Vec<u8>,
    _phantom: PhantomData<(V, S, D)>,
}

impl<V: ValidState<S>, S: State, D: Direction<V, S>> WriteConnection<V, S, D> {
    /// Send a packet to the connection.
    ///
    /// # Errors
    ///
    /// TODO
    #[inline]
    pub async fn send<'facet, T: Into<D::Send>>(
        &mut self,
        packet: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        D::Send: AssertProtocol<'facet>,
    {
        self.send_ref(&packet.into()).await
    }

    /// Send a packet to the connection.
    ///
    /// # Errors
    ///
    /// TODO
    pub async fn send_ref<'facet>(
        &mut self,
        packet: &D::Send,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        D::Send: AssertProtocol<'facet>,
    {
        super::protocol::send_type::<D::Send, _>(
            packet,
            &mut self.buffer,
            &mut self.scratch,
            &self.config,
            &mut self.conn,
        )
        .await
    }

    /// Attempt to recombine this [`WriteConnection`] and a [`ReadConnection`]
    /// back into a full [`Connection`].
    ///
    /// # Errors
    ///
    /// Returns an error if the split halves are not from the same original
    /// connection.
    pub fn try_combine(
        self,
        read: ReadConnection<V, S, D>,
    ) -> Result<Connection<V, S, D>, Box<dyn Error + Send + Sync>> {
        Ok(Connection {
            conn: read.conn.try_combine(self.conn)?,
            config: self.config,
            buffer: self.buffer,
            scratch: self.scratch,
            _phantom: PhantomData,
        })
    }
}
