//! TODO

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_net::TcpStream;
use bevy::{prelude::*, tasks::block_on};
use froglight::{
    network::{
        connection::FuturesLite,
        event::{ClientboundLoginEvent, ServerboundHandshakeEvent, ServerboundLoginEvent},
    },
    packet::common::{
        handshake::{ConnectionIntent, HandshakeContent},
        login::LoginHelloContent,
    },
    prelude::*,
};

mod message;

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
            .add_systems(PreUpdate, message::receive_messages)
            .add_systems(Update, BotPlugin::message_handler)
            .add_systems(PostUpdate, (message::send_messages, message::poll_connection).chain());
    }
}

impl BotPlugin {
    /// Connect to the server and spawn the bot entity.
    ///
    /// Run once during [`Startup`].
    fn create_bot(world: &mut World) {
        const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
        const USERNAME: &str = "FrogBot";

        // Connect to the server.
        info!("Connecting to {ADDRESS} as {USERNAME}...");
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
        let connection = ClientConnection::new::<V26_1, FuturesLite, TcpStream>(stream, false);

        // Prepare the handshake and login events.
        let handshake = HandshakeContent::new_socket::<V26_1>(ADDRESS, ConnectionIntent::Login);
        let login = LoginHelloContent::new_from_profile(&profile);

        // Spawn the bot entity and send the handshake and login events.
        let entity = world.spawn((connection, profile)).into_readonly();
        let conn = entity.get::<ClientConnection>().unwrap();
        let _ = conn.send(ServerboundHandshakeEvent::Handshake(handshake), entity);
        let _ = conn.send(ServerboundLoginEvent::Hello(login), entity);
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
                ClientboundEventEnum::Play(_event) => todo!(),

                // Handle configuration events.
                ClientboundEventEnum::Config(_event) => todo!(),

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
                        info!("Received query request: <placeholder>");
                    }
                    ClientboundLoginEvent::CookieRequest() => {
                        info!("Received cookie request: <placeholder>");
                    }
                    ClientboundLoginEvent::Profile(profile) => {
                        info!("Received profile for \"{}\"", profile.username());
                        commands
                            .entity(bot.entity())
                            .insert((profile.username().clone(), profile.clone()));
                    }
                    ClientboundLoginEvent::LoginComplete => {
                        info!("Login complete!");
                        writer.write(ServerboundMessage::new(
                            bot.id(),
                            ServerboundLoginEvent::AcknowledgeLogin,
                        ));
                    }
                },

                // Can't receive a status event since the bot attempted to login.
                ClientboundEventEnum::Status(_) => unreachable!(),
            }
        }
    }
}
