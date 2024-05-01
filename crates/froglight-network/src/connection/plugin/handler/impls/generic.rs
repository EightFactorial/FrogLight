use std::sync::Arc;

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::schedule::{
    common_conditions::{any_with_component, on_event},
    Condition, IntoSystemConfigs,
};
use bevy_log::error;
use bevy_tasks::futures_lite::future::try_zip;
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::{fire_recvpacket, handle_connection_error, listen_sendpacket};
use crate::connection::{
    channels::TaskChannel,
    events::{RecvPacket, SendPacket},
    handler::{ConfigurationHandler, HandshakeHandler, LoginHandler, PlayHandler, StatusHandler},
    systemsets::{NetworkPostUpdateSet, NetworkPreUpdateSet},
    Connection, ConnectionError, ConnectionHandler, ConnectionMarker, NetworkDirection,
    PacketChannel, ReadConnection, Serverbound, WriteConnection,
};

impl<V> ConnectionHandler for V
where
    V: Version
        + HandshakeHandler
        + StatusHandler
        + LoginHandler
        + ConfigurationHandler
        + PlayHandler,
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
    type PacketChannels = PacketChannel<Self>;

    fn version_build(app: &mut App) {
        // Add login, configuration, and play events
        app.add_event::<SendPacket<Self, Play>>()
            .add_event::<RecvPacket<Self, Play>>()
            .add_event::<SendPacket<Self, Configuration>>()
            .add_event::<RecvPacket<Self, Configuration>>()
            .add_event::<SendPacket<Self, Login>>()
            .add_event::<RecvPacket<Self, Login>>();

        // Add systems to send packet events
        app.add_systems(
            PreUpdate,
            fire_recvpacket::<Self>
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .in_set(NetworkPreUpdateSet),
        );

        // Add systems to listen for packet events
        app.add_systems(
            PostUpdate,
            listen_sendpacket::<Self>
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .run_if(
                    on_event::<SendPacket<Self, Play>>().or_else(
                        on_event::<SendPacket<Self, Configuration>>()
                            .or_else(on_event::<SendPacket<Self, Login>>()),
                    ),
                )
                .in_set(NetworkPostUpdateSet),
        );
    }

    async fn handle_packets(
        conn: Connection<Self, Login>,
        channels: TaskChannel<Self>,
    ) -> ConnectionError {
        // Split the connection into read and write halves.
        let (read, write) = conn.configuration().into_split();

        // Handle packets in both directions.
        match try_zip(server_to_bevy(read, channels.clone()), bevy_to_server(write, channels)).await
        {
            Ok(((), ())) => unreachable!("Both tasks should never complete Ok"),
            Err(err) => err,
        }
    }
}

// --- Structure ---

enum ReadConnectionEnum<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    Configuration(ReadConnection<V, Configuration>),
    Play(ReadConnection<V, Play>),
}

impl<V: Version> ReadConnectionEnum<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn into_config(self) -> Self {
        match self {
            Self::Configuration(_) => self,
            Self::Play(read) => Self::Configuration(read.set_state()),
        }
    }

    fn into_play(self) -> Self {
        match self {
            Self::Play(_) => self,
            Self::Configuration(read) => Self::Play(read.set_state()),
        }
    }
}

enum WriteConnectionEnum<V: Version>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    Configuration(WriteConnection<V, Configuration>),
    Play(WriteConnection<V, Play>),
}

impl<V: Version + ConfigurationHandler + PlayHandler> WriteConnectionEnum<V>
where
    Serverbound: NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Configuration: State<V>,
    Play: State<V>,
{
    fn into_config(self) -> Self {
        match self {
            Self::Configuration(_) => self,
            Self::Play(write) => Self::Configuration(write.set_state()),
        }
    }

    fn into_play(self) -> Self {
        match self {
            Self::Play(_) => self,
            Self::Configuration(write) => Self::Play(write.set_state()),
        }
    }
}

// --- Async functions ---

/// Server -> Bevy
async fn server_to_bevy<V: Version + ConfigurationHandler + PlayHandler>(
    read: ReadConnection<V, Configuration>,
    channels: TaskChannel<V>,
) -> Result<(), ConnectionError>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    // Hold the read connection in an enum to allow for state transitions.
    let mut read = ReadConnectionEnum::Configuration(read);

    loop {
        match &mut read {
            ReadConnectionEnum::Configuration(config_read) => match config_read.recv().await {
                Ok(packet) => {
                    // Check if the connection should enter the `Play` state.
                    if V::clientbound_enter_play(&packet) {
                        read = read.into_play();
                    }

                    // Send the packet to Bevy.
                    if let Err(err) = channels.config.recv.send(Arc::new(packet)).await {
                        error!("Failed to send packet to Bevy: \"{err:?}\"");
                        return Err(ConnectionError::ConnectionClosed);
                    }
                }
                Err(err) => handle_connection_error(err)?,
            },
            ReadConnectionEnum::Play(play_read) => match play_read.recv().await {
                Ok(packet) => {
                    // Check if the connection should enter the `Configuration` state.
                    if V::clientbound_enter_config(&packet) {
                        read = read.into_config();
                    }

                    // Send the packet to Bevy.
                    if let Err(err) = channels.play.recv.send(Arc::new(packet)).await {
                        error!("Failed to send packet to Bevy: \"{err:?}\"");
                        return Err(ConnectionError::ConnectionClosed);
                    }
                }
                Err(err) => handle_connection_error(err)?,
            },
        }
    }
}

/// Bevy -> Server
async fn bevy_to_server<V: Version + ConfigurationHandler + PlayHandler>(
    write: WriteConnection<V, Configuration>,
    channels: TaskChannel<V>,
) -> Result<(), ConnectionError>
where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    // Hold the write connection in an enum to allow for state transitions.
    let mut write = WriteConnectionEnum::Configuration(write);

    loop {
        match &mut write {
            WriteConnectionEnum::Configuration(config_write) => {
                match channels.config.send.recv().await {
                    Ok(packet) => {
                        // Send the packet to the server.
                        if let Err(err) = config_write.send_packet(&packet).await {
                            error!("Failed to send packet to Server: \"{err:?}\"");
                            return Err(err);
                        }

                        // Check if the connection should enter the `Play` state.
                        if V::serverbound_enter_play(&packet) {
                            write = write.into_play();
                        }
                    }
                    Err(err) => {
                        error!("Failed to receive packet from Bevy: \"{err:?}\"");
                        return Err(ConnectionError::ConnectionClosed);
                    }
                }
            }
            WriteConnectionEnum::Play(play_write) => match channels.play.send.recv().await {
                Ok(packet) => {
                    // Send the packet to the server.
                    if let Err(err) = play_write.send_packet(&packet).await {
                        error!("Failed to send packet to Server: \"{err:?}\"");
                        return Err(err);
                    }

                    // Check if the connection should enter the `Configuration` state.
                    if V::serverbound_enter_config(&packet) {
                        write = write.into_config();
                    }
                }
                Err(err) => {
                    error!("Failed to receive packet from Bevy: \"{err:?}\"");
                    return Err(ConnectionError::ConnectionClosed);
                }
            },
        }
    }
}
