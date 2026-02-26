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
                    ClientboundPlayEvent::Placeholder => info!("Received <placeholder>"),
                    other => info!("Received unhandled play event: {other:?}"),
                },

                // Handle configuration events.
                ClientboundEventEnum::Config(event) => match event {
                    ClientboundConfigEvent::Disconnect(reason) => {
                        error!("Disconnected from server: {reason:?}");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundConfigEvent::TransferServer() => {
                        error!("Received transfer server event!");
                        error!("Did you attempt to login to a BungeeCord/Velocity proxy?");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundConfigEvent::KeepAlive(id) => {
                        info!("Received KeepAlive ({id})");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::KeepAlive(*id),
                        ));
                    }
                    ClientboundConfigEvent::Ping(id) => {
                        info!("Received Ping ({id})");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::Pong(*id),
                        ));
                    }
                    ClientboundConfigEvent::ResetChat => {
                        info!("Received ResetChat");
                    }
                    ClientboundConfigEvent::ResourcePackQuery(known) => {
                        info!("Received ResourcePackQuery: {known:#?}");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::ResourcePackResponse(Vec::new()),
                        ));
                    }
                    ClientboundConfigEvent::ResourcePackPush() => {
                        info!("Received ResourcePackPush: <placeholder>");
                    }
                    ClientboundConfigEvent::ResourcePackPop() => {
                        info!("Received ResourcePackPop: <placeholder>");
                    }
                    ClientboundConfigEvent::UpdateRegistries() => {
                        info!("Received UpdateRegistries: <placeholder>");
                    }
                    ClientboundConfigEvent::UpdateFeatures() => {
                        info!("Received UpdateFeatures: <placeholder>");
                    }
                    ClientboundConfigEvent::UpdateTags() => {
                        info!("Received UpdateTags: <placeholder>");
                    }
                    ClientboundConfigEvent::ServerLinks() => {
                        info!("Received ServerLinks: <placeholder>");
                    }
                    ClientboundConfigEvent::CodeOfConduct() => {
                        warn!("Accepting code of conduct...");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::AcceptCodeOfConduct,
                        ));
                    }
                    ClientboundConfigEvent::ReportDetails() => {
                        info!("Received ReportDetails: <placeholder>");
                    }
                    ClientboundConfigEvent::CustomQuery(identifier, _) => {
                        info!("Received CustomQuery: \"{identifier}\"");

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
                    ClientboundConfigEvent::CookieRequest() => {
                        info!("Received CookieRequest: <placeholder>");
                    }
                    ClientboundConfigEvent::CookieStore() => {
                        info!("Received CookieStore: <placeholder>");
                    }
                    ClientboundConfigEvent::ShowDialog() => {
                        info!("Received ShowDialog: <placeholder>");
                    }
                    ClientboundConfigEvent::ClearDialog => {
                        info!("Clearing dialog...");
                    }
                    ClientboundConfigEvent::FinishConfig => {
                        info!("Successfully finished configuration!");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundConfigEvent::AcknowledgeConfig,
                        ));
                    }
                    other => warn!("Received an unhandled config event: {other:?}"),
                },

                // Handle login events.
                ClientboundEventEnum::Login(event) => match event {
                    ClientboundLoginEvent::Disconnect(reason) => {
                        error!("Failed to connect to server: {reason}");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundLoginEvent::EncryptionRequest() => {
                        error!("Received encryption request!");
                        error!("Did you attempt to login to an online-mode server?");
                        commands.write_message(AppExit::error());
                    }
                    ClientboundLoginEvent::QueryRequest() => {
                        info!("Received QueryRequest: <placeholder>");
                    }
                    ClientboundLoginEvent::CookieRequest() => {
                        info!("Received CookieRequest: <placeholder>");
                    }
                    ClientboundLoginEvent::Profile(profile) => {
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
                    other => warn!("Received an unhandled login event: {other:?}"),
                },

                // Can't receive a status event since the bot attempted to login.
                ClientboundEventEnum::Status(_) => unreachable!(),
            }
        }
    }
}
