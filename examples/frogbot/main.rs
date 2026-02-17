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

struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BotPlugin::create_bot)
            .add_systems(PreUpdate, message::receive_messages)
            .add_systems(Update, BotPlugin::message_handler)
            .add_systems(PostUpdate, (message::send_messages, message::poll_connection).chain());
    }
}

impl BotPlugin {
    /// Connect to the server and spawn the bot entity.
    fn create_bot(world: &mut World) {
        const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);
        const USERNAME: &str = "FrogBot";

        // Connect to the server.
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
        let connection = ClientConnection::new::<V26_1, FuturesLite, TcpStream>(stream);

        // Prepare the handshake and login events.
        let handshake = HandshakeContent::new_socket::<V26_1>(ADDRESS, ConnectionIntent::Login);
        let login = LoginHelloContent::new_from_profile(&profile);

        // Spawn the bot entity and send the handshake and login events.
        let entity = world.spawn((connection, profile)).into_readonly();
        let conn = entity.get::<ClientConnection>().unwrap();
        conn.send(ServerboundHandshakeEvent::Handshake(handshake), entity).unwrap();
        conn.send(ServerboundLoginEvent::Hello(login), entity).unwrap();
    }

    /// Handle reading/writing all messages for the bot.
    fn message_handler(
        bot: Single<EntityRef, With<ClientConnection>>,
        mut reader: MessageReader<ClientboundMessage>,
        _writer: MessageWriter<ServerboundMessage>,
        mut commands: Commands,
    ) {
        for message in reader.read() {
            match message.event() {
                ClientboundEventEnum::Play(_event) => todo!(),

                ClientboundEventEnum::Config(_event) => todo!(),

                ClientboundEventEnum::Login(event) => match event {
                    ClientboundLoginEvent::Profile(profile) => {
                        info!("Successfully logged in as \"{}\"", profile.username());
                        commands.entity(bot.entity()).insert(profile.clone());
                    }
                    ClientboundLoginEvent::Disconnect(reason) => {
                        error!("Failed to connect to server: {reason}");
                        commands.write_message(AppExit::error());
                    }
                },

                ClientboundEventEnum::Status(_) => unreachable!("Bot attempts to login"),
            }
        }
    }
}
