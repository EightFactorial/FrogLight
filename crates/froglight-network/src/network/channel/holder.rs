use froglight_protocol::{
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use crate::connection::{NetworkDirection, ReadConnection as Read, WriteConnection as Write};

/// An enum representing a connection in a certain state.
pub enum ConnectionHolder<V: Version, D>
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// A [`Connection`](crate::connection::Connection) in the [`Handshake`]
    /// state.
    Handshake(Read<V, Handshake, D>, Write<V, Handshake, D>),
    /// A [`Connection`](crate::connection::Connection) in the [`Status`] state.
    Status(Read<V, Status, D>, Write<V, Status, D>),
    /// A [`Connection`](crate::connection::Connection) in the [`Login`] state.
    Login(Read<V, Login, D>, Write<V, Login, D>),
    /// A [`Connection`](crate::connection::Connection) in the [`Configuration`]
    /// state.
    Config(Read<V, Configuration, D>, Write<V, Configuration, D>),
    /// A [`Connection`](crate::connection::Connection) in the [`Play`] state.
    Play(Read<V, Play, D>, Write<V, Play, D>),
}

impl<V: Version, D> ConnectionHolder<V, D>
where
    D: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    /// Set the state of the connection to the [`Handshake`].
    #[must_use]
    pub fn into_handshake(self) -> Self {
        match self {
            ConnectionHolder::Handshake(read, write) => ConnectionHolder::Handshake(read, write),
            ConnectionHolder::Status(read, write) => {
                ConnectionHolder::Handshake(read.set_state(), write.set_state())
            }
            ConnectionHolder::Login(read, write) => {
                ConnectionHolder::Handshake(read.set_state(), write.set_state())
            }
            ConnectionHolder::Config(read, write) => {
                ConnectionHolder::Handshake(read.set_state(), write.set_state())
            }
            ConnectionHolder::Play(read, write) => {
                ConnectionHolder::Handshake(read.set_state(), write.set_state())
            }
        }
    }

    /// Set the state of the connection to the [`Status`].
    #[must_use]
    pub fn into_status(self) -> Self {
        match self {
            ConnectionHolder::Handshake(read, write) => {
                ConnectionHolder::Status(read.set_state(), write.set_state())
            }
            ConnectionHolder::Status(read, write) => ConnectionHolder::Status(read, write),
            ConnectionHolder::Login(read, write) => {
                ConnectionHolder::Status(read.set_state(), write.set_state())
            }
            ConnectionHolder::Config(read, write) => {
                ConnectionHolder::Status(read.set_state(), write.set_state())
            }
            ConnectionHolder::Play(read, write) => {
                ConnectionHolder::Status(read.set_state(), write.set_state())
            }
        }
    }

    /// Set the state of the connection to the [`Login`].
    #[must_use]
    pub fn into_login(self) -> Self {
        match self {
            ConnectionHolder::Handshake(read, write) => {
                ConnectionHolder::Login(read.set_state(), write.set_state())
            }
            ConnectionHolder::Status(read, write) => {
                ConnectionHolder::Login(read.set_state(), write.set_state())
            }
            ConnectionHolder::Login(read, write) => ConnectionHolder::Login(read, write),
            ConnectionHolder::Config(read, write) => {
                ConnectionHolder::Login(read.set_state(), write.set_state())
            }
            ConnectionHolder::Play(read, write) => {
                ConnectionHolder::Login(read.set_state(), write.set_state())
            }
        }
    }

    /// Set the state of the connection to the [`Configuration`].
    #[must_use]
    pub fn into_config(self) -> Self {
        match self {
            ConnectionHolder::Handshake(read, write) => {
                ConnectionHolder::Config(read.set_state(), write.set_state())
            }
            ConnectionHolder::Status(read, write) => {
                ConnectionHolder::Config(read.set_state(), write.set_state())
            }
            ConnectionHolder::Login(read, write) => {
                ConnectionHolder::Config(read.set_state(), write.set_state())
            }
            ConnectionHolder::Config(read, write) => ConnectionHolder::Config(read, write),
            ConnectionHolder::Play(read, write) => {
                ConnectionHolder::Config(read.set_state(), write.set_state())
            }
        }
    }

    /// Set the state of the connection to the [`Play`].
    #[must_use]
    pub fn into_play(self) -> Self {
        match self {
            ConnectionHolder::Handshake(read, write) => {
                ConnectionHolder::Play(read.set_state(), write.set_state())
            }
            ConnectionHolder::Status(read, write) => {
                ConnectionHolder::Play(read.set_state(), write.set_state())
            }
            ConnectionHolder::Login(read, write) => {
                ConnectionHolder::Play(read.set_state(), write.set_state())
            }
            ConnectionHolder::Config(read, write) => {
                ConnectionHolder::Play(read.set_state(), write.set_state())
            }
            ConnectionHolder::Play(read, write) => ConnectionHolder::Play(read, write),
        }
    }
}
