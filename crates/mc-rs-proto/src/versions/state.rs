/// The `Handshake` state.
///
/// This is the state the connection is in when the client is first connecting to the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handshaking;

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

/// The `Play` state.
///
/// This is the state the connection is in when the client is playing on the server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Play;
