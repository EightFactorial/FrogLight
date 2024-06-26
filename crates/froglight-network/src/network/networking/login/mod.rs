use froglight_protocol::{
    states::{Configuration, Login, Play},
    traits::{State, Version},
};

use crate::connection::{
    Connection, ConnectionError, NetworkDirection, Serverbound, WriteConnection,
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
    /// Send the initial login packet to the server.
    fn perform_login(
        conn: Connection<Self, Login, Serverbound>,
    ) -> impl std::future::Future<Output = Result<Connection<Self, Login>, ConnectionError>> + Send + Sync;

    /// Returns `true` if the login finished successfully,
    /// or `false` if the login is still in progress.
    fn login_state_handle(
        packet: &<Serverbound as NetworkDirection<Self, Login>>::Recv,
        conn: &WriteConnection<Self, Login, Serverbound>,
    ) -> impl std::future::Future<Output = Result<bool, ConnectionError>> + Send + Sync;

    /// Returns `true` when the end of the login has been acknowledged.
    fn login_ack_handle(packet: &<Serverbound as NetworkDirection<Self, Login>>::Send) -> bool;
}
