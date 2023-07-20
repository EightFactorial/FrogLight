#![allow(unused_variables)]
#![allow(dead_code)]

use std::marker::PhantomData;

use async_std::net::TcpStream;

use crate::{State, Version};

#[derive(Debug, Clone)]
pub struct Connection<V: Version, S: State<V>> {
    _version: PhantomData<V>,
    _state: PhantomData<S>,
    compression: Option<i32>,
    stream: TcpStream,
}

impl<V: Version, S: State<V>> Connection<V, S> {
    /// Creates a new connection.
    pub fn new(version: V, stream: TcpStream) -> Self {
        Self {
            _version: PhantomData,
            _state: PhantomData,
            compression: None,
            stream,
        }
    }

    /// Sends a packet to the server.
    pub fn send_packet(&mut self, packet: impl Into<<S as State<V>>::Serverbound>) { todo!() }

    /// Receives a packet from the server.
    pub fn receive_packet(&mut self) -> <S as State<V>>::Clientbound { todo!() }

    /// Converts this connection into a connection with a different state.
    ///
    /// You cannot convert a connection into a connection with a different version.
    pub fn into<S2>(self) -> Connection<V, S2>
    where
        S2: State<V>,
    {
        Connection {
            _version: PhantomData,
            _state: PhantomData,
            compression: self.compression,
            stream: self.stream,
        }
    }

    /// Sets the compression threshold.
    pub fn set_compression(&mut self, threshold: Option<i32>) { self.compression = threshold }

    /// Gets the compression threshold.
    pub fn get_compression(&self) -> &Option<i32> { &self.compression }

    /// Destroy the connection and return the underlying stream.
    pub fn into_inner(self) -> TcpStream { self.stream }
}
