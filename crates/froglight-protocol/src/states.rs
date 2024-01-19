//! Protocol states
//!
//! TODO: Better documentation

use crate::versions::Version;

/// A Protocol state
///
/// Different states have different packets.
pub trait State<V: Version>: 'static + Copy + Eq {
    /// Packets sent from the client to the server
    type ServerboundPacket;
    /// Packets sent from the server to the client
    type ClientboundPacket;
}

/// The Handshake state
///
/// This is the initial state of a connection,
/// where the client sends a handshake packet to the server.
pub struct Handshake;

/// The Status state
///
/// This is the state where the client sends a status request to the server
/// and the server responds with a status response.
pub struct Status;

/// The Login state
///
/// This is the state where the client sends authentication information
/// to the server and encryption is enabled.
pub struct Login;

/// The Configuration state
///
/// This is the state where the client and server
/// exchange configuration information.
pub struct Configuration;

/// The Game state
///
/// This is the state where the client is connected
/// to the server and the game is running.
pub struct Game;
