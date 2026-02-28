//! TODO

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_net::TcpStream;
use bevy::{prelude::*, tasks::block_on};
use froglight::{
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
    plugins::NetworkPlugin,
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FroglightPlugins)
        .add_plugins(BotPlugin)
        .run()
}

// -------------------------------------------------------------------------------------------------

/// A custom [`Plugin`] for FrogBot.
struct BotPlugin;

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
        const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
        const USERNAME: &str = "FrogBot";
        type Protocol = V26_1;

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
        let connection = ClientConnection::new::<Protocol, FuturesLite, TcpStream>(stream, false);

        info!(
            "Attempting to login as \"{}\" ({})...",
            profile.username(),
            profile.uuid().as_hyphenated()
        );

        // Prepare the handshake and login events.
        let handshake = HandshakeContent::new_socket::<Protocol>(ADDRESS, ConnectionIntent::Login);
        let login = LoginHelloContent::new_from_profile(&profile);

        // Spawn the bot entity and exit the app when it despawns.
        let mut entity = world.spawn((connection, profile));
        entity.observe(BotPlugin::exit_on_despawn);

        // Send the handshake and login events.
        let entity = entity.into_readonly();
        let conn = entity.get::<ClientConnection>().unwrap();
        let _ = conn.send(ServerboundHandshakeEvent::Handshake(handshake), entity);
        let _ = conn.send(ServerboundLoginEvent::Hello(login), entity);
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
                    ClientboundPlayEvent::AddEntity(entity_data) => {
                        let bundle = (
                            EntityOfInstance::new(bot.id()),
                            entity_data.entity_id.0,
                            entity_data.entity_uuid,
                        );

                        let entity = commands.spawn(bundle).id();
                        debug!("Spawning Entity {entity} ({})", entity_data.entity_id.0.0);
                    }
                    // ClientboundPlayEvent::Animate() => todo!(),
                    // ClientboundPlayEvent::AwardStats() => todo!(),
                    // ClientboundPlayEvent::BlockChangedAck() => todo!(),
                    // ClientboundPlayEvent::BlockDestruction() => todo!(),
                    // ClientboundPlayEvent::BlockEntityData() => todo!(),
                    // ClientboundPlayEvent::BlockEvent() => todo!(),
                    // ClientboundPlayEvent::BlockUpdate() => todo!(),
                    // ClientboundPlayEvent::BossEvent() => todo!(),
                    // ClientboundPlayEvent::BundleDelimiter => todo!(),
                    // ClientboundPlayEvent::ChangeDifficulty() => todo!(),
                    // ClientboundPlayEvent::ChatSuggestions() => todo!(),
                    // ClientboundPlayEvent::ChunkBatchFinished() => todo!(),
                    // ClientboundPlayEvent::ChunkBatchStart() => todo!(),
                    // ClientboundPlayEvent::ChunkBiomes() => todo!(),
                    // ClientboundPlayEvent::ChunkCacheCenter() => todo!(),
                    // ClientboundPlayEvent::ChunkCacheRadius() => todo!(),
                    // ClientboundPlayEvent::ChunkSectionUpdate() => todo!(),
                    // ClientboundPlayEvent::ChunkWithLight() => todo!(),
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
                    // ClientboundPlayEvent::EntityPosition() => todo!(),
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

                        // Prepare the bot's `WorldInstance` for tracking entities
                        let mut commands = commands.entity(bot.id());
                        commands.insert((
                            WorldInstance::new(login.spawn_info.dimension.clone()),
                            EntityOfInstance::new(bot.id()),
                        ));

                        // Add the bot's `EntityId` and `EntityUuid`
                        let profile = bot.get::<PlayerProfile>().unwrap();
                        commands.insert((login.player_id, EntityUuid::new(*profile.uuid())));
                    }
                    // ClientboundPlayEvent::MapItemData() => todo!(),
                    // ClientboundPlayEvent::MerchantOffers() => todo!(),
                    // ClientboundPlayEvent::MountScreen() => todo!(),
                    // ClientboundPlayEvent::MoveEntityPos() => todo!(),
                    // ClientboundPlayEvent::MoveEntityPosRot() => todo!(),
                    // ClientboundPlayEvent::MoveEntityRot() => todo!(),
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
                    // ClientboundPlayEvent::PlayerPosition() => todo!(),
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
                                commands.entity(entity).despawn();
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
                    // ClientboundPlayEvent::RotateHead() => todo!(),
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
                    // ClientboundPlayEvent::SetEntityData() => todo!(),
                    // ClientboundPlayEvent::SetEntityLink() => todo!(),
                    // ClientboundPlayEvent::SetEntityMotion() => todo!(),
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
                    // ClientboundPlayEvent::SetTime() => todo!(),
                    // ClientboundPlayEvent::SetTitleAnimation() => todo!(),
                    // ClientboundPlayEvent::SetTitleText() => todo!(),
                    // ClientboundPlayEvent::ShowDialog() => todo!(),
                    // ClientboundPlayEvent::Sound() => todo!(),
                    // ClientboundPlayEvent::SoundEntity() => todo!(),
                    ClientboundPlayEvent::StartConfiguration => {
                        info!("Reconfiguring...");
                        commands.entity(bot.id()).remove::<WorldInstance>();
                    }
                    // ClientboundPlayEvent::StopSound() => todo!(),
                    // ClientboundPlayEvent::StoreCookie() => todo!(),
                    // ClientboundPlayEvent::SystemChat() => todo!(),
                    // ClientboundPlayEvent::TabList() => todo!(),
                    // ClientboundPlayEvent::TagQuery() => todo!(),
                    // ClientboundPlayEvent::TakeItemEntity() => todo!(),
                    // ClientboundPlayEvent::TeleportEntity() => todo!(),
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
                    ClientboundConfigEvent::RegistryData() => {
                        info!("Received RegistryData: <placeholder>");
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
                    ClientboundConfigEvent::UpdateTags() => {
                        info!("Received UpdateTags: <placeholder>");
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
