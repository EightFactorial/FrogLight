//! Send a status request to "localhost" and prints the response.

use bevy::{app::AppExit, prelude::*};
use froglight_internal::{prelude::*, HeadlessPlugins};
use froglight_network::{
    network::{ConnectionTrait, NetworkErrorEvent, PolledTask},
    versions::v1_21_0::{
        configuration::{ConfigurationClientboundPackets, SelectKnownPacksC2SPacket},
        login::{LoginClientboundPackets, LoginQueryResponsePacket},
        play::{CookieResponsePacket, CustomPayloadC2SPacket},
        V1_21_0,
    },
};
use froglight_protocol::protocol::FrogWrite;

fn main() {
    let mut app = App::new();
    app.add_plugins(HeadlessPlugins);

    app.add_systems(
        Update,
        (
            create_connection.run_if(run_once()),
            print_packets.run_if(any_with_component::<ConnectionTask>),
            exit_on_error.run_if(on_event::<NetworkErrorEvent>()),
        )
            .chain(),
    );

    app.run();
}

const SERVER_ADDRESS: &str = "localhost";

fn create_connection(mut commands: Commands, resolver: Res<Resolver>) {
    let (task, channel) = V1_21_0::connect(SERVER_ADDRESS, &resolver);
    commands.spawn((task, channel, PolledTask));
}

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
        while let Ok(packet) = channel.config.recv.try_recv() {
            match packet.as_ref() {
                ConfigurationClientboundPackets::SelectKnownPacks(resourcepack_packet) => {
                    info!("Config: ResourcePacks");
                    for pack in &resourcepack_packet.resourcepacks {
                        info!("    \"{}:{}\" v{}", pack.namespace, pack.id, pack.version);
                    }

                    // Respond that we only know about packs in the default namespace
                    channel
                        .config
                        .send(SelectKnownPacksC2SPacket {
                            resourcepacks: resourcepack_packet
                                .iter()
                                .filter_map(|p| {
                                    if p.namespace == "minecraft" {
                                        Some(p.clone())
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        })
                        .unwrap();
                }
                ConfigurationClientboundPackets::CustomPayload(payload_packet) => {
                    info!(
                        "Config: Payload \"{}\" -> {:?}",
                        payload_packet.identifier, payload_packet.payload
                    );

                    // Create a response packet that says we don't understand the payload
                    let mut response = CustomPayloadC2SPacket {
                        identifier: payload_packet.identifier.clone(),
                        payload: None,
                    };

                    // If the payload is a brand request, respond with "froglight"
                    if payload_packet.identifier.as_str() == "minecraft:brand" {
                        info!("    Response: \"froglight\"");

                        // Set the payload to "froglight"
                        let mut payload = UnsizedBuffer::new();
                        "froglight".fg_write(&mut payload).unwrap();
                        response.payload = Some(payload);
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
                ConfigurationClientboundPackets::DynamicRegistries(_) => {
                    info!("Config: DynamicRegistries(DynamicRegistriesPacket {{..}})");
                }
                ConfigurationClientboundPackets::SynchronizeTags(_) => {
                    info!("Config: SynchronizeTags(SynchronizeTagsPacket {{..}})");
                }
                _ => {
                    info!("Config: {packet:?}");
                }
            }
        }

        while let Ok(packet) = channel.play.recv.try_recv() {
            info!("Play: {packet:?}");
        }
    }
}

/// Exit when a network error occurs.
///
/// The error will already be logged, so we just need to exit.
fn exit_on_error(mut events: EventReader<NetworkErrorEvent>, mut exit: EventWriter<AppExit>) {
    if let Some(error) = events.read().next() {
        error!("Error: {}", error.error);
        error!("Exiting...");
        exit.send(AppExit);
    }
}
