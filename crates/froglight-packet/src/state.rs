//! Network connection states.
use core::{fmt::Debug, hash::Hash};

/// A marker trait for connection states.
pub trait State: Debug + Default + Copy + Eq + Hash + Send + Sync + 'static {}

// -------------------------------------------------------------------------------------------------

/// The initial handshake between client and server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handshake;
impl State for Handshake {}

// -------------------------------------------------------------------------------------------------

/// The status state, where the client can query the server status.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Status;
impl State for Status {}

// -------------------------------------------------------------------------------------------------

/// The login state, where the client can login to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Login;
impl State for Login {}

// -------------------------------------------------------------------------------------------------

/// The configuration state, where the server can configure the client.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Config;
impl State for Config {}

// -------------------------------------------------------------------------------------------------

/// The play state, where the client is playing the game.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Play;
impl State for Play {}
