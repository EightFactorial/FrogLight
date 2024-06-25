use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use crate::{
    connection::{Connection, ConnectionError, NetworkDirection, Serverbound},
    network::channel::ConnectionTaskChannel,
};

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
    fn perform_play(
        conn: Connection<Self, Play, Serverbound>,
        task_channel: &ConnectionTaskChannel<Self, Serverbound>,
    ) -> impl std::future::Future<Output = Result<(), ConnectionError>> + Send + Sync;
}
