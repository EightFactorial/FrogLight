use std::future::Future;

use froglight_protocol::{
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use crate::{
    connection::{Connection, ConnectionError, NetworkDirection, Serverbound},
    network::channel::AsyncConnectionChannel,
};

mod v1_21_0;

/// A trait for performing a connection to a server.
pub trait PerformServerConnection: Version
where
    Serverbound: NetworkDirection<Self, Handshake>
        + NetworkDirection<Self, Status>
        + NetworkDirection<Self, Login>
        + NetworkDirection<Self, Configuration>
        + NetworkDirection<Self, Play>,
    Handshake: State<Self>,
    Status: State<Self>,
    Login: State<Self>,
    Configuration: State<Self>,
    Play: State<Self>,
{
    fn perform_handshake(
        conn: Connection<Self, Handshake, Serverbound>,
    ) -> impl Future<Output = Result<Connection<Self, Handshake, Serverbound>, ConnectionError>>
           + Send
           + Sync;

    fn perform_login(
        conn: Connection<Self, Login, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> impl Future<Output = Result<Connection<Self, Login, Serverbound>, ConnectionError>> + Send + Sync;

    fn perform_configuration(
        conn: Connection<Self, Configuration, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> impl Future<Output = Result<Connection<Self, Configuration, Serverbound>, ConnectionError>>
           + Send
           + Sync;

    fn perform_play(
        conn: Connection<Self, Play, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> impl Future<Output = Result<Option<Connection<Self, Play, Serverbound>>, ConnectionError>>
           + Send
           + Sync;
}

pub(super) async fn perform_server_connection<V: Version + PerformServerConnection>(
    conn: Connection<V, Handshake, Serverbound>,
    channel: &AsyncConnectionChannel<V, Serverbound>,
) -> Result<(), ConnectionError>
where
    Serverbound: NetworkDirection<V, Handshake>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshake: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    #[cfg(debug_assertions)]
    bevy_log::debug!("Connecting: \"{}:{}\"", conn.info.get_address(), conn.info.get_port());

    // Perform the handshake.
    let hand = V::perform_handshake(conn).await?;
    // Perform the login.
    let login = V::perform_login(hand.login(), channel).await?;
    // Perform the configuration.
    let mut config = V::perform_configuration(login.configuration(), channel).await?;

    // Connect to the server and play.
    // If it returns an open connection, perform configuration and play again.
    while let Some(play) = V::perform_play(config.play(), channel).await? {
        config = V::perform_configuration(play.configuration(), channel).await?;
    }

    Ok(())
}
