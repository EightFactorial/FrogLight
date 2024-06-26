use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use crate::connection::{ConnectionError, NetworkDirection, Serverbound, WriteConnection};

mod v1_21_0;

/// A trait that implements the [`Play`] state.
pub(super) trait PlayState: Version
where
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
    Serverbound: NetworkDirection<Self, Login>
        + NetworkDirection<Self, Configuration>
        + NetworkDirection<Self, Play>,
{
    /// Returns `true` if the client should exit the play state,
    /// or `false` if the client is still playing.
    fn play_state_handle(
        packet: &<Serverbound as NetworkDirection<Self, Play>>::Recv,
        conn: &WriteConnection<Self, Play, Serverbound>,
    ) -> impl std::future::Future<Output = Result<bool, ConnectionError>> + Send + Sync;

    /// Returns `true` when the end of the play state has been acknowledged.
    fn play_ack_handle(packet: &<Serverbound as NetworkDirection<Self, Play>>::Send) -> bool;
}
