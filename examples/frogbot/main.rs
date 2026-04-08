//! TODO

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_net::TcpStream;
use bevy::{math::DVec3, prelude::*, tasks::block_on};
use froglight::{
    bevy::plugins::NetworkPlugin,
    modules::{
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
        },
    },
    prelude::*,
};

fn main() -> AppExit {
    App::new().add_plugins((DefaultPlugins, FroglightPlugins)).add_plugins(BotPlugin).run()
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
            .add_systems(Update, BotPlugin::message_handler)
            .add_systems(
                PostUpdate,
                (NetworkPlugin::serverbound_messages, NetworkPlugin::poll_connections).chain(),
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

        // Prepare the handshake and login events.
        let handshake = HandshakeContent::new_socket::<Version>(ADDRESS, ConnectionIntent::Login);
        let login = LoginHelloContent::new_from_profile(&profile);

        // Spawn the bot entity and exit the app when it despawns.
        let mut entity = world.spawn((connection, profile));
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

    /// Handle reading/writing all messages for the bot.
    ///
    /// Run every frame during [`Update`].
    fn message_handler(
        bot: Single<EntityRef, With<ClientConnection>>,
        mut reader: MessageReader<ClientboundMessage>,
        mut writer: MessageWriter<ServerboundMessage>,
        mut commands: Commands,
    ) {
        for message in reader.read() {
            // Warn if the message isn't for the bot entity.
            if message.source() != bot.id() {
                warn!(
                    "Received a message for a different entity: {} != {}",
                    message.source(),
                    bot.id()
                );
                continue;
            }

            match message.event() {
                // Handle gameplay events.
                ClientboundEventEnum::Play(event) => match event {
                    // ClientboundPlayEvent::ActionBarText() => todo!(),
                    ClientboundPlayEvent::AddEntity(data) => {
                        let mut entity = commands.spawn((
                            EntityOfInstance::new(bot.id()),
                            data.entity_id.0,
                            data.entity_uuid,
                            Transform::from_translation(Vec3::new(
                                data.position_x as f32,
                                data.position_y as f32,
                                data.position_z as f32,
                            )),
                            Velocity::new(data.velocity.as_vec3()),
                        ));

                        let entities = Version::entities().load();
                        if let Some(bundle) = entities.get_entity(data.entity_type.into()) {
                            entity.insert(bundle);
                        } else {
                            error!("Unknown Entity Type {:?}!", data.entity_type);
                        }

                        debug!("Spawning Entity {} ({})", entity.id(), data.entity_id.0.0);
                    }
                    // ClientboundPlayEvent::Animate() => todo!(),
                    // ClientboundPlayEvent::AwardStats() => todo!(),
                    // ClientboundPlayEvent::BlockChangedAck() => todo!(),
                    // ClientboundPlayEvent::BlockDestruction() => todo!(),
                    // ClientboundPlayEvent::BlockEntityData() => todo!(),
                    // ClientboundPlayEvent::BlockEvent() => todo!(),
                    // ClientboundPlayEvent::BlockUpdate() => todo!(),
                    // ClientboundPlayEvent::BossEvent() => todo!(),
                    ClientboundPlayEvent::BundleDelimiter => {}
                    // ClientboundPlayEvent::ChangeDifficulty() => todo!(),
                    // ClientboundPlayEvent::ChatSuggestions() => todo!(),
                    ClientboundPlayEvent::ChunkBatchFinished(size) => {
                        debug!("Received ChunkBatchFinished: {size} chunks");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundPlayEvent::ChunkBatchReceived(8.0),
                        ));
                    }
                    ClientboundPlayEvent::ChunkBatchStart => {
                        debug!("Received ChunkBatchStart");
                    }
                    // ClientboundPlayEvent::ChunkBiomes() => todo!(),
                    // ClientboundPlayEvent::ChunkCacheCenter() => todo!(),
                    // ClientboundPlayEvent::ChunkCacheRadius() => todo!(),
                    // ClientboundPlayEvent::ChunkSectionUpdate() => todo!(),
                    ClientboundPlayEvent::ChunkWithLight(chunk_pos, chunk_data, _) => {
                        let Some(instance) = bot.get::<WorldInstanceChunks>() else {
                            error!(
                                "Received ChunkWithLight but bot doesn't have a WorldInstanceChunks!"
                            );
                            continue;
                        };

                        match chunk_data.as_chunk::<Version>(Some((
                            instance.height_max(),
                            instance.height_min(),
                        ))) {
                            Ok(chunk) => {
                                let entity = commands.spawn((
                                    ChunkOfInstance::new(bot.id()),
                                    SharedChunk::new(chunk),
                                    *chunk_pos,
                                ));

                                debug!(
                                    "Spawning Chunk as Entity {} ({}, {})",
                                    entity.id(),
                                    chunk_pos.x(),
                                    chunk_pos.z()
                                );
                            }
                            Err(err) => error!("Failed to convert chunk data: {err:?}"),
                        }
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
                    // ClientboundPlayEvent::EntityEvent() => todo!(),
                    ClientboundPlayEvent::EntityPosition(entity_id, data, on_ground) => {
                        let entity_id = *entity_id;
                        let data = *data;
                        let on_ground = *on_ground;

                        commands.entity(bot.id()).queue(move |entity: EntityWorldMut| {
                            let Some(instance) = entity.get::<WorldInstance>() else { return };
                            let Some(target) = instance.get(&entity_id) else {
                                error!(
                                    "Received EntityPosition for unknown EntityId {}!",
                                    entity_id.0
                                );
                                return;
                            };

                            let mut entity = entity.into_world_mut().entity_mut(target);
                            trace!("Moving Entity {target} ({})", entity_id.0);

                            if let Some(mut transform) = entity.get_mut::<Transform>() {
                                transform.translation =
                                    DVec3::new(data.position_x, data.position_y, data.position_z)
                                        .as_vec3();
                                // TODO: Yaw/Pitch
                            }
                            if let Some(mut velocity) = entity.get_mut::<Velocity>() {
                                velocity.0 =
                                    DVec3::new(data.velocity_x, data.velocity_y, data.velocity_z)
                                        .as_vec3();
                            }

                            if let Some(mut ground) = entity.get_mut::<OnGround>() {
                                ground.0 = on_ground;
                            }
                        });
                    }
                    // ClientboundPlayEvent::Explode() => todo!(),
                    // ClientboundPlayEvent::ForgetChunk() => todo!(),
                    // ClientboundPlayEvent::GameEvent() => todo!(),
                    // ClientboundPlayEvent::GameRule() => todo!(),
                    // ClientboundPlayEvent::GameTestHighlight() => todo!(),
                    // ClientboundPlayEvent::GhostRecipe() => todo!(),
                    // ClientboundPlayEvent::HurtAnimation() => todo!(),
                    // ClientboundPlayEvent::InitializeBorder() => todo!(),
                    ClientboundPlayEvent::KeepAlive(id) => {
                        info!("Received KeepAlive: {id}");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundPlayEvent::KeepAlive(*id),
                        ));
                    }
                    // ClientboundPlayEvent::LevelEvent() => todo!(),
                    // ClientboundPlayEvent::LevelParticles() => todo!(),
                    // ClientboundPlayEvent::LightUpdate() => todo!(),
                    ClientboundPlayEvent::Login(login) => {
                        info!(
                            "Joining as Entity {} ({:?}) in \"{}\"!",
                            bot.id(),
                            login.player_id.0,
                            login.spawn_info.dimension
                        );

                        // TODO: Get the `height_max` and `height_min` from the server.
                        // Currently panics if the bot logs into other dimensions.

                        // Prepare the bot's `WorldInstance` for tracking entities
                        let mut commands = commands.entity(bot.id());
                        commands.insert((
                            WorldInstance::new(login.spawn_info.dimension.clone()),
                            WorldInstanceChunks::new(320, -64),
                            EntityOfInstance::new(bot.id()),
                            EntityBundle::new::<entity::Player, Version>(),
                        ));

                        // Add the bot's `EntityId` and `EntityUuid`
                        let profile = bot.get::<PlayerProfile>().unwrap();
                        commands.insert((login.player_id, EntityUuid::new(*profile.uuid())));
                    }
                    // ClientboundPlayEvent::MapItemData() => todo!(),
                    // ClientboundPlayEvent::MerchantOffers() => todo!(),
                    // ClientboundPlayEvent::MountScreen() => todo!(),
                    ClientboundPlayEvent::MoveEntityPos(data)
                    | ClientboundPlayEvent::MoveEntityPosRot(data)
                    | ClientboundPlayEvent::MoveEntityRot(data) => {
                        let data = *data;

                        commands.entity(bot.id()).queue(move |entity: EntityWorldMut| {
                            let Some(instance) = entity.get::<WorldInstance>() else { return };
                            let Some(target) = instance.get(&data.entity_id) else {
                                error!(
                                    "Received MoveEntity for unknown EntityId {}!",
                                    data.entity_id.0
                                );
                                return;
                            };

                            let mut entity = entity.into_world_mut().entity_mut(target);
                            trace!("Moving Entity {target} ({})", data.entity_id.0);

                            if let Some(delta) = data.delta
                                && let Some(mut transform) = entity.get_mut::<Transform>()
                            {
                                transform.translation = delta.add_to_vec(transform.translation);
                            }

                            if let Some((_y_rot, _x_rot)) = data.yaw_pitch
                                && let Some(_transform) = entity.get_mut::<Transform>()
                            {
                                // TODO: Yaw/Pitch
                            }

                            if let Some(mut on_ground) = entity.get_mut::<OnGround>() {
                                on_ground.0 = data.on_ground;
                            }
                        });
                    }
                    // ClientboundPlayEvent::MoveMinecartTrack() => todo!(),
                    // ClientboundPlayEvent::MoveVehicle() => todo!(),
                    // ClientboundPlayEvent::OpenBook() => todo!(),
                    // ClientboundPlayEvent::OpenScreen() => todo!(),
                    // ClientboundPlayEvent::OpenSignEditor() => todo!(),
                    ClientboundPlayEvent::Ping(id) => {
                        info!("Received Ping: {id}");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundPlayEvent::Pong(*id),
                        ));
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
                        // Tell the server we accepted the teleport.
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundPlayEvent::AcceptTeleportation(*teleport),
                        ));

                        // Set the player's position/rotation/velocity.
                        let data = *data;
                        let flags = *flags;
                        commands.entity(bot.id()).queue(move |mut entity: EntityWorldMut| {
                            if let Ok((mut transform, mut velocity)) = entity.get_components_mut::<(
                                &mut Transform,
                                &mut Velocity,
                            )>(
                            ) {
                                let Transform { translation, rotation, .. } = &mut *transform;
                                data.apply_relative(translation, rotation, &mut velocity, &flags);
                            } else {
                                error!(
                                    "Received TeleportEntity for Player without Transform, Velocity, or OnGround!"
                                );
                            }
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
                        let Some(instance) = bot.get::<WorldInstance>() else {
                            error!("Received RemoveEntities but bot doesn't have a WorldInstance!");
                            continue;
                        };

                        for entity_id in entities {
                            if let Some(entity) = instance.get(entity_id) {
                                debug!("Despawning Entity {entity} ({})", entity_id.0);

                                let mut entity = commands.entity(entity);

                                // Debug log the entity's components before despawning it.
                                entity.queue(|mut entity: EntityWorldMut| {
                                    let entity_id = entity.id();
                                    entity.world_scope(|world| {
                                        // Get the type registry
                                        let registry = world.resource::<AppTypeRegistry>().clone();
                                        let registry = registry.read();

                                        // Iterate over the entity's components
                                        trace!("Despawning Entity {entity_id} with:");
                                        for component in world.inspect_entity(entity_id).unwrap() {
                                            if let Some(info) =
                                                registry.get_type_info(component.type_id().unwrap())
                                            {
                                                // Log the component's type
                                                trace!(
                                                    "    - {}",
                                                    info.type_path_table().short_path()
                                                );
                                            }
                                        }
                                    })
                                });

                                entity.despawn();
                            } else {
                                error!("Attempted to despawn unknown EntityId {:?}!", entity_id.0);
                            }
                        }
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

                        commands.entity(bot.id()).queue(move |entity: EntityWorldMut| {
                            let Some(instance) = entity.get::<WorldInstance>() else { return };

                            if let Some(target) = instance.get(&id) {
                                let mut entity = entity.into_world_mut().entity_mut(target);

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

                        commands.entity(bot.id()).queue(move |entity: EntityWorldMut| {
                            let Some(instance) = entity.get::<WorldInstance>() else { return };

                            if let Some(target) = instance.get(&id) {
                                if let Some(mut velocity) =
                                    entity.into_world_mut().get_mut::<Velocity>(target)
                                {
                                    **velocity += delta.as_vec3();
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
                        commands.remove::<WorldInstance>().remove::<WorldInstanceChunks>();
                    }
                    // ClientboundPlayEvent::StopSound() => todo!(),
                    // ClientboundPlayEvent::StoreCookie() => todo!(),
                    // ClientboundPlayEvent::SystemChat() => todo!(),
                    // ClientboundPlayEvent::TabList() => todo!(),
                    // ClientboundPlayEvent::TagQuery() => todo!(),
                    // ClientboundPlayEvent::TakeItemEntity() => todo!(),
                    ClientboundPlayEvent::TeleportEntity(id, data, flags, on_ground) => {
                        let id = *id;
                        let data = *data;
                        let flags = *flags;
                        let on_ground = *on_ground;

                        commands.entity(bot.id()).queue(move |entity: EntityWorldMut| {
                            let Some(instance) = entity.get::<WorldInstance>() else { return };

                            if let Some(target) = instance.get(&id) {
                                let mut entity = entity.into_world_mut().entity_mut(target);

                                if let Ok((mut transform, mut velocity, mut ground)) = entity.get_components_mut::<(
                                    &mut Transform,
                                    &mut Velocity,
                                    &mut OnGround,
                                )>(
                                ) {
                                    let Transform { translation, rotation, .. } = &mut *transform;
                                    data.apply_relative(translation, rotation, &mut velocity, &flags);
                                    ground.0 = on_ground;
                                } else {
                                    error!(
                                        "Received TeleportEntity for Entity {target} without Transform, Velocity, or OnGround!"
                                    );
                                }
                            } else {
                                error!("Received SetEntityMotion for unknown EntityId {}!", id.0);
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
                },

                // Handle configuration events.
                ClientboundEventEnum::Config(event) => match event {
                    ClientboundConfigEvent::ClearDialog => {
                        info!("Received ClearDialog");
                    }
                    ClientboundConfigEvent::CodeOfConduct() => {
                        info!("Received Code of Conduct: <placeholder>");
                        warn!("Accepting Code of Conduct...");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::AcceptCodeOfConduct,
                        ));
                    }
                    ClientboundConfigEvent::CookieRequest(identifier) => {
                        info!("Received CookieRequest: \"{identifier}\"");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::CookieResponse(identifier.clone(), None),
                        ));
                    }
                    ClientboundConfigEvent::CustomPayload(identifier, _) => {
                        info!("Received CustomPayload: \"{identifier}\"");

                        // Use this as the trigger to send the client information packet
                        if identifier == "minecraft:brand" {
                            info!("Sending client information...");
                            writer.write(ServerboundMessage::new(
                                bot.id(),
                                ServerboundConfigEvent::ClientInformation(
                                    ClientInformation::default(),
                                ),
                            ));
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
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::AcknowledgeConfig,
                        ));
                    }
                    ClientboundConfigEvent::KeepAlive(id) => {
                        info!("Received KeepAlive: {id}");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::KeepAlive(*id),
                        ));
                    }
                    ClientboundConfigEvent::KnownResourcePacks(known) => {
                        info!("Received KnownResourcePacks: {known:?}");
                        info!("Selecting no resource packs...");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::ResourcePackResponse(Vec::new()),
                        ));
                    }
                    ClientboundConfigEvent::Ping(id) => {
                        info!("Received Ping: {id}");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::Pong(*id),
                        ));
                    }
                    ClientboundConfigEvent::RegistryData(identifier, _) => {
                        info!("Received RegistryData: \"{identifier}\"");
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
                        let mut registry = Version::registry().write();

                        for (identifier, tags) in &tags.0 {
                            info!("Received UpdateTags: \"{identifier}\"");
                            let storage = registry.get_mut_or_default(identifier.clone());

                            for tag in tags.clone() {
                                debug!(" - \"{}\"", tag.identifier);
                                storage.get_mut_or_default(tag.identifier).set_values(tag.values);
                            }
                        }

                        // As an example, trace log the "minecraft:slabs" tag.
                        let blocks = Version::blocks().load();
                        let Some(block_reg) = registry.get("minecraft:block") else { continue };
                        let Some(slabs_val) = block_reg.get_by_name("minecraft:slabs") else {
                            continue;
                        };

                        trace!("Example UpdateTags: \"minecraft:block\" -> \"minecraft:slabs\"");
                        for val in slabs_val.values() {
                            if let Some(meta) =
                                u32::try_from(*val).ok().and_then(|v| blocks.get_block_by_id(v))
                            {
                                trace!(" - \"{}\"", meta.identifier());
                            } else {
                                error!("Failed to get metadata for block with ID {val}!");
                            }
                        }
                    }
                    other => warn!("Unhandled Event: {other:?}"),
                },

                // Handle login events.
                ClientboundEventEnum::Login(event) => match event {
                    ClientboundLoginEvent::CookieRequest(identifier) => {
                        info!("Received CookieRequest: \"{identifier}\"");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundLoginEvent::CookieResponse(identifier.clone(), None),
                        ));
                    }
                    ClientboundLoginEvent::CustomPayload(id, identifier, _) => {
                        info!("Received CustomPayload: \"{identifier}\"");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundLoginEvent::CustomPayload(*id, None),
                        ));
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
                        commands
                            .entity(bot.entity())
                            .insert((profile.username().clone(), profile.clone()));
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundLoginEvent::AcknowledgeLogin,
                        ));
                    }
                    other => warn!("Unhandled Event: {other:?}"),
                },

                // Can't receive a status event since the bot attempted to login.
                ClientboundEventEnum::Status(_) => unreachable!(),
            }
        }
    }
}
