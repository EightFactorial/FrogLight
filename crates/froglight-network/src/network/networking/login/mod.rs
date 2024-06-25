use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use crate::{
    connection::{Connection, ConnectionError, NetworkDirection, Serverbound},
    network::channel::ConnectionTaskChannel,
};

mod v1_21_0;

/// A trait that implements the [`Login`] state.
pub(super) trait LoginState: Version
where
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
    Serverbound: NetworkDirection<Self, Login>
        + NetworkDirection<Self, Configuration>
        + NetworkDirection<Self, Play>,
{
    fn perform_login(
        conn: Connection<Self, Login, Serverbound>,
        task_channel: &ConnectionTaskChannel<Self, Serverbound>,
    ) -> impl std::future::Future<
        Output = Result<Connection<Self, Login, Serverbound>, ConnectionError>,
    > + Send
           + Sync;
}
