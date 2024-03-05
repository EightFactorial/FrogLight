//! Protocol states
//!
//! TODO: Better documentation

use bevy_reflect::Reflect;

/// The Handshake state
///
/// This is the initial state of a connection,
/// where the client sends a handshake packet to the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Handshaking;

/// The Status state
///
/// This is the state where the client sends a status request to the server
/// and the server responds with a status response.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Status;

/// The Login state
///
/// This is the state where the client sends authentication information
/// to the server and encryption is enabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Login;

/// The Configuration state
///
/// This is the state where the client and server
/// exchange configuration information.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Configuration;

/// The Play state
///
/// This is the state where the client is connected
/// to the server and the game is running.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Play;
