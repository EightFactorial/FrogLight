//! TODO

use froglight_common::version::Version;

use crate::state::State;

/// A marker trait for valid connection states for a given [`Version`].
pub trait ValidState<S: State>: Version {
    /// Packets sent from the server to the client.
    type Clientbound: Send + Sync;
    /// Packets sent from the client to the server.
    type Serverbound: Send + Sync;
}

// -------------------------------------------------------------------------------------------------
