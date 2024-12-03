//! Connect to a server and log the packets received.
//!
//! The connection will eventually be closed by the server
//! due to not responding to keep-alive packets.

use std::{io::Cursor, num::NonZeroU8, sync::Arc};

use bevy::prelude::*;
use froglight::{
    network::{
        connection::AccountInformation,
        versions::v1_21_0::{
            configuration::{
                ConfigurationClientboundPackets, CookieResponsePacket, CustomPayloadC2SPacket,
                ReadyC2SPacket, SelectKnownPacksC2SPacket,
            },
            login::{EnterConfigurationPacket, LoginClientboundPackets, LoginQueryResponsePacket},
            play::{
                AcknowledgeChunksPacket, PlayClientboundPackets, ResourcePackStatusPacket,
                TeleportConfirmPacket,
            },
            V1_21_0,
        },
    },
    prelude::{
        registry::{BlockRegistry, ItemRegistry},
        *,
    },
    protocol::FrogWrite,
    DefaultPlugins,
};

/// The server address to connect to.
const SERVER_ADDRESS: &str = "localhost";

/// The account information to use when connecting to the server.
const ACCOUNT: AccountInformation = AccountInformation::const_new("froglight", Uuid::nil());

fn main() -> AppExit {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // I recommend polling for packets during the `First` stage.
    // This way you can handle packets before any other systems run.
    app.add_systems(First, print_packets.run_if(any_with_component::<ConnectionChannel<V1_21_0>>));

    app.add_systems(
        Update,
        (
            // Create a connection to the server once
            create_connection.run_if(run_once),
            // Exit if a network error occurs
            exit_on_error
                .run_if(any_component_removed::<PolledTask>.or(on_event::<ConnectionErrorEvent>)),
        )
            .chain(),
    );

    app.run()
}

/// Create a connection to the server.
///
/// **Do not** drop the task or the channel before it is done.
/// If either are dropped the connection will immediately close.
///
/// If you don't want to manage entity lifetimes and polling tasks,
/// you can use the [`PolledTask`] [`Component`], which will
/// despawn the entity automatically when the task is done.
fn create_connection(mut commands: Commands, resolver: Res<Resolver>) {
    info!("Connecting to \"{SERVER_ADDRESS}\"...");
    let (channel, task) = V1_21_0::connect(SERVER_ADDRESS, ACCOUNT, &resolver);
    commands.spawn((channel, task, PolledTask));
}

/// Log and exit if a network error occurs.
fn exit_on_error(mut events: EventReader<ConnectionErrorEvent>, mut exit: EventWriter<AppExit>) {
    if let Some(error) = events.read().next() {
        error!("Error: {}", error.error);
    }

    error!("Exiting...");
    exit.send(AppExit::Error(NonZeroU8::new(1).unwrap()));
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
        while let Ok(received) = channel.try_recv() {
            match received {
                ChannelRecvPacket::Handshake(packet) => {
                    error!("Handshake: {packet:?}");
                }
                ChannelRecvPacket::Status(packet) => {
                    error!("Status: {packet:?}");
                }
                ChannelRecvPacket::Login(packet) => match Arc::unwrap_or_clone(packet) {
                    LoginClientboundPackets::LoginSuccess(profile_packet) => {
                        // Log the profile information
                        info!("Login: Profile");
                        info!(
                            "    Username: \"{}\", Uuid: \"{}\"",
                            profile_packet.profile.name, profile_packet.profile.uuid
                        );

                        // Send an acknowledgement packet
                        channel.send_login(EnterConfigurationPacket);

                        // Insert the profile into the ECS
                        commands.entity(entity).insert(profile_packet.profile);
                    }
                    LoginClientboundPackets::LoginQueryRequest(query_packet) => {
                        info!(
                            "Login: Query \"{}\" -> {:?}",
                            query_packet.identifier, query_packet.payload
                        );

                        // Respond that we don't understand the query
                        channel.send_login(LoginQueryResponsePacket {
                            id: query_packet.id,
                            identifier: query_packet.identifier,
                            payload: None,
                        });
                    }
                    LoginClientboundPackets::CookieRequest(cookie_packet) => {
                        info!("Login: Cookie \"{}\"", cookie_packet.cookie);

                        // Respond that we don't have that cookie
                        channel.send_login(CookieResponsePacket {
                            cookie: cookie_packet.cookie,
                            payload: None,
                        });
                    }
                    LoginClientboundPackets::LoginDisconnect(disconnect_packet) => {
                        info!("Login: Disconnect \"{:?}\"", disconnect_packet.reason);

                        // Disconnect from the server
                        commands.entity(entity).despawn_recursive();
                    }
                    other => {
                        info!("Login: {other:?}");
                    }
                },
                ChannelRecvPacket::Config(packet) => match Arc::unwrap_or_clone(packet) {
                    ConfigurationClientboundPackets::SelectKnownPacks(resourcepack_packet) => {
                        info!("Config: ResourcePacks");

                        // Log the received packs
                        info!("    Received:");
                        for p in &resourcepack_packet.resourcepacks {
                            info!("        {p:?}");
                        }

                        // Filter out packs that aren't in the default namespace
                        let resourcepacks: Vec<_> = resourcepack_packet
                            .resourcepacks
                            .into_iter()
                            .filter(|p| p.namespace == "minecraft")
                            .collect();

                        // Log the response
                        info!("    Responded:");
                        for p in &resourcepacks {
                            info!("        {p:?}");
                        }

                        // Respond that we only know about packs in the default namespace
                        //
                        // If you respond with no packs, the server will send all registry data
                        channel.send_configuration(SelectKnownPacksC2SPacket { resourcepacks });
                    }
                    ConfigurationClientboundPackets::CustomPayload(payload_packet) => {
                        info!(
                            "Config: Payload \"{}\" -> {:?}",
                            payload_packet.identifier, payload_packet.payload
                        );

                        // Create an empty payload
                        let mut payload = UnsizedBuffer::new();

                        // If the payload is a brand request, respond with "froglight"
                        if payload_packet.identifier.as_str() == "minecraft:brand" {
                            info!("    Response: \"froglight\"");
                            "froglight".fg_write(&mut payload).unwrap();
                        } else {
                            info!("    Response: None");
                        }

                        // Send the response
                        channel.send_configuration(CustomPayloadC2SPacket {
                            identifier: payload_packet.identifier,
                            payload,
                        });
                    }
                    ConfigurationClientboundPackets::CookieRequest(cookie_packet) => {
                        info!("Config: Cookie \"{}\"", cookie_packet.cookie);

                        // Respond that we don't have that cookie
                        channel.send_configuration(CookieResponsePacket {
                            cookie: cookie_packet.cookie,
                            payload: None,
                        });
                    }
                    ConfigurationClientboundPackets::Ready(_) => {
                        // Send an acknowledgement packet
                        channel.send_configuration(ReadyC2SPacket);
                    }
                    ConfigurationClientboundPackets::SynchronizeTags(tags_packet) => {
                        info!("Config: SynchronizeTags");
                        for (key, data) in &tags_packet.tags {
                            match key.as_str() {
                                // Use `ItemRegistry` to convert the ids to actual items
                                "minecraft:item" | "minecraft:banner_pattern" => {
                                    info!("    \"{key}\":");
                                    for (tag, data) in &data.data {
                                        info!(
                                            "        \"{tag}\": {:?}",
                                            data.iter()
                                                .map(|id| {
                                                    RegistryId::<V1_21_0>::from_id(*id).unwrap()
                                                })
                                                .collect::<Vec<ItemRegistry>>()
                                        );
                                    }
                                }
                                // Use `BlockRegistry` to convert the ids to actual blocks
                                "minecraft:block" | "minecraft:point_of_interest_type" => {
                                    info!("    \"{key}\":");
                                    for (tag, data) in &data.data {
                                        info!(
                                            "        \"{tag}\": {:?}",
                                            data.iter()
                                                .map(|id| {
                                                    RegistryId::<V1_21_0>::from_id(*id).unwrap()
                                                })
                                                .collect::<Vec<BlockRegistry>>()
                                        );
                                    }
                                }
                                // Log the data as is
                                key => {
                                    info!("    \"{key}\":");
                                    for (tag, data) in &data.data {
                                        info!("        \"{tag}\": {data:?}");
                                    }
                                }
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
                    ConfigurationClientboundPackets::ResourcePackSend(resourcepack_packet) => {
                        info!("Config: ResourcePackSend");
                        info!("    UUID: \"{}\"", resourcepack_packet.uuid);
                        info!("    URL: {}", resourcepack_packet.url);
                        info!("    Hash: {}", resourcepack_packet.hash);
                        info!("    Required: {}", resourcepack_packet.required);

                        // Respond that we downloaded and successfully loaded the pack
                        channel.send_configuration(ResourcePackStatusPacket {
                            resourcepack: resourcepack_packet.uuid,
                            status: ResourcePackStatus::Accepted,
                        });
                        channel.send_configuration(ResourcePackStatusPacket {
                            resourcepack: resourcepack_packet.uuid,
                            status: ResourcePackStatus::SuccessfullyLoaded,
                        });
                    }
                    other => {
                        info!("Config: {other:?}");
                    }
                },
                ChannelRecvPacket::Play(packet) => match Arc::unwrap_or_clone(packet) {
                    PlayClientboundPackets::CookieRequest(cookie_packet) => {
                        info!("Play: Cookie \"{}\"", cookie_packet.cookie);

                        // Respond that we don't have that cookie
                        channel.send_play(CookieResponsePacket {
                            cookie: cookie_packet.cookie,
                            payload: None,
                        });
                    }
                    PlayClientboundPackets::CustomPayload(payload_packet) => {
                        info!(
                            "Play: Payload \"{}\" -> {:?}",
                            payload_packet.identifier, payload_packet.payload
                        );

                        // Respond saying we don't understand the payload
                        channel.send_play(CustomPayloadC2SPacket {
                            identifier: payload_packet.identifier,
                            payload: UnsizedBuffer::new(),
                        });
                    }
                    PlayClientboundPackets::PlayerPositionLook(position_packet) => {
                        info!("Play: PlayerPositionLook");
                        info!("    Position: {:?}", position_packet.position);
                        info!(
                            "    Yaw: {:?}, Pitch: {:?}",
                            position_packet.yaw, position_packet.pitch
                        );

                        // Respond that we accepted the position
                        channel.send_play(TeleportConfirmPacket {
                            teleport_id: position_packet.teleport_id,
                        });
                    }
                    PlayClientboundPackets::ChunkData(chunk_packet) => {
                        info!("Play: ChunkData");
                        info!("    Position: {:?}", chunk_packet.position);
                        info!("    Entities: {:?}", chunk_packet.chunk_data.entities);
                        // trace!("   Nbt: {:?}", chunk_packet.chunk_data.heightmaps);
                        // trace!("   Light: {:?}", chunk_packet.light_data);

                        let mut cursor = Cursor::new(chunk_packet.chunk_data.data.as_slice());
                        match Chunk::read_from(320, -64, &mut cursor) {
                            Err(err) => error!("    Error: {err}"),
                            Ok(chunk) => {
                                let mut buf = Vec::new();
                                chunk.fg_write(&mut buf).unwrap();

                                if chunk_packet.chunk_data.data.as_slice() == buf.as_slice() {
                                    info!("    Valid!");
                                } else {
                                    warn!("    Data Mismatch!");
                                }
                            }
                        }

                        // Respond that we accepted the chunk
                        // Not actually true or how it works, but the server accepts it :)
                        channel.send_play(AcknowledgeChunksPacket { chunks_per_tick: 1.0 });
                    }
                    PlayClientboundPackets::SynchronizeTags(tags_packet) => {
                        info!("Play: SynchronizeTags");
                        for (key, data) in &tags_packet.tags {
                            match key.as_str() {
                                // Use `ItemRegistry` to convert the ids to actual items
                                "minecraft:item" | "minecraft:banner_pattern" => {
                                    info!("    \"{key}\":");
                                    for (tag, data) in &data.data {
                                        info!(
                                            "        \"{tag}\": {:?}",
                                            data.iter()
                                                .map(|id| {
                                                    RegistryId::<V1_21_0>::from_id(*id).unwrap()
                                                })
                                                .collect::<Vec<ItemRegistry>>()
                                        );
                                    }
                                }
                                // Use `BlockRegistry` to convert the ids to actual blocks
                                "minecraft:block" | "minecraft:point_of_interest_type" => {
                                    info!("    \"{key}\":");
                                    for (tag, data) in &data.data {
                                        info!(
                                            "        \"{tag}\": {:?}",
                                            data.iter()
                                                .map(|id| {
                                                    RegistryId::<V1_21_0>::from_id(*id).unwrap()
                                                })
                                                .collect::<Vec<BlockRegistry>>()
                                        );
                                    }
                                }
                                // Log the data as is
                                key => {
                                    info!("    \"{key}\":");
                                    for (tag, data) in &data.data {
                                        info!("        \"{tag}\": {data:?}");
                                    }
                                }
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
                    PlayClientboundPackets::EntityMoveRelative(packet) => {
                        debug!("Play: {packet:?}");
                    }
                    PlayClientboundPackets::EntityRotateAndMoveRelative(packet) => {
                        debug!("Play: {packet:?}");
                    }
                    PlayClientboundPackets::EntityPosition(packet) => {
                        debug!("Play: {packet:?}");
                    }
                    PlayClientboundPackets::EntitySetHeadYaw(packet) => {
                        debug!("Play: {packet:?}");
                    }
                    PlayClientboundPackets::EntityVelocityUpdate(packet) => {
                        debug!("Play: {packet:?}");
                    }
                    PlayClientboundPackets::BundleDelimiter(..) => {}
                    other => {
                        info!("Play: {other:?}");
                    }
                },
            }
        }
    }
}
