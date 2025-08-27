//! Network connection states.
use core::{fmt::Debug, hash::Hash};

use froglight_common::version::Version;

/// A marker trait for connection states.
pub trait State: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {}

/// A marker trait for valid connection states for a given [`Version`].
pub trait ValidState<S: State>: Version {
    /// Packets sent from the server to the client.
    type Clientbound: Send + Sync;
    /// Packets sent from the client to the server.
    type Serverbound: Send + Sync;
}

// -------------------------------------------------------------------------------------------------

/// The initial handshake between client and server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handshake;
impl State for Handshake {}

// -------------------------------------------------------------------------------------------------

/// The status state, where the client can query the server's status.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status;
impl State for Status {}

// -------------------------------------------------------------------------------------------------

/// The login state, where the client begins joining the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Login;
impl State for Login {}

// -------------------------------------------------------------------------------------------------

/// The configuration state, where the server configures the client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config;
impl State for Config {}

// -------------------------------------------------------------------------------------------------

/// The play state, where the client is actively playing the game.
///
/// ---
///
/// Note: The server may at any time request to reconfigure the client,
/// which will transition the connection back into the [`Config`] state.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Play;
impl State for Play {}
