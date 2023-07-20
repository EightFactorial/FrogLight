/// The `Handshake` state.
///
/// This is the state the connection is in when the client first connects to the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handshake;

/// The `Status` state.
///
/// This is the state the connection is in when the client is querying the server's status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status;

/// The `Login` state.
///
/// This is the state the connection is in when the client is logging into the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Login;

/// The `Game` state.
///
/// This is the state the connection is in when the client is playing on the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Game;
