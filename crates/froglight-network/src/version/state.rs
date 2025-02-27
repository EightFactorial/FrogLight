//! All possible states of the network.
use std::{fmt::Debug, hash::Hash};

use froglight_common::version::Version;
use froglight_io::prelude::{FrogRead, FrogWrite};

/// A trait for all possible states of the network.
pub trait State: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {}

/// A trait implemented for all valid states of the network for a given version.
pub trait ValidState<S: State>: Version {
    /// Packets sent from the server to the client.
    type Clientbound: FrogRead + FrogWrite + Send + Sync;
    /// Packets sent from the client to the server.
    type Serverbound: FrogRead + FrogWrite + Send + Sync;
}

/// The initial handshake between client and server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handshake;
impl State for Handshake {}

/// The status state, where the client can query the server status.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status;
impl State for Status {}

/// The login state, where the client can login to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Login;
impl State for Login {}

/// The configuration state, where the server can configure the client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config;
impl State for Config {}

/// The play state, where the client is playing the game.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Play;
impl State for Play {}
