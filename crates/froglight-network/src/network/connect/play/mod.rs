use std::future::Future;

use froglight_protocol::{
    states::{Configuration, Handshake, Login, Play, Status},
    traits::{State, Version},
};

use crate::{
    connection::{Connection, ConnectionError, NetworkDirection, Serverbound},
    network::channel::{AsyncConnectionChannel, ConnectionHolder},
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
        conn: ConnectionHolder<Self, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> impl Future<Output = Result<ConnectionHolder<Self, Serverbound>, ConnectionError>> + Send + Sync;

    fn perform_configuration(
        conn: ConnectionHolder<Self, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> impl Future<Output = Result<ConnectionHolder<Self, Serverbound>, ConnectionError>> + Send + Sync;

    fn perform_play(
        conn: ConnectionHolder<Self, Serverbound>,
        channel: &AsyncConnectionChannel<Self, Serverbound>,
    ) -> impl Future<Output = Result<Option<ConnectionHolder<Self, Serverbound>>, ConnectionError>>
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
    // Split the connection into read and write halves.
    let (read, write) = hand.into_split();
    let hand = ConnectionHolder::Handshake(read, write);

    // Perform the login.
    let login = V::perform_login(hand.into_login(), channel).await?;

    // Perform the configuration.
    let mut config = V::perform_configuration(login.into_config(), channel).await?;

    // Connect to the server and play.
    // If it returns an open connection, perform configuration and play again.
    while let Some(play) = V::perform_play(config.into_play(), channel).await? {
        config = V::perform_configuration(play.into_config(), channel).await?;
    }

    Ok(())
}
