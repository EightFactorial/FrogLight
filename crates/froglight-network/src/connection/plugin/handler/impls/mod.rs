use std::sync::Arc;

use async_channel::{TryRecvError, TrySendError};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    query::QueryEntityError,
    system::{Commands, Query, Res},
};
use bevy_log::{error, warn};
use froglight_protocol::{
    states::{Configuration, Handshaking, Login, Play, Status},
    traits::{State, Version},
};

use super::ConnectionHandler;
use crate::connection::{
    channels::{traits::PacketTrait, PacketPair},
    events::{ConnectionRequest, RecvPacket, SendPacket, StatusRequest},
    plugin::channels::traits::PacketChannelTrait,
    ConnectionError, ConnectionMarker, LegacyPacketChannel, LoginPlugins, NetworkDirection,
    PacketChannel, Serverbound,
};

mod generic;
mod v1_20_0;

// --- Event Systems ---

#[cfg(feature = "resolver")]
pub(super) fn listen_requests<V: Version + ConnectionHandler>(
    mut status_requests: EventReader<StatusRequest>,
    mut connection_requests: EventReader<ConnectionRequest>,
    resolver: Res<crate::resolver::Resolver>,
    plugins: Res<LoginPlugins>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Handshaking> + NetworkDirection<V, Status> + NetworkDirection<V, Login>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
{
    // Listen for `StatusRequest` events
    for event in status_requests.read().filter(|&event| event.is_version::<V>()) {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            entity.insert(V::status_of(&event.address, &resolver));
        } else {
            warn!("StatusRequest entity does not exist!");
        }
    }

    // Listen for `ConnectionRequest` events
    for event in connection_requests.read().filter(|&event| event.is_version::<V>()) {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            let (bevy_half, task_half) = V::PacketChannels::new();
            let task = V::connect_to(&event.address, task_half, &resolver, &plugins);
            entity.insert((bevy_half, task, ConnectionMarker::<V>::default()));
        } else {
            warn!("ConnectionRequest entity does not exist!");
        }
    }
}

#[cfg(not(feature = "resolver"))]
pub(super) fn listen_requests<V: Version + ConnectionHandler>(
    mut status_requests: EventReader<StatusRequest>,
    mut connection_requests: EventReader<ConnectionRequest>,
    plugins: Res<LoginPlugins>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Handshaking> + NetworkDirection<V, Status> + NetworkDirection<V, Login>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
{
    // Listen for `StatusRequest` events
    for event in status_requests.read().filter(|&event| event.is_version::<V>()) {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            entity.insert(V::status_of_socket(&event.address, &resolver));
        } else {
            warn!("StatusRequest entity does not exist!");
        }
    }

    // Listen for `ConnectionRequest` events
    for event in connection_requests.read().filter(|&event| event.is_version::<V>()) {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            let (bevy_half, task_half) = V::PacketChannels::new();
            let task = V::connect_to_socket(event.address, task_half, &plugins);
            entity.insert((bevy_half, task, ConnectionMarker::<V>::default()));
        } else {
            warn!("ConnectionRequest entity does not exist!");
        }
    }
}

// --- Legacy Systems ---

// Listens for packets to be received from the channel
// and sends out RecvPacket events.
fn fire_legacy_recvpacket<V: Version>(
    query: Query<(Entity, &LegacyPacketChannel<V>)>,
    mut login_events: EventWriter<RecvPacket<V, Login>>,
    mut play_events: EventWriter<RecvPacket<V, Play>>,
    mut commands: Commands,
) where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    LegacyPacketChannel<V>: PacketTrait<V, Login> + PacketTrait<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    for (entity, channel) in &query {
        // Send Login Packets
        receive_legacyrecvpacket(entity, channel, &mut login_events, &mut commands);
        // Send Play Packets
        receive_legacyrecvpacket(entity, channel, &mut play_events, &mut commands);
    }
}

// Listens for SendPacket events and sends the packets through the channel.
fn listen_legacy_sendpacket<V: Version>(
    query: Query<(Entity, &LegacyPacketChannel<V>)>,
    mut login_events: EventReader<SendPacket<V, Login>>,
    mut play_events: EventReader<SendPacket<V, Play>>,
    mut commands: Commands,
) where
    Serverbound: NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    LegacyPacketChannel<V>: PacketTrait<V, Login> + PacketTrait<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    // Listen Login Packets
    send_legacysendpacket(&query, &mut login_events, &mut commands);
    // Listen Play Packets
    send_legacysendpacket(&query, &mut play_events, &mut commands);
}

// --- Current Systems ---

fn fire_recvpacket<V: Version>(
    query: Query<(Entity, &PacketChannel<V>)>,
    mut login_events: EventWriter<RecvPacket<V, Login>>,
    mut config_events: EventWriter<RecvPacket<V, Configuration>>,
    mut play_events: EventWriter<RecvPacket<V, Play>>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    PacketChannel<V>: PacketTrait<V, Login> + PacketTrait<V, Configuration> + PacketTrait<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    for (entity, channel) in &query {
        // Send Login Packets
        receive_recvpacket(entity, channel, &mut login_events, &mut commands);
        // Send Configuration Packets
        receive_recvpacket(entity, channel, &mut config_events, &mut commands);
        // Send Play Packets
        receive_recvpacket(entity, channel, &mut play_events, &mut commands);
    }
}

/// Listens for [`SendPacket`] events and sends the packets through the channel.
fn listen_sendpacket<V: Version>(
    query: Query<(Entity, &PacketChannel<V>)>,
    mut login_events: EventReader<SendPacket<V, Login>>,
    mut config_events: EventReader<SendPacket<V, Configuration>>,
    mut play_events: EventReader<SendPacket<V, Play>>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Login> + NetworkDirection<V, Configuration> + NetworkDirection<V, Play>,
    PacketChannel<V>: PacketTrait<V, Login> + PacketTrait<V, Configuration> + PacketTrait<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    // Send Login Packets
    send_sendpacket(&query, &mut login_events, &mut commands);
    // Send Configuration Packets
    send_sendpacket(&query, &mut config_events, &mut commands);
    // Send Play Packets
    send_sendpacket(&query, &mut play_events, &mut commands);
}

// --- Legacy Helpers ---

fn receive_legacyrecvpacket<V: Version, S: State<V>>(
    entity: Entity,
    channel: &LegacyPacketChannel<V>,
    events: &mut EventWriter<RecvPacket<V, S>>,
    commands: &mut Commands,
) where
    Serverbound: NetworkDirection<V, S> + NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    LegacyPacketChannel<V>: PacketTrait<V, S> + PacketTrait<V, Login> + PacketTrait<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    let pair = <LegacyPacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
    loop {
        match pair.recv.try_recv() {
            Ok(packet) => {
                events.send(RecvPacket::new(packet, entity));
            }
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Closed) => {
                warn!("LegacyPacketChannel was closed, despawning!");
                commands.entity(entity).despawn();
            }
        }
    }
}

fn send_legacysendpacket<V: Version, S: State<V>>(
    query: &Query<(Entity, &LegacyPacketChannel<V>)>,
    events: &mut EventReader<SendPacket<V, S>>,
    commands: &mut Commands,
) where
    Serverbound: NetworkDirection<V, S> + NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    LegacyPacketChannel<V>: PacketTrait<V, S> + PacketTrait<V, Login> + PacketTrait<V, Play>,
    Login: State<V>,
    Play: State<V>,
{
    for event in events.read() {
        if let Some(entity) = event.connection {
            // Send to specific connection
            match query.get(entity) {
                Ok((entity, channel)) => {
                    let pair = <LegacyPacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                    send_through_pair(event.packet.clone(), pair, entity, commands);
                }
                Err(QueryEntityError::NoSuchEntity(_)) => {
                    warn!("Requested Entity does not exist!");
                }
                Err(QueryEntityError::QueryDoesNotMatch(_)) => {
                    warn!("Requested Entity does not have a LegacyPacketChannel!");
                }
                _ => unreachable!("Query is not mutable"),
            }
        } else {
            // Send to all connections
            for (entity, channel) in query.iter() {
                let pair = <LegacyPacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                send_through_pair(event.packet.clone(), pair, entity, commands);
            }
        }
    }
}

// --- Current Helpers ---

/// Receives packets from a channel and sends out [`RecvPacket`] events.
fn receive_recvpacket<V: Version, S: State<V>>(
    entity: Entity,
    channel: &PacketChannel<V>,
    events: &mut EventWriter<RecvPacket<V, S>>,
    commands: &mut Commands,
) where
    Serverbound: NetworkDirection<V, S>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    PacketChannel<V>: PacketTrait<V, S>
        + PacketTrait<V, Login>
        + PacketTrait<V, Configuration>
        + PacketTrait<V, Play>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    let pair = <PacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
    loop {
        match pair.recv.try_recv() {
            Ok(packet) => {
                events.send(RecvPacket::new(packet, entity));
            }
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Closed) => {
                warn!("PacketChannel was closed, despawning!");
                commands.entity(entity).despawn();
            }
        }
    }
}

// Sends packets through a channel
fn send_sendpacket<V: Version, S: State<V>>(
    query: &Query<(Entity, &PacketChannel<V>)>,
    events: &mut EventReader<SendPacket<V, S>>,
    commands: &mut Commands,
) where
    Serverbound: NetworkDirection<V, S>
        + NetworkDirection<V, Login>
        + NetworkDirection<V, Configuration>
        + NetworkDirection<V, Play>,
    PacketChannel<V>: PacketTrait<V, S>,
    Login: State<V>,
    Configuration: State<V>,
    Play: State<V>,
{
    for event in events.read() {
        if let Some(entity) = event.connection {
            // Send to specific connection
            match query.get(entity) {
                Ok((entity, channel)) => {
                    let pair = <PacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                    send_through_pair(event.packet.clone(), pair, entity, commands);
                }
                Err(QueryEntityError::NoSuchEntity(_)) => {
                    warn!("Requested Entity does not exist!");
                }
                Err(QueryEntityError::QueryDoesNotMatch(_)) => {
                    warn!("Requested Entity does not have a PacketChannel!");
                }
                _ => unreachable!("Query is not mutable"),
            }
        } else {
            // Send to all connections
            for (entity, channel) in query {
                let pair = <PacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                send_through_pair(event.packet.clone(), pair, entity, commands);
            }
        }
    }
}

// --- Generic Helper Functions ---

/// Sends a packet through a channel.
fn send_through_pair<V: Version, S: State<V>>(
    packet: Arc<<Serverbound as NetworkDirection<V, S>>::Send>,
    pair: &PacketPair<V, S>,
    entity: Entity,
    commands: &mut Commands,
) where
    Serverbound: NetworkDirection<V, S>,
{
    if let Err(err) = pair.send.try_send(packet) {
        match err {
            TrySendError::Full(_) => {
                warn!("PacketChannel is full, dropping packet!");
            }
            TrySendError::Closed(_) => {
                error!("PacketChannel was closed, despawning!");
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Used when reading packets, continues when unable to read a packet.
fn handle_connection_error(err: ConnectionError) -> Result<(), ConnectionError> {
    #[allow(clippy::redundant_else)]
    if let ConnectionError::PacketReadError(_) = err {
        error!("Failed to read packet from Server: \"{err:?}\"");

        #[cfg(debug_assertions)]
        {
            warn!("Debug Mode: Closing Connection");
            Err(err)
        }
        #[cfg(not(debug_assertions))]
        {
            Ok(())
        }
    } else {
        error!("Failed to receive packet from Server: \"{err:?}\"");
        Err(err)
    }
}
