use std::sync::Arc;

use bevy_app::App;
use bevy_log::error;
use froglight_protocol::{
    common::{ConnectionIntent, GameProfile},
    packet::ServerStatus,
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::HandleConnection;
use crate::connection::{
    plugin::{
        channel::{conn_enum::ConnectionEnum, current::async_task::PacketChannel},
        systems::states::{
            configuration::ConfigurationState, handshaking::HandshakeState, login::LoginState,
            play::PlayState, status::StatusState,
        },
    },
    Connection, ConnectionChannel, ConnectionError, NetworkDirection, Serverbound,
};

impl<V: Version> HandleConnection for V
where
    V: HandshakeState + StatusState + LoginState + ConfigurationState + PlayState,
    Serverbound: NetworkDirection<V, Handshaking>
        + NetworkDirection<V, Status>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    type Channel = ConnectionChannel<Self>;

    fn add_systems(_app: &mut App) {}

    async fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> Result<(), ConnectionError> {
        Self::version_handshake(conn, intent).await
    }

    async fn perform_status(
        conn: &mut Connection<Self, Status>,
    ) -> Result<ServerStatus, ConnectionError> {
        Self::version_status_request(conn).await
    }

    async fn perform_login(
        conn: &mut Connection<Self, Login>,
    ) -> Result<GameProfile, ConnectionError> {
        Self::version_login(conn).await
    }

    async fn task_function(conn: Connection<Self, Login>, channel: PacketChannel<Self>) {
        let conn = ConnectionEnum::from_config(conn.configuration());
        let _ = futures::try_join!(
            listen_from_server(conn.clone(), channel.clone()),
            listen_from_channel(channel, conn)
        );
    }
}

/// Listens for packets from the server and sends them to the channel.
///
/// (Bevy)
/// Server -> Channel
async fn listen_from_server<V>(
    mut conn: ConnectionEnum<V>,
    channel: PacketChannel<V>,
) -> Result<(), ()>
where
    V: Version + ConfigurationState + PlayState,
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    let mut state = match conn {
        ConnectionEnum::Config(_) => ConnectionIntent::Configuration,
        ConnectionEnum::Play(_) => ConnectionIntent::Play,
    };

    loop {
        match state {
            ConnectionIntent::Configuration => match conn.recv_config().await {
                Ok(Some(packet)) => {
                    // Check if the connection should enter the Play state
                    if V::clientbound_enter_play(&packet) {
                        state = ConnectionIntent::Play;
                    }

                    // Send the Configuration packet through the channel
                    if let Err(err) = channel.send_config(Arc::new(packet)).await {
                        error!("Failed to send Configuration packet: \"{err:?}\"");
                        return Err(());
                    }
                }
                Ok(None) => {
                    error!("Received `None` from Connection::recv_config");
                }
                Err(err) => {
                    error!("Failed to receive Configuration packet: \"{err:?}\"");

                    if let Err(err) = channel.send_error(err).await {
                        error!("Failed to send error to Channel: \"{err:?}\"");
                        return Err(());
                    }

                    #[cfg(debug_assertions)]
                    {
                        error!("Debug: Closing Connection");
                        return Err(());
                    }
                }
            },
            ConnectionIntent::Play => match conn.recv_play().await {
                Ok(Some(packet)) => {
                    // Check if the connection should enter the Configuration state
                    if V::clientbound_enter_configuration(&packet) {
                        state = ConnectionIntent::Configuration;
                    }

                    // Send the Play packet through the channel
                    if let Err(err) = channel.send_play(Arc::new(packet)).await {
                        error!("Failed to send Play packet: \"{err:?}\"");
                        return Err(());
                    }
                }
                Ok(None) => {
                    error!("Received `None` from Connection::recv_play");
                }
                Err(err) => {
                    error!("Failed to receive Play packet: \"{err:?}\"");
                    return Err(());
                }
            },
            _ => unreachable!(),
        }
    }
}

/// Listens for packets from the channel and sends them to the server.
///
/// (Bevy)
/// Channel -> Server
async fn listen_from_channel<V>(
    channel: PacketChannel<V>,
    mut conn: ConnectionEnum<V>,
) -> Result<(), ()>
where
    V: Version + ConfigurationState + PlayState,
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    let mut state = match conn {
        ConnectionEnum::Config(_) => ConnectionIntent::Configuration,
        ConnectionEnum::Play(_) => ConnectionIntent::Play,
    };

    loop {
        match state {
            ConnectionIntent::Configuration => match channel.recv_config().await {
                Ok(packet) => {
                    // Check if the connection should enter the Play state
                    if V::serverbound_enter_play(&packet) {
                        state = ConnectionIntent::Play;
                    }

                    // Send the Configuration packet to the server
                    if let Err(err) = conn.send_config(&packet).await {
                        if let Some(err) = err {
                            error!("Failed to send Configuration packet: \"{err:?}\"");

                            if let Err(err) = channel.send_error(err).await {
                                error!("Failed to send error to Channel: \"{err:?}\"");
                            }
                        }
                        return Err(());
                    }
                }
                Err(err) => {
                    error!("Failed to receive Configuration packet: \"{err:?}\"");
                    return Err(());
                }
            },
            ConnectionIntent::Play => match channel.recv_play().await {
                Ok(packet) => {
                    // Check if the connection should enter the Configuration state
                    if V::serverbound_enter_configuration(&packet) {
                        state = ConnectionIntent::Configuration;
                    }

                    // Send the Play packet to the server
                    if let Err(err) = conn.send_play(&packet).await {
                        if let Some(err) = err {
                            error!("Failed to send Play packet: \"{err:?}\"");

                            if let Err(err) = channel.send_error(err).await {
                                error!("Failed to send error to Channel: \"{err:?}\"");
                            }
                        }
                        return Err(());
                    }
                }
                Err(err) => {
                    error!("Failed to receive Play packet: \"{err:?}\"");
                    return Err(());
                }
            },
            _ => unreachable!(),
        }
    }
}
