//! A manual implementation of the [`HandleConnection`] trait for [`V1_20_0`],
//! since it does not have a [`Configuration`](super::Configuration) state.

use std::sync::Arc;

use async_channel::{TryRecvError, TrySendError};
use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::QueryEntityError,
    schedule::{
        common_conditions::{any_with_component, on_event},
        IntoSystemConfigs,
    },
    system::{Commands, Query},
};
use bevy_log::{error, warn};
use froglight_protocol::{
    common::{ConnectionIntent, GameProfile},
    packet::ServerStatus,
    states::{Handshaking, Login, Play, Status},
    versions::v1_20_0::V1_20_0,
};

use super::ConnectionHandler;
use crate::connection::{
    plugin::{
        channel::legacy::async_task::LegacyPacketChannel,
        systems::{
            misc::{ConnectionMarker, ConnectionPostUpdateSet, ConnectionPreUpdateSet},
            states::{handshaking::HandshakeHandler, status::StatusHandler},
        },
    },
    Connection, ConnectionError, LegacyChannel, RecvPacketEvent, SendPacketEvent,
};

impl ConnectionHandler for V1_20_0 {
    type Channel = LegacyChannel<Self>;

    fn build_version(app: &mut App) {
        // Add packet events
        app.add_event::<SendPacketEvent<Self, Play>>().add_event::<RecvPacketEvent<Self, Play>>();

        // Listen for SendPacketEvents
        app.add_systems(
            PostUpdate,
            listen_for_sendpacket_event
                .run_if(on_event::<SendPacketEvent<Self, Play>>())
                .in_set(ConnectionPostUpdateSet::<Self>::default()),
        );

        // Create RecvPacketEvents
        app.add_systems(
            PreUpdate,
            create_recvpacket_event
                .run_if(any_with_component::<ConnectionMarker<Self>>)
                .in_set(ConnectionPreUpdateSet::<Self>::default()),
        );
    }

    async fn perform_handshake(
        conn: &mut Connection<Self, Handshaking>,
        intent: ConnectionIntent,
    ) -> Result<(), ConnectionError> {
        V1_20_0::version_handshake(conn, intent).await
    }

    async fn perform_status(
        conn: &mut Connection<Self, Status>,
    ) -> Result<ServerStatus, ConnectionError> {
        V1_20_0::version_status_request(conn).await
    }

    async fn perform_login(
        _conn: &mut Connection<Self, Login>,
    ) -> Result<GameProfile, ConnectionError> {
        todo!()
    }

    async fn task_function(conn: Connection<Self, Login>, channel: LegacyPacketChannel<Self>) {
        let conn = conn.play();
        let _ = futures::try_join!(
            listen_from_server(conn.clone(), channel.clone()),
            listen_from_channel(channel, conn)
        );
    }
}

fn listen_for_sendpacket_event(
    query: Query<(Entity, &LegacyChannel<V1_20_0>)>,
    mut events: EventReader<SendPacketEvent<V1_20_0, Play>>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let Some(entity) = event.entity {
            match query.get(entity) {
                    Ok((_, channel)) => {
                        if let Err(err) = channel.send_packet(event.packet.clone()) {
                            match err {
                                TrySendError::Full(_) => {
                                    warn!("Bevy tried to send a packet to a full channel!");
                                }
                                TrySendError::Closed(_) => {
                                    error!("Bevy tried to send a packet to a closed channel!");
                                    commands.entity(entity).remove::<LegacyChannel<V1_20_0>>();
                                }
                            }
                        }
                    }
                    Err(QueryEntityError::NoSuchEntity(_)) => warn!("Bevy tried to send a packet to a non-existent entity!"),
                    Err(QueryEntityError::QueryDoesNotMatch(_)) => warn!("Bevy tried to send a packet to an entity with a different ConnectionChannel type!"),
                    _ => unreachable!("The Query is not mutable"),
                }
        } else {
            for (entity, channel) in &query {
                if let Err(err) = channel.send_packet(event.packet.clone()) {
                    match err {
                        TrySendError::Full(_) => {
                            warn!("Bevy tried to send a packet to a full channel!");
                        }
                        TrySendError::Closed(_) => {
                            error!("Bevy tried to send a packet to a closed channel!");
                            commands.entity(entity).remove::<LegacyChannel<V1_20_0>>();
                        }
                    }
                }
            }
        }
    }
}

fn create_recvpacket_event(
    query: Query<(Entity, &LegacyChannel<V1_20_0>)>,
    mut events: EventWriter<RecvPacketEvent<V1_20_0, Play>>,
    mut commands: Commands,
) {
    for (entity, channel) in &query {
        match channel.recv_packet() {
            Ok(packet) => {
                events.send(RecvPacketEvent::new(packet, entity));
            }
            Err(err) => {
                if matches!(err, TryRecvError::Closed) {
                    error!("Bevy tried to receive a packet from a closed channel!");
                    commands.entity(entity).remove::<LegacyChannel<V1_20_0>>();
                }
            }
        }
    }
}

/// Listens for packets from the server and sends them to the channel.
///
/// (Bevy)
/// Server -> Channel
async fn listen_from_server(
    mut conn: Connection<V1_20_0, Play>,
    channel: LegacyPacketChannel<V1_20_0>,
) -> Result<(), ()> {
    loop {
        match conn.recv().await {
            Ok(packet) => {
                // Send the packet to the channel
                if let Err(err) = channel.send_packet(Arc::new(packet)).await {
                    error!("Failed to send packet to Channel: \"{err:?}\"");
                    return Err(());
                }
            }
            Err(err) => {
                error!("Failed to receive packet from Connection: \"{err:?}\"");
                if let Err(err) = channel.errors.send(err).await {
                    error!("Failed to send error to Channel: \"{err:?}\"");
                    return Err(());
                }

                #[cfg(debug_assertions)]
                {
                    error!("Debug: Closing Connection");
                    return Err(());
                }
            }
        }
    }
}

/// Listens for packets from the channel and sends them to the server.
///
/// (Bevy)
/// Channel -> Server
async fn listen_from_channel(
    channel: LegacyPacketChannel<V1_20_0>,
    mut conn: Connection<V1_20_0, Play>,
) -> Result<(), ()> {
    loop {
        match channel.recv_packet().await {
            Ok(packet) => {
                // Send the packet to the server
                if let Err(err) = conn.send_packet(&packet).await {
                    error!("Failed to send packet to Connection: \"{err:?}\"");
                    return Err(());
                }
            }
            Err(err) => {
                error!("Failed to receive packet from Channel: \"{err:?}\"");
                return Err(());
            }
        }
    }
}
