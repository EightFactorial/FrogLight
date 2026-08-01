//! TODO
#![allow(clippy::std_instead_of_alloc, reason = "Example")]
#![allow(clippy::std_instead_of_core, reason = "Example")]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_net::TcpStream;
use bevy::{
    app::PluginGroupBuilder, diagnostic::DiagnosticsStore, ecs::resource::IsResource, math::DVec3,
    prelude::*, tasks::block_on,
};
use froglight::{
    bevy::plugins::{InstancePlugin, NetworkPlugin, PhysicsPlugin, TickMeasurementPlugin},
    modules::{
        api::api::Offline,
        network::{
            bevy::ClientDespawn,
            connection::FuturesLite,
            event::enums::{
                ClientboundConfigEvent, ClientboundLoginEvent, ClientboundPlayEvent,
                ServerboundConfigEvent, ServerboundHandshakeEvent, ServerboundLoginEvent,
                ServerboundPlayEvent,
            },
        },
        packet::common::{
            client_information::ClientInformation,
            handshake::{ConnectionIntent, HandshakeContent},
            login::LoginHelloContent,
            registry::RegistryDataEntry,
        },
    },
    prelude::*,
};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> AppExit {
    App::new()
        .add_plugins(default_plugins())
        .add_plugins(FroglightPlugins)
        .add_plugins(BotPlugin)
        .run()
}

/// Set a custom `LogPlugin` that doesn't escape ANSI :rolling_eyes:
fn default_plugins() -> PluginGroupBuilder {
    use bevy::log::LogPlugin;
    use tracing_subscriber::fmt::Layer;

    DefaultPlugins.set(LogPlugin {
        fmt_layer: |_| Some(Box::new(Layer::default().with_ansi_sanitization(false))),
        ..LogPlugin::default()
    })
}

// -------------------------------------------------------------------------------------------------

/// A custom [`Plugin`] for FrogBot.
struct BotPlugin;

const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
const USERNAME: &str = "FrogBot";
type Version = V26_1;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        // Add systems for creating the bot and handling messages.
        app.add_systems(Startup, BotPlugin::create_bot)
            .add_systems(PreUpdate, NetworkPlugin::clientbound_messages)
            .add_systems(
                Update,
                (BotPlugin::message_handler, BotPlugin::tick_runtime).ambiguous_with_all(),
            )
            .add_systems(
                PostUpdate,
                (
                    InstancePlugin::apply_blockedits,
                    PhysicsPlugin::update_collisions,
                    PhysicsPlugin::update_prev_components,
                    (NetworkPlugin::serverbound_messages, NetworkPlugin::poll_connections).chain(),
                )
                    .ambiguous_with_all(),
            );
    }
}

impl BotPlugin {
    /// Connect to the server and spawn the bot entity.
    ///
    /// Run once during [`Startup`].
    fn create_bot(world: &mut World) {
        // Connect to the server.
        info!("Connecting to {ADDRESS}...");
        let stream = match block_on(TcpStream::connect(ADDRESS)) {
            Ok(stream) => stream,
            Err(err) => {
                error!("Failed to connect to server: {err}");
                world.write_message(AppExit::error());
                return;
            }
        };

        // Prepare the connection and player profile.
        let profile = PlayerProfile::new_offline(Username::new_from(USERNAME));
        let connection = ClientConnection::new::<Version, FuturesLite, TcpStream>(
            stream,
            cfg!(debug_assertions),
        );

        info!(
            "Attempting to login as \"{}\" ({})...",
            profile.username(),
            profile.uuid().as_hyphenated()
        );

        // Add the profile to the Offline API.
        let api = ClientApi::offline();
        Offline::insert_profile(profile.clone()).unwrap();

        // Prepare the handshake and login events.
        let handshake = HandshakeContent::new_socket::<Version>(ADDRESS, ConnectionIntent::Login);
        let login = LoginHelloContent::from_profile(&profile);

        // Spawn the bot entity and exit the app when it despawns.
        let mut entity = world.spawn((api, profile, connection));
        entity.observe(BotPlugin::exit_on_despawn);

        // Send the handshake and login events.
        let entity = entity.into_readonly();
        let conn = entity.get::<ClientConnection>().unwrap();
        conn.send(ServerboundHandshakeEvent::Handshake(handshake), entity).unwrap();
        conn.send(ServerboundLoginEvent::Hello(login), entity).unwrap();
    }

    /// An [`Observer`] that exits the app when the bot entity despawns.
    fn exit_on_despawn(_: On<ClientDespawn>, mut commands: Commands) {
        info!("Exiting...");
        commands.write_message(AppExit::Success);
    }

    /// Log the amount of time to took to run a tick.
    fn tick_runtime(diag: Res<DiagnosticsStore>, time: Res<Time>, mut timer: Local<Option<Timer>>) {
        let timer = timer.get_or_insert_with(|| Timer::from_seconds(10., TimerMode::Repeating));
        if timer.tick(time.delta()).just_finished()
            && let Some(diag) = diag.get(&TickMeasurementPlugin::TICK_RUNTIME)
            && let Some(average) = diag.average()
        {
            info!("Tick Runtime: {average:.3}{}", diag.suffix);
        }
    }

    /// Handle reading/writing all messages for the bot.
    ///
    /// Runs every frame during [`Update`].
    ///
    /// # Note
    ///
    /// All world operationsa are done through [`Commands`] to guarantee the
    /// correct order of operations. Otherwise, a position might be applied
    /// before an entity is spawned.
    #[allow(clippy::too_many_lines, reason = "Example")]
    #[allow(clippy::match_same_arms, reason = "Example")]
    #[allow(clippy::cast_possible_truncation, reason = "Ignored")]
    fn message_handler(
        bot: Query<EntityRef, (With<ClientConnection>, Without<IsResource>)>,
        mut reader: MessageReader<ClientboundMessage>,
        mut commands: Commands,
    ) {
        for message in reader.read() {
            let bot = match bot.get(message.source()) {
                Ok(bot) => bot,
                Err(err) => {
                    error!("Failed to get bot entity: {err}");
                    continue;
                }
            };

            match message.event() {
                // Handle gameplay events.
                ClientboundEventEnum::Play(event) => {
                    match event {
                        // ClientboundPlayEvent::ActionBarText() => todo!(),
                        ClientboundPlayEvent::AddEntity(data) => {
                            if let Some(bundle) =
                                Version::entities().get_entity_by_id(data.entity_type.into())
                            {
                                let ident = bundle.identifier();
                                let entity = commands.spawn((
                                    PartOfInstance::new(bot.id()),
                                    data.entity_id,
                                    data.entity_uuid,
                                    bundle,
                                    Position::new_xyz(
                                        data.position_x as f32,
                                        data.position_y as f32,
                                        data.position_z as f32,
                                    ),
                                    Velocity::new(data.velocity.as_vec3a()),
                                ));

                                info!(
                                    "Spawning Entity {} ({}) as \"{ident}\"",
                                    entity.id(),
                                    data.entity_id.0,
                                );
                            } else {
                                error!("Unknown Entity Type {:?}!", data.entity_type);
                            }
                        }
                        // ClientboundPlayEvent::Animate() => todo!(),
                        // ClientboundPlayEvent::AwardStats() => todo!(),
                        // ClientboundPlayEvent::BlockChangedAck() => todo!(),
                        // ClientboundPlayEvent::BlockDestruction() => todo!(),
                        // ClientboundPlayEvent::BlockEntityData() => todo!(),
                        // ClientboundPlayEvent::BlockEvent() => todo!(),
                        ClientboundPlayEvent::BlockUpdate(blockpos, block_id) => {
                            let blockpos = *blockpos;
                            let block_id = *block_id;

                            commands.entity(bot.id()).queue(move |mut entity: EntityWorldMut<'_>| {
                            let Some(instance) = entity.get::<SessionInstance>() else {
                                error!("Received BlockUpdate but bot doesn't have a SessionInstance!");
                                return;
                            };

                            let Some(block) = instance.version_blocks().get_block_by_state(block_id) else {
                                error!("Received BlockUpdate with unknown BlockState \"{}\"!", block_id.into_inner());
                                return;
                            };

                            debug!(
                                "Received BlockUpdate \"{}\" at {blockpos}: {:?}",
                                block.identifier(),
                                block.get_attributes().collect::<Vec<_>>()
                            );

                            let Some(mut queue) = entity.get_mut::<BlockEditQueue>() else {
                                error!(
                                    "Received BlockUpdate but bot doesn't have a BlockEditQueue!"
                                );
                                return;
                            };

                            queue.push(blockpos, block);
                        });
                        }
                        // ClientboundPlayEvent::BossEvent() => todo!(),
                        ClientboundPlayEvent::BundleDelimiter => {}
                        // ClientboundPlayEvent::ChangeDifficulty() => todo!(),
                        // ClientboundPlayEvent::ChatSuggestions() => todo!(),
                        ClientboundPlayEvent::ChunkBatchFinished(size) => {
                            debug!("Received ChunkBatchFinished: {size} chunks");

                            commands.entity(bot.id()).queue(|mut entity: EntityWorldMut<'_>| {
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                    ServerboundMessage::new(
                                        entity_id,
                                        ServerboundPlayEvent::ChunkBatchReceived(16.0),
                                    ),
                                );
                            });
                        }
                        ClientboundPlayEvent::ChunkBatchStart => {
                            debug!("Received ChunkBatchStart");
                        }
                        // ClientboundPlayEvent::ChunkBiomes() => todo!(),
                        // ClientboundPlayEvent::ChunkCacheCenter() => todo!(),
                        // ClientboundPlayEvent::ChunkCacheRadius() => todo!(),
                        ClientboundPlayEvent::ChunkSectionUpdate() => {}
                        ClientboundPlayEvent::ChunkWithLight(chunkpos, chunk_data, _) => {
                            let chunkpos = *chunkpos;
                            let chunk_data = chunk_data.clone();

                            commands.entity(bot.id()).queue(move |mut entity: EntityWorldMut<'_>| {
                                let bot_id = entity.id();
                                let Some(instance) = entity.get::<SessionInstance>() else {
                                    error!("Received ChunkWithLight but bot doesn't have a SessionInstance!");
                                    return
                                };

                                let chunk_id = instance.query_chunk(&chunkpos);
                                let chunk = match chunk_data.try_parse::<Version>(
                                    instance.height_max(),
                                    instance.height_min(),
                                ) {
                                    Ok(chunk) => chunk,
                                    Err(err) => {
                                        error!("Failed to parse Chunk: {err:?}");
                                        return;
                                    }
                                };


                                if let Some(chunk_id) = chunk_id {
                                    // Store the chunk in the existing chunk entity.
                                    entity.world_scope(|world| {
                                        if let Some(mut shared) =  world.get_mut::<SharedChunk>(chunk_id) {
                                            shared.store(chunk);

                                            info!(
                                                "Updating Chunk Entity {chunk_id} ({}, {})",
                                                chunkpos.x(),
                                                chunkpos.z()
                                            );
                                        }
                                    });

                                    // Remove any queued block edits for the new chunk.
                                    if let Some(mut queue) = entity.get_mut::<BlockEditQueue>() {
                                        queue.remove(&chunkpos);
                                    }
                                } else {
                                    // Spawn a new entity containing the chunk.
                                    let chunk = entity.into_world_mut().spawn((
                                        PartOfInstance::new(bot_id),
                                        SharedChunk::new(chunk),
                                        chunkpos,
                                    ));

                                    info!(
                                        "Spawning Chunk Entity {} ({}, {})",
                                        chunk.id(),
                                        chunkpos.x(),
                                        chunkpos.z()
                                    );
                                }
                            });
                        }
                        // ClientboundPlayEvent::ClearDialog => todo!(),
                        // ClientboundPlayEvent::ClearTitles() => todo!(),
                        // ClientboundPlayEvent::CommandSuggestions() => todo!(),
                        // ClientboundPlayEvent::Commands() => todo!(),
                        // ClientboundPlayEvent::ContainerClose() => todo!(),
                        // ClientboundPlayEvent::ContainerContent() => todo!(),
                        // ClientboundPlayEvent::ContainerData() => todo!(),
                        // ClientboundPlayEvent::ContainerSlot() => todo!(),
                        // ClientboundPlayEvent::CookieRequest() => todo!(),
                        // ClientboundPlayEvent::Cooldown() => todo!(),
                        ClientboundPlayEvent::CustomPayload(identifier, payload) => {
                            info!("Received CustomPayload \"{identifier}\": {payload:?}");
                        }
                        // ClientboundPlayEvent::CustomReportDetails() => todo!(),
                        // ClientboundPlayEvent::DamageEvent() => todo!(),
                        // ClientboundPlayEvent::DebugBlock() => todo!(),
                        // ClientboundPlayEvent::DebugChunk() => todo!(),
                        // ClientboundPlayEvent::DebugEntity() => todo!(),
                        // ClientboundPlayEvent::DebugEvent() => todo!(),
                        // ClientboundPlayEvent::DebugSample() => todo!(),
                        // ClientboundPlayEvent::DeleteChat() => todo!(),
                        ClientboundPlayEvent::Disconnect(reason) => {
                            info!("Disconnected from server: {reason:?}");
                            commands.write_message(AppExit::Success);
                        }
                        // ClientboundPlayEvent::DisguisedChat() => todo!(),
                        // ClientboundPlayEvent::DiskSpaceWarning() => todo!(),
                        ClientboundPlayEvent::EntityEvent() => {}
                        ClientboundPlayEvent::EntityPosition(entity_id, data, _on_ground) => {
                            let entity_id = *entity_id;
                            let data = *data;
                            // let _on_ground = *on_ground;

                            commands.entity(bot.id()).queue(move |entity: EntityWorldMut<'_>| {
                            let Some(instance) = entity.get::<SessionInstance>() else {
                                error!("Received EntityPosition but bot doesn't have a SessionInstance!");
                                return
                            };
                            let Some(target) = instance.query_id(&entity_id) else {
                                error!(
                                    "Received EntityPosition for unknown EntityId {}!",
                                    entity_id.0
                                );
                                return;
                            };

                            let Ok(mut entity) = entity.into_world_mut().get_entity_mut(target) else {
                                error!(
                                    "Received EntityPosition for Entity {target} that doesn't exist!"
                                );
                                return;
                            };
                            trace!("Moving Entity {target} ({})", entity_id.0);

                            if let Some(mut position) = entity.get_mut::<Position>() {
                                **position =
                                    DVec3::new(data.position_x, data.position_y, data.position_z)
                                        .as_vec3a();
                            }

                            if let Some(mut rotation) = entity.get_mut::<Rotation>() {
                                *rotation = Rotation::new(data.yaw, data.pitch);
                            }

                            if let Some(mut velocity) = entity.get_mut::<Velocity>() {
                                **velocity =
                                    DVec3::new(data.velocity_x, data.velocity_y, data.velocity_z)
                                        .as_vec3a();
                            }

                            // if let Some(mut ground) =
                            // entity.get_mut::<OnGround>() {
                            //     ground.0 = on_ground;
                            // }
                        });
                        }
                        // ClientboundPlayEvent::Explode() => todo!(),
                        ClientboundPlayEvent::ForgetChunk(chunkpos) => {
                            let chunkpos = *chunkpos;

                            commands.entity(bot.id()).queue(move |mut entity: EntityWorldMut<'_>| {
                                if let Some(mut queue) = entity.get_mut::<BlockEditQueue>()  {
                                    queue.remove(&chunkpos);
                                }

                                let Some(instance) = entity.get::<SessionInstance>() else {
                                    error!("Received ForgetChunk but bot doesn't have a SessionInstance!");
                                    return;
                                };

                                let Some(chunk_id) = instance.query_chunk(&chunkpos) else {
                                    warn!(
                                        "Received ForgetChunk for unknown Chunk Position ({}, {})!",
                                        chunkpos.x(),
                                        chunkpos.z()
                                    );
                                    return;
                                };

                                let Ok(world) = entity.into_world_mut().get_entity_mut(chunk_id) else {
                                    error!(
                                        "Received ForgetChunk for Chunk Entity {chunk_id} that doesn't exist!"
                                    );
                                    return;
                                };

                                info!(
                                    "Despawning Chunk Entity {chunk_id} ({}, {})",
                                    chunkpos.x(),
                                    chunkpos.z()
                                );

                                world.despawn();
                            });
                        }
                        // ClientboundPlayEvent::GameEvent() => todo!(),
                        // ClientboundPlayEvent::GameRule() => todo!(),
                        // ClientboundPlayEvent::GameTestHighlight() => todo!(),
                        // ClientboundPlayEvent::GhostRecipe() => todo!(),
                        // ClientboundPlayEvent::HurtAnimation() => todo!(),
                        // ClientboundPlayEvent::InitializeBorder() => todo!(),
                        ClientboundPlayEvent::KeepAlive(id) => {
                            info!("Received KeepAlive: {id}");

                            let id = *id;
                            commands.entity(bot.id()).queue(
                                move |mut entity: EntityWorldMut<'_>| {
                                    let entity_id = entity.id();
                                    entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                        ServerboundMessage::new(
                                            entity_id,
                                            ServerboundPlayEvent::KeepAlive(id),
                                        ),
                                    );
                                },
                            );
                        }
                        // ClientboundPlayEvent::LevelEvent() => todo!(),
                        // ClientboundPlayEvent::LevelParticles() => todo!(),
                        ClientboundPlayEvent::LightUpdate(_chunkpos, _light_data) => {}
                        ClientboundPlayEvent::Login(login) => {
                            info!(
                                "Joining as Entity {} ({:?}) in \"{}\"!",
                                bot.id(),
                                login.player_id.0,
                                login.spawn_info.dimension
                            );
                            debug!("Login Info: {login:#?}");

                            // Get the "minecraft:dimension_type" registry
                            let registry = Version::registry().read();
                            let dimensions =
                                registry.get_nbt_by_identifier("minecraft:dimension_type");

                            // Get the dimension's "min_y" and "logical_height" values.
                            #[expect(clippy::cast_possible_wrap, reason = "Desired")]
                            let (height_max, height_min) = if let Some(dimensions) = dimensions
                                && let Some(dim) =
                                    dimensions.get_by_identifier(&login.spawn_info.dimension)
                                && let Some(min_y) =
                                    dim.get("min_y").and_then(IndexedValue::into_int)
                                && let Some(logical_height) =
                                    dim.get("logical_height").and_then(IndexedValue::into_int)
                            {
                                info!(
                                    "Dimension \"{}\" has min_y=\"{}\" and logical_height=\"{logical_height}\"!",
                                    login.spawn_info.dimension, min_y as i32
                                );

                                // Convert from logical to relative height.
                                let height = logical_height
                                    .checked_add_signed(min_y as i32)
                                    .expect("Dimension height overflowed?!");

                                (height, min_y as i32)
                            } else {
                                error!(
                                    "Failed to get dimension \"{}\" from registry, using default \"minecraft:overworld\" values!",
                                    login.spawn_info.dimension
                                );

                                (320, -64)
                            };

                            // Insert the bot's initial components.
                            let profile = bot.get::<PlayerProfile>().unwrap();
                            commands.entity(bot.id()).insert((
                                SessionInstance::new::<Version>(
                                    login.spawn_info.dimension.clone(),
                                    height_max,
                                    height_min,
                                ),
                                PartOfInstance::new(bot.id()),
                                BlockEditQueue::new(),
                                TickTimer::default(),
                                login.player_id,
                                EntityUuid::new(*profile.uuid()),
                                EntityBundle::new::<entity::Player, Version>(),
                                Position::ZERO,
                                Rotation::IDENTITY,
                                Velocity::ZERO,
                                Acceleration::ZERO,
                            ));
                        }
                        // ClientboundPlayEvent::MapItemData() => todo!(),
                        // ClientboundPlayEvent::MerchantOffers() => todo!(),
                        // ClientboundPlayEvent::MountScreen() => todo!(),
                        ClientboundPlayEvent::MoveEntityPos(data)
                        | ClientboundPlayEvent::MoveEntityPosRot(data)
                        | ClientboundPlayEvent::MoveEntityRot(data) => {
                            let data = *data;

                            commands.entity(bot.id()).queue(move |entity: EntityWorldMut<'_>| {
                            let Some(instance) = entity.get::<SessionInstance>() else { return };
                            let Some(target) = instance.query_id(&data.entity_id) else {
                                error!(
                                    "Received MoveEntity for unknown EntityId {}!",
                                    data.entity_id.0
                                );
                                return;
                            };

                            let Ok(mut entity) = entity.into_world_mut().get_entity_mut(target)
                            else {
                                error!(
                                    "Received MoveEntity for Entity {target} that doesn't exist!"
                                );
                                return;
                            };
                            trace!("Moving Entity {target} ({})", data.entity_id.0);

                            if let Some(delta) = data.delta
                                && let Some(mut position) = entity.get_mut::<Position>()
                            {
                                *position =
                                    Position::new_vec3(delta.add_to_vec(position.to_vec3()));
                            }

                            if let Some(angle) = data.rotation
                                && let Some(mut rotation) = entity.get_mut::<Rotation>()
                            {
                                (*rotation.yaw_mut(), *rotation.pitch_mut()) = angle.into_degrees();
                            }

                            // if let Some(mut on_ground) =
                            // entity.get_mut::<OnGround>() {
                            //     on_ground.0 = data.on_ground;
                            // }
                        });
                        }
                        // ClientboundPlayEvent::MoveMinecartTrack() => todo!(),
                        // ClientboundPlayEvent::MoveVehicle() => todo!(),
                        // ClientboundPlayEvent::OpenBook() => todo!(),
                        // ClientboundPlayEvent::OpenScreen() => todo!(),
                        // ClientboundPlayEvent::OpenSignEditor() => todo!(),
                        ClientboundPlayEvent::Ping(id) => {
                            info!("Received Ping: {id}");

                            let id = *id;
                            commands.entity(bot.id()).queue(
                                move |mut entity: EntityWorldMut<'_>| {
                                    let entity_id = entity.id();
                                    entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                        ServerboundMessage::new(
                                            entity_id,
                                            ServerboundPlayEvent::Pong(id),
                                        ),
                                    );
                                },
                            );
                        }
                        // ClientboundPlayEvent::PlayerAbilities() => todo!(),
                        // ClientboundPlayEvent::PlayerChat() => todo!(),
                        // ClientboundPlayEvent::PlayerCombatEnd() => todo!(),
                        // ClientboundPlayEvent::PlayerCombatEnter() => todo!(),
                        // ClientboundPlayEvent::PlayerCombatKill() => todo!(),
                        // ClientboundPlayEvent::PlayerInfoRemove() => todo!(),
                        // ClientboundPlayEvent::PlayerInfoUpdate() => todo!(),
                        // ClientboundPlayEvent::PlayerLookAt() => todo!(),
                        ClientboundPlayEvent::PlayerPosition(teleport, data, flags) => {
                            let teleport = *teleport;
                            let data = *data;
                            let flags = *flags;

                            commands.entity(bot.id()).queue(move |mut entity: EntityWorldMut<'_>| {
                                // Set the player's position/rotation/velocity.
                                if let Ok((mut position, mut rotation, mut velocity)) = entity.get_components_mut::<(
                                    &mut Position,
                                    &mut Rotation,
                                    &mut Velocity,
                                )>(
                                ) {
                                    data.apply_relative(&mut position, rotation.as_vec3a(), &mut velocity, &flags);
                                } else {
                                    error!(
                                        "Received TeleportEntity for Player without Position, Rotation, or Velocity!"
                                    );
                                }

                                // Tell the server we accepted the teleport.
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(ServerboundMessage::new(
                                    entity_id,
                                    ServerboundPlayEvent::AcceptTeleportation(teleport),
                                ));
                            });
                        }
                        // ClientboundPlayEvent::PlayerRotation() => todo!(),
                        ClientboundPlayEvent::Pong(id) => {
                            info!("Received Pong: {id}");
                        }
                        // ClientboundPlayEvent::ProjectilePower() => todo!(),
                        // ClientboundPlayEvent::RecipeBookAdd() => todo!(),
                        // ClientboundPlayEvent::RecipeBookRemove() => todo!(),
                        // ClientboundPlayEvent::RecipeBookSettings() => todo!(),
                        ClientboundPlayEvent::RemoveEntities(entities) => {
                            let removed = entities.clone();
                            let bot_id = bot.id();

                            commands.entity(bot.id()).queue(move |entity: EntityWorldMut<'_>| {
                                let (entities, mut commands) = entity.into_world_mut().entities_and_commands();

                                let Ok(bot) = entities.get(bot_id) else { return };
                                let Some(instance) = bot.get::<SessionInstance>() else {
                                    error!(
                                        "Received RemoveEntities but bot doesn't have a SessionInstance!"
                                    );
                                    return;
                                };

                                for entity_id in removed {
                                    if let Some(entity) = instance.query_id(&entity_id) {
                                        let Ok(entity_ref) = entities.get(entity) else { continue };
                                        let identifier = entity_ref.get::<EntityBundle>().map_or("<unknown>", |bundle| bundle.metadata().identifier());
                                        info!("Despawning Entity {entity} ({}) as \"{identifier}\"", entity_id.0);

                                        commands.entity(entity).despawn();
                                    } else {
                                        error!("Attempted to despawn unknown EntityId {:?}!", entity_id.0);
                                    }
                                }
                            });
                        }
                        // ClientboundPlayEvent::RemoveMobEffect() => todo!(),
                        // ClientboundPlayEvent::ResetScore() => todo!(),
                        // ClientboundPlayEvent::ResourcePackPop() => todo!(),
                        // ClientboundPlayEvent::ResourcePackPush() => todo!(),
                        // ClientboundPlayEvent::Respawn() => todo!(),
                        ClientboundPlayEvent::RotateHead() => {}
                        // ClientboundPlayEvent::SelectAdvancementTab() => todo!(),
                        // ClientboundPlayEvent::ServerData() => todo!(),
                        // ClientboundPlayEvent::ServerLinks() => todo!(),
                        // ClientboundPlayEvent::SetBorderCenter() => todo!(),
                        // ClientboundPlayEvent::SetBorderLerpSize() => todo!(),
                        // ClientboundPlayEvent::SetBorderSize() => todo!(),
                        // ClientboundPlayEvent::SetBorderWarningDelay() => todo!(),
                        // ClientboundPlayEvent::SetBorderWarningDistance() => todo!(),
                        // ClientboundPlayEvent::SetCamera() => todo!(),
                        // ClientboundPlayEvent::SetCursorItem() => todo!(),
                        // ClientboundPlayEvent::SetDefaultSpawn() => todo!(),
                        // ClientboundPlayEvent::SetDisplayObjective() => todo!(),
                        ClientboundPlayEvent::SetEntityData(data) => {
                            debug!("Received SetEntityData for EntityId {}", data.entity_id().0);

                            let id = data.entity_id();
                            let Ok(dataset) = data.parse() else {
                                error!("Failed to parse EntityData for EntityId {}!", id.0);
                                continue;
                            };

                            commands.entity(bot.id()).queue(move |entity: EntityWorldMut<'_>| {
                            let Some(instance) = entity.get::<SessionInstance>() else { return };

                            if let Some(target) = instance.query_id(&id) {
                                let Ok(mut entity) = entity.into_world_mut().get_entity_mut(target) else {
                                    error!(
                                        "Received SetEntityData for Entity {target} that doesn't exist!"
                                    );
                                    return;
                                };

                                if let Some(bundle) = entity.get::<EntityBundle>().cloned()
                                    && let Ok(bundle) = bundle.with_dataset(dataset)
                                {
                                    trace!("Adding to Entity {} ({}):", entity.id(), id.0);
                                    bundle.inspect_reflect(|ty| {
                                        trace!("    - {}", ty.reflect_short_type_path());
                                    });

                                    entity.insert(bundle);
                                } else {
                                    error!(
                                        "Received SetEntityData for Entity {target} without EntityBundle!"
                                    );
                                }
                            } else {
                                error!("Received SetEntityData for unknown EntityId {}!", id.0);
                            }
                        });
                        }
                        // ClientboundPlayEvent::SetEntityLink() => todo!(),
                        ClientboundPlayEvent::SetEntityMotion(id, delta) => {
                            let id = *id;
                            let delta = *delta;

                            commands.entity(bot.id()).queue(move |entity: EntityWorldMut<'_>| {
                            let Some(instance) = entity.get::<SessionInstance>() else { return };

                            if let Some(target) = instance.query_id(&id) {
                                if let Some(mut velocity) =
                                    entity.into_world_mut().get_mut::<Velocity>(target)
                                {
                                    **velocity += delta.as_vec3a();
                                } else {
                                    error!(
                                        "Received SetEntityMotion for Entity {target} without Velocity!"
                                    );
                                }
                            } else {
                                error!("Received SetEntityMotion for unknown EntityId {}!", id.0);
                            }
                        });
                        }
                        // ClientboundPlayEvent::SetEquipment() => todo!(),
                        // ClientboundPlayEvent::SetExperience() => todo!(),
                        // ClientboundPlayEvent::SetHealth() => todo!(),
                        // ClientboundPlayEvent::SetHeldSlot() => todo!(),
                        // ClientboundPlayEvent::SetObjective() => todo!(),
                        // ClientboundPlayEvent::SetPassengers() => todo!(),
                        // ClientboundPlayEvent::SetPlayerInventory() => todo!(),
                        // ClientboundPlayEvent::SetPlayerTeam() => todo!(),
                        // ClientboundPlayEvent::SetScore() => todo!(),
                        // ClientboundPlayEvent::SetSimulationDistance() => todo!(),
                        // ClientboundPlayEvent::SetSubtitleText() => todo!(),
                        ClientboundPlayEvent::SetTime() => {}
                        // ClientboundPlayEvent::SetTitleAnimation() => todo!(),
                        // ClientboundPlayEvent::SetTitleText() => todo!(),
                        // ClientboundPlayEvent::ShowDialog() => todo!(),
                        // ClientboundPlayEvent::Sound() => todo!(),
                        // ClientboundPlayEvent::SoundEntity() => todo!(),
                        ClientboundPlayEvent::StartConfiguration => {
                            info!("Reconfiguring...");
                            let mut commands = commands.entity(bot.id());

                            commands.remove::<SessionInstance>();
                        }
                        // ClientboundPlayEvent::StopSound() => todo!(),
                        // ClientboundPlayEvent::StoreCookie() => todo!(),
                        // ClientboundPlayEvent::SystemChat() => todo!(),
                        // ClientboundPlayEvent::TabList() => todo!(),
                        // ClientboundPlayEvent::TagQuery() => todo!(),
                        // ClientboundPlayEvent::TakeItemEntity() => todo!(),
                        ClientboundPlayEvent::TeleportEntity(id, data, flags, _on_ground) => {
                            let id = *id;
                            let data = *data;
                            let flags = *flags;
                            // let on_ground = *on_ground;

                            commands.entity(bot.id()).queue(move |entity: EntityWorldMut<'_>| {
                                let Some(instance) = entity.get::<SessionInstance>() else { return };
                                let Some(target) = instance.query_id(&id) else {
                                    error!("Received SetEntityMotion for unknown EntityId {}!", id.0);
                                    return;
                                };

                                let world = entity.into_world_mut();
                                let Ok(mut entity) = world.get_entity_mut(target) else {
                                    error!(
                                        "Received TeleportEntity for Entity {target} that doesn't exist!"
                                    );
                                    return;
                                };

                                if let Ok((mut position, mut rotation, mut velocity)) = entity.get_components_mut::<(
                                    &mut Position,
                                    &mut Rotation,
                                    &mut Velocity,
                                    // &mut OnGround,
                                )>(
                                ) {
                                    data.apply_relative(&mut position, rotation.as_vec3a(), &mut velocity, &flags);
                                    // ground.0 = on_ground;
                                } else {
                                    error!(
                                        "Received TeleportEntity for Entity {target} without Transform, Velocity, or OnGround!"
                                    );
                                }
                            });
                        }
                        // ClientboundPlayEvent::TestBlockStatus() => todo!(),
                        // ClientboundPlayEvent::TickingState() => todo!(),
                        // ClientboundPlayEvent::TickingStep() => todo!(),
                        // ClientboundPlayEvent::Transfer() => todo!(),
                        // ClientboundPlayEvent::UpdateAdvancements() => todo!(),
                        // ClientboundPlayEvent::UpdateAttributes() => todo!(),
                        // ClientboundPlayEvent::UpdateMobEffect() => todo!(),
                        // ClientboundPlayEvent::UpdateRecipes() => todo!(),
                        // ClientboundPlayEvent::UpdateTags() => todo!(),
                        // ClientboundPlayEvent::Waypoint() => todo!(),
                        other => debug!("Unhandled Event: {other:?}"),
                    }
                }

                // Handle configuration events.
                ClientboundEventEnum::Config(event) => match event {
                    ClientboundConfigEvent::ClearDialog => {
                        info!("Received ClearDialog");
                    }
                    ClientboundConfigEvent::CodeOfConduct() => {
                        info!("Received Code of Conduct: <placeholder>");
                        warn!("Accepting Code of Conduct...");

                        commands.entity(bot.entity()).queue(|mut entity: EntityWorldMut<'_>| {
                            let entity_id = entity.id();
                            entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                ServerboundMessage::new(
                                    entity_id,
                                    ServerboundConfigEvent::AcceptCodeOfConduct,
                                ),
                            );
                        });
                    }
                    ClientboundConfigEvent::CookieRequest(identifier) => {
                        info!("Received CookieRequest: \"{identifier}\"");

                        let identifier = identifier.clone();
                        commands.entity(bot.entity()).queue(|mut entity: EntityWorldMut<'_>| {
                            let entity_id = entity.id();
                            entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                ServerboundMessage::new(
                                    entity_id,
                                    ServerboundConfigEvent::CookieResponse(identifier, None),
                                ),
                            );
                        });
                    }
                    ClientboundConfigEvent::CustomPayload(identifier, _) => {
                        info!("Received CustomPayload: \"{identifier}\"");

                        // Use this as the trigger to send the client information packet
                        if identifier == "minecraft:brand" {
                            info!("Sending client information...");

                            commands.entity(bot.entity()).queue(
                                |mut entity: EntityWorldMut<'_>| {
                                    let entity_id = entity.id();
                                    entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                        ServerboundMessage::new(
                                            entity_id,
                                            ServerboundConfigEvent::ClientInformation(
                                                ClientInformation::default(),
                                            ),
                                        ),
                                    );
                                },
                            );
                        }
                    }
                    ClientboundConfigEvent::CustomReportDetails() => {
                        info!("Received CustomReportDetails: <placeholder>");
                    }
                    ClientboundConfigEvent::Disconnect(reason) => {
                        error!("Disconnected from server: {reason:?}");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundConfigEvent::EnabledFeatures() => {
                        info!("Received EnabledFeatures: <placeholder>");
                    }
                    ClientboundConfigEvent::FinishConfig => {
                        info!("Successfully configured!");
                        commands.entity(bot.entity()).queue(|mut entity: EntityWorldMut<'_>| {
                            let entity_id = entity.id();
                            entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                ServerboundMessage::new(
                                    entity_id,
                                    ServerboundConfigEvent::AcknowledgeConfig,
                                ),
                            );
                        });
                    }
                    ClientboundConfigEvent::KeepAlive(id) => {
                        info!("Received KeepAlive: {id}");

                        let id = *id;
                        commands.entity(bot.entity()).queue(
                            move |mut entity: EntityWorldMut<'_>| {
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                    ServerboundMessage::new(
                                        entity_id,
                                        ServerboundConfigEvent::KeepAlive(id),
                                    ),
                                );
                            },
                        );
                    }
                    ClientboundConfigEvent::KnownResourcePacks(known) => {
                        info!("Received KnownResourcePacks: {known:?}");
                        info!("Selecting no resource packs...");
                        commands.entity(bot.entity()).queue(|mut entity: EntityWorldMut<'_>| {
                            let entity_id = entity.id();
                            entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                ServerboundMessage::new(
                                    entity_id,
                                    ServerboundConfigEvent::ResourcePackResponse(Vec::new()),
                                ),
                            );
                        });
                    }
                    ClientboundConfigEvent::Ping(id) => {
                        info!("Received Ping: {id}");

                        let id = *id;
                        commands.entity(bot.entity()).queue(
                            move |mut entity: EntityWorldMut<'_>| {
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                    ServerboundMessage::new(
                                        entity_id,
                                        ServerboundConfigEvent::Pong(id),
                                    ),
                                );
                            },
                        );
                    }
                    ClientboundConfigEvent::RegistryData(identifier, entries) => {
                        info!("Received RegistryData: \"{identifier}\"");

                        // Write to the current registry.
                        let mut registry = Version::registry().write();
                        let metadata = registry.nbt_mut();

                        let storage = metadata.entry(identifier.clone()).or_default();
                        for RegistryDataEntry { identifier, nbt } in entries.clone() {
                            if let Some(nbt) = &nbt {
                                debug!(" - \"{identifier}\":");
                                for entry in nbt.as_compound() {
                                    debug!("   - \"{}\": <hidden>", entry.name().get());
                                }
                            } else {
                                debug!(" - \"{identifier}\": <empty>");
                            }

                            storage.insert(identifier, nbt.unwrap_or_default());
                        }
                    }
                    ClientboundConfigEvent::ResetChat => {
                        info!("Received ResetChat");
                    }
                    ClientboundConfigEvent::ResourcePackPop() => {
                        info!("Received ResourcePackPop: <placeholder>");
                    }
                    ClientboundConfigEvent::ResourcePackPush() => {
                        info!("Received ResourcePackPush: <placeholder>");
                    }
                    ClientboundConfigEvent::ServerLinks() => {
                        info!("Received ServerLinks: <placeholder>");
                    }
                    ClientboundConfigEvent::ShowDialog() => {
                        info!("Received ShowDialog: <placeholder>");
                    }
                    ClientboundConfigEvent::StoreCookie(identifier, payload) => {
                        info!("Received StoreCookie: \"{identifier}\": {payload:?}");
                    }
                    ClientboundConfigEvent::Transfer() => {
                        error!("Received Transfer: <placeholder>");
                        error!("Did you attempt to join a proxy?");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundConfigEvent::UpdateTags(tags) => {
                        // Write to the current registry.
                        let mut registry = Version::registry().write();
                        let metadata = registry.tags_mut();

                        // Update the metadata with the new tags.
                        for (identifier, tags) in &tags.0 {
                            info!("Received UpdateTags: \"{identifier}\"");
                            let storage = metadata.entry(identifier.clone()).or_default();

                            for tag in tags.clone() {
                                debug!(" - \"{}\"", tag.identifier);
                                storage.insert(tag.identifier, tag.values);
                            }
                        }
                    }
                    other => warn!("Unhandled Event: {other:?}"),
                },

                // Handle login events.
                ClientboundEventEnum::Login(event) => match event {
                    ClientboundLoginEvent::CookieRequest(identifier) => {
                        info!("Received CookieRequest: \"{identifier}\"");

                        let identifier = identifier.clone();
                        commands.entity(bot.entity()).queue(
                            move |mut entity: EntityWorldMut<'_>| {
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                    ServerboundMessage::new(
                                        entity_id,
                                        ServerboundLoginEvent::CookieResponse(identifier, None),
                                    ),
                                );
                            },
                        );
                    }
                    ClientboundLoginEvent::CustomPayload(id, identifier, _) => {
                        info!("Received CustomPayload: \"{identifier}\"");

                        let id = *id;
                        commands.entity(bot.entity()).queue(
                            move |mut entity: EntityWorldMut<'_>| {
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                    ServerboundMessage::new(
                                        entity_id,
                                        ServerboundLoginEvent::CustomPayload(id, None),
                                    ),
                                );
                            },
                        );
                    }
                    ClientboundLoginEvent::Disconnect(reason) => {
                        error!("Failed to connect to server: {reason}");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundLoginEvent::EncryptionRequest() => {
                        error!("Received encryption request!");
                        error!("Did you attempt to login to an online-mode server?");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundLoginEvent::LoginFinished(profile) => {
                        info!(
                            "Logged in as \"{}\" ({})!",
                            profile.username(),
                            profile.uuid().as_hyphenated()
                        );

                        if let Some(existing) = bot.get::<PlayerProfile>()
                            && (existing.username() != profile.username()
                                || existing.uuid() != profile.uuid())
                        {
                            warn!(
                                "Bot using \"{}\" ({}) was changed to \"{}\" ({})!",
                                existing.username(),
                                existing.uuid().as_hyphenated(),
                                profile.username(),
                                profile.uuid().as_hyphenated()
                            );
                        }

                        commands
                            .entity(bot.entity())
                            .insert((profile.username().clone(), profile.clone()))
                            .queue(|mut entity: EntityWorldMut<'_>| {
                                let entity_id = entity.id();
                                entity.resource_mut::<Messages<ServerboundMessage>>().write(
                                    ServerboundMessage::new(
                                        entity_id,
                                        ServerboundLoginEvent::AcknowledgeLogin,
                                    ),
                                );
                            });
                    }
                    other => warn!("Unhandled Event: {other:?}"),
                },

                // Can't receive a status event since the bot attempted to login.
                ClientboundEventEnum::Status(_) => unreachable!(),
            }
        }
    }
}
