//! Connect to a server and log the packets received.
//!
//! The connection will eventually be closed by the server
//! due to not responding to keep-alive packets.

use std::num::NonZeroU8;

use bevy::{app::AppExit, prelude::*};
use bevy_log::LogPlugin;
use froglight_network::{
    common::UnsizedBuffer,
    network::{
        ConnectionChannel, ConnectionTrait, NetworkErrorEvent, NetworkPreUpdateSet, PolledTask,
    },
    resolver::Resolver,
    versions::v1_21_0::{
        configuration::{
            ConfigurationClientboundPackets, ReadyC2SPacket, SelectKnownPacksC2SPacket,
        },
        login::{EnterConfigurationPacket, LoginClientboundPackets, LoginQueryResponsePacket},
        play::{
            AcknowledgeChunksPacket, CookieResponsePacket, CustomPayloadC2SPacket,
            PlayClientboundPackets, TeleportConfirmPacket,
        },
        V1_21_0,
    },
    NetworkPlugins,
};
use froglight_protocol::protocol::FrogWrite;

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, LogPlugin::default(), NetworkPlugins.as_plugingroup()));

    // I recommend polling for packets during the `PreUpdate` stage.
    // This way you can handle packets and move/spawn entities,
    // update inventories, etc. before the main game logic.
    app.add_systems(
        PreUpdate,
        print_packets
            .run_if(any_with_component::<ConnectionChannel<V1_21_0>>)
            .in_set(NetworkPreUpdateSet),
    );

    app.add_systems(
        Update,
        (
            // Create a connection to the server once
            create_connection.run_if(run_once()),
            // Exit if a network error occurs
            exit_on_error.run_if(on_event::<NetworkErrorEvent>()),
        )
            .chain(),
    );

    app.run()
}

/// The server address to connect to.
const SERVER_ADDRESS: &str = "localhost";

/// Create a connection to the server.
///
/// **Do not** drop the task or the channel before it is done.
/// If either are dropped the connection will immediately close.
///
/// If you don't want to manage entity lifetimes and polling tasks,
/// you can use the [`PolledTask`] [`Component`], which will
/// despawn the entity automatically when the task is done.
fn create_connection(mut commands: Commands, resolver: Res<Resolver>) {
    let (task, channel) = V1_21_0::connect(SERVER_ADDRESS, &resolver);
    commands.spawn((task, channel, PolledTask));
}

/// Log and exit if a network error occurs.
fn exit_on_error(mut events: EventReader<NetworkErrorEvent>, mut exit: EventWriter<AppExit>) {
    if let Some(error) = events.read().next() {
        error!("Error: {}", error.error);
        error!("Exiting...");
        exit.send(AppExit::Error(NonZeroU8::new(1).unwrap()));
    }
}

/// Query for any [`ConnectionChannel`]s and print any packets received.
///
/// I do recommend keeping this as one large system,
/// but it should be split up into multiple functions for readability.
///
/// Disabled the lint since it's just an example.
#[allow(clippy::too_many_lines)]
fn print_packets(channels: Query<(Entity, &ConnectionChannel<V1_21_0>)>, mut commands: Commands) {
    for (entity, channel) in channels.iter() {
        // Handle `Login` packets
        while let Ok(packet) = channel.login.recv() {
            match packet.as_ref() {
                LoginClientboundPackets::LoginSuccess(profile_packet) => {
                    // Log the profile information
                    info!("Login: Profile");
                    info!(
                        "    Username: \"{}\", Uuid: \"{}\"",
                        profile_packet.profile.name, profile_packet.profile.uuid
                    );

                    // Send an acknowledgement packet
                    channel.login.send(EnterConfigurationPacket).unwrap();

                    // Insert the profile into the ECS
                    commands.entity(entity).insert(profile_packet.profile.clone());
                }
                LoginClientboundPackets::LoginQueryRequest(query_packet) => {
                    info!(
                        "Login: Query \"{}\" -> {:?}",
                        query_packet.identifier, query_packet.payload
                    );

                    // Respond that we don't understand the query
                    channel
                        .login
                        .send(LoginQueryResponsePacket {
                            id: query_packet.id,
                            identifier: query_packet.identifier.clone(),
                            payload: None,
                        })
                        .unwrap();
                }
                LoginClientboundPackets::CookieRequest(cookie_packet) => {
                    info!("Login: Cookie \"{}\"", cookie_packet.cookie);

                    // Respond that we don't have that cookie
                    channel
                        .login
                        .send(CookieResponsePacket {
                            cookie: cookie_packet.cookie.clone(),
                            payload: None,
                        })
                        .unwrap();
                }
                _ => {
                    info!("Login: {packet:?}");
                }
            }
        }

        // Handle `Configuration` packets
        while let Ok(packet) = channel.config.recv() {
            match packet.as_ref() {
                ConfigurationClientboundPackets::SelectKnownPacks(resourcepack_packet) => {
                    info!("Config: ResourcePacks");

                    // Log the received packs
                    info!("    Received:");
                    for p in &resourcepack_packet.resourcepacks {
                        info!("        {p:?}");
                    }

                    // Filter out packs that aren't in the default namespace
                    let resourcepacks: Vec<_> =
                        resourcepack_packet
                            .iter()
                            .filter_map(|p| {
                                if p.namespace == "minecraft" {
                                    Some(p.clone())
                                } else {
                                    None
                                }
                            })
                            .collect();

                    // Log the response
                    info!("    Responded:");
                    for p in &resourcepacks {
                        info!("        {p:?}");
                    }

                    // Respond that we only know about packs in the default namespace
                    channel.config.send(SelectKnownPacksC2SPacket { resourcepacks }).unwrap();
                }
                ConfigurationClientboundPackets::CustomPayload(payload_packet) => {
                    info!(
                        "Config: Payload \"{}\" -> {:?}",
                        payload_packet.identifier, payload_packet.payload
                    );

                    // Create a response packet that says we don't understand the payload
                    let mut response = CustomPayloadC2SPacket {
                        identifier: payload_packet.identifier.clone(),
                        payload: UnsizedBuffer::new(),
                    };

                    // If the payload is a brand request, respond with "froglight"
                    if payload_packet.identifier.as_str() == "minecraft:brand" {
                        info!("    Response: \"froglight\"");
                        "froglight".fg_write(&mut response.payload).unwrap();
                    } else {
                        info!("    Response: None");
                    }

                    // Send the response
                    channel.config.send(response).unwrap();
                }
                ConfigurationClientboundPackets::CookieRequest(cookie_packet) => {
                    info!("Config: Cookie \"{}\"", cookie_packet.cookie);

                    // Respond that we don't have that cookie
                    channel
                        .config
                        .send(CookieResponsePacket {
                            cookie: cookie_packet.cookie.clone(),
                            payload: None,
                        })
                        .unwrap();
                }
                ConfigurationClientboundPackets::Ready(_) => {
                    // Send an acknowledgement packet
                    channel.config.send(ReadyC2SPacket).unwrap();
                }
                ConfigurationClientboundPackets::SynchronizeTags(tags_packet) => {
                    info!("Config: SynchronizeTags");
                    for (key, data) in &tags_packet.tags {
                        info!("    \"{key}\":");
                        for (tag, data) in &data.data {
                            info!("        \"{tag}\": {data:?}");
                        }
                    }
                }
                ConfigurationClientboundPackets::DynamicRegistries(registries_packet) => {
                    info!("Config: DynamicRegistries");
                    info!("    Identifier: \"{}\"", registries_packet.identifier);
                    for data in &registries_packet.registry_data {
                        if let Some(nbt) = &data.data {
                            info!("        \"{}\": {nbt:?},", data.identifier);
                        } else {
                            info!("        \"{}\",", data.identifier);
                        }
                    }
                }
                _ => {
                    info!("Config: {packet:?}");
                }
            }
        }

        while let Ok(packet) = channel.play.recv() {
            match packet.as_ref() {
                PlayClientboundPackets::CookieRequest(cookie_packet) => {
                    info!("Play: Cookie \"{}\"", cookie_packet.cookie);

                    // Respond that we don't have that cookie
                    channel
                        .play
                        .send(CookieResponsePacket {
                            cookie: cookie_packet.cookie.clone(),
                            payload: None,
                        })
                        .unwrap();
                }
                PlayClientboundPackets::CustomPayload(payload_packet) => {
                    info!(
                        "Play: Payload \"{}\" -> {:?}",
                        payload_packet.identifier, payload_packet.payload
                    );

                    // Respond saying we don't understand the payload
                    channel
                        .play
                        .send(CustomPayloadC2SPacket {
                            identifier: payload_packet.identifier.clone(),
                            payload: UnsizedBuffer::new(),
                        })
                        .unwrap();
                }
                PlayClientboundPackets::PlayerPositionLook(position_packet) => {
                    info!("Play: PlayerPositionLook");
                    info!("    Position: {:?}", position_packet.position);
                    info!("    Yaw: {:?}, Pitch: {:?}", position_packet.yaw, position_packet.pitch);

                    // Respond that we accepted the position
                    channel
                        .play
                        .send(TeleportConfirmPacket { teleport_id: position_packet.teleport_id })
                        .unwrap();
                }
                PlayClientboundPackets::ChunkData(chunk_packet) => {
                    info!("Play: ChunkData");
                    info!("    Position: {:?}", chunk_packet.position);
                    info!("    Entities: {:?}", chunk_packet.chunk_data.entities);

                    // Respond that we accepted the chunk
                    // Not actually true or how it works, but the server accepts it :)
                    channel.play.send(AcknowledgeChunksPacket { chunks_per_tick: 1.0 }).unwrap();
                }
                PlayClientboundPackets::SynchronizeTags(tags_packet) => {
                    info!("Play: SynchronizeTags");
                    for (key, data) in &tags_packet.tags {
                        info!("    \"{key}\":");
                        for (tag, data) in &data.data {
                            info!("        \"{tag}\": {data:?}");
                        }
                    }
                }
                PlayClientboundPackets::LightUpdate(_) => {
                    info!("Play: LightUpdate(LightUpdatePacket {{..}})");
                }
                PlayClientboundPackets::SynchronizeRecipes(_) => {
                    info!("Play: SynchronizeRecipes(SynchronizeRecipesPacket {{..}})");
                }
                PlayClientboundPackets::ChangeUnlockedRecipes(_) => {
                    info!("Play: ChangeUnlockedRecipes(ChangeUnlockedRecipesPacket {{..}})");
                }
                PlayClientboundPackets::CommandTree(_) => {
                    info!("Play: CommandTree(CommandTreePacket {{..}})");
                }
                PlayClientboundPackets::EntityAttributes(_) => {
                    info!("Play: EntityAttributes(EntityAttributesPacket {{..}})");
                }
                PlayClientboundPackets::AdvancementUpdate(_) => {
                    info!("Play: AdvancementUpdate(AdvancementUpdatePacket {{..}})");
                }
                _ => {
                    info!("Play: {packet:?}");
                }
            }
        }
    }
}
