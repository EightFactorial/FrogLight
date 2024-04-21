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
    LegacyPacketChannel, LoginPlugins, NetworkDirection, PacketChannel, Serverbound,
};

mod generic;
mod v1_20_0;

// --- Event Systems ---

#[cfg(feature = "resolver")]
pub(super) fn listen_status_request<V: Version + ConnectionHandler>(
    mut events: EventReader<StatusRequest>,
    resolver: Res<crate::resolver::Resolver>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Handshaking> + NetworkDirection<V, Status> + NetworkDirection<V, Login>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
{
    for event in events.read().filter(|&event| event.is_version::<V>()) {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            entity.insert(V::status_of(&event.address, &resolver));
        } else {
            warn!("StatusRequest entity does not exist!");
        }
    }
}

#[cfg(not(feature = "resolver"))]
pub(super) fn listen_status_request<V: Version + ConnectionHandler>(
    mut events: EventReader<StatusRequest>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Handshaking> + NetworkDirection<V, Status> + NetworkDirection<V, Login>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
{
    for event in events.read().filter(|&event| event.is_version::<V>()) {
        if let Some(mut entity) = commands.get_entity(event.entity) {
            entity.insert(V::status_of_socket(&event.address, &resolver));
        } else {
            warn!("StatusRequest entity does not exist!");
        }
    }
}

#[cfg(feature = "resolver")]
pub(super) fn listen_connection_request<V: Version + ConnectionHandler>(
    mut events: EventReader<ConnectionRequest>,
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
    use crate::connection::ConnectionMarker;

    for event in events.read().filter(|&event| event.is_version::<V>()) {
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
pub(super) fn listen_connection_request<V: Version + ConnectionHandler>(
    mut events: EventReader<ConnectionRequest>,
    plugins: Res<LoginPlugins>,
    mut commands: Commands,
) where
    Serverbound:
        NetworkDirection<V, Handshaking> + NetworkDirection<V, Status> + NetworkDirection<V, Login>,
    Handshaking: State<V>,
    Status: State<V>,
    Login: State<V>,
{
    for event in events.read().filter(|&event| event.is_version::<V>()) {
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
fn fire_legacy_recvpacket<V: Version, S: State<V>>(
    query: Query<(Entity, &LegacyPacketChannel<V>)>,
    mut events: EventWriter<RecvPacket<V, S>>,
    mut commands: Commands,
) where
    Serverbound: NetworkDirection<V, S> + NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    LegacyPacketChannel<V>: PacketTrait<V, S>,
    Login: State<V>,
    Play: State<V>,
{
    for (entity, channel) in &query {
        let pair = <LegacyPacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
        loop {
            match pair.recv.try_recv() {
                Ok(packet) => {
                    events.send(RecvPacket::new(packet, entity));
                }
                Err(err) => {
                    if let TryRecvError::Closed = err {
                        warn!("LegacyPacketChannel was closed, despawning!");
                        commands.entity(entity).despawn();
                    }
                    break;
                }
            }
        }
    }
}

// Listens for SendPacket events and sends the packets through the channel.
fn listen_legacy_sendpacket<V: Version, S: State<V>>(
    query: Query<(Entity, &LegacyPacketChannel<V>)>,
    mut events: EventReader<SendPacket<V, S>>,
    mut commands: Commands,
) where
    Serverbound: NetworkDirection<V, S> + NetworkDirection<V, Login> + NetworkDirection<V, Play>,
    LegacyPacketChannel<V>: PacketTrait<V, S>,
    Login: State<V>,
    Play: State<V>,
{
    for event in events.read() {
        if let Some(entity) = event.connection {
            // Send to specific connection
            match query.get(entity) {
                Ok((entity, channel)) => {
                    let pair = <LegacyPacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                    send_through_pair(event.packet.clone(), pair, entity, &mut commands);
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
            for (entity, channel) in &query {
                let pair = <LegacyPacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                send_through_pair(event.packet.clone(), pair, entity, &mut commands);
            }
        }
    }
}

// --- Current Systems ---

#[allow(dead_code)]
fn fire_sendpacket<V: Version, S: State<V>>(
    query: Query<(Entity, &PacketChannel<V>)>,
    mut events: EventWriter<RecvPacket<V, S>>,
    mut commands: Commands,
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
    for (entity, channel) in &query {
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
}

#[allow(dead_code)]
fn listen_sendpacket<V: Version, S: State<V>>(
    query: Query<(Entity, &PacketChannel<V>)>,
    mut events: EventReader<SendPacket<V, S>>,
    mut commands: Commands,
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
                    send_through_pair(event.packet.clone(), pair, entity, &mut commands);
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
            for (entity, channel) in &query {
                let pair = <PacketChannel<V> as PacketTrait<V, S>>::get_pair(channel);
                send_through_pair(event.packet.clone(), pair, entity, &mut commands);
            }
        }
    }
}

// --- Helper Functions ---

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
