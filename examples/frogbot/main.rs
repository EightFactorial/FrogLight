//! TODO

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use async_net::TcpStream;
use bevy::{prelude::*, tasks::block_on};
use froglight::{
    network::{connection::FuturesLite, event::ServerboundHandshakeEvent},
    packet::common::handshake::{ConnectionIntent, HandshakeContent},
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

struct BotPlugin;

impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_bot)
            .add_systems(PreUpdate, Self::recv_connection_events)
            .add_systems(PostUpdate, Self::send_connection_events);
    }
}

impl BotPlugin {
    /// Connect to the server and spawn the bot entity.
    fn create_bot(world: &mut World) {
        const ADDRESS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 25565);

        // Connect to the server.
        let stream = match block_on(TcpStream::connect(ADDRESS)) {
            Ok(stream) => stream,
            Err(err) => {
                error!("Failed to connect to server: {err}");
                world.write_message(AppExit::error());
                return;
            }
        };

        // Create a `ClientConnection` and spawn it.
        let conn = ClientConnection::new::<V26_1, FuturesLite, TcpStream>(stream);
        let entity = world.spawn(conn);
        let conn = entity.get::<ClientConnection>().unwrap();

        // Send a handshake packet to the server.
        let handshake = HandshakeContent::new_socket::<V26_1>(ADDRESS, ConnectionIntent::Login);
        conn.send(ServerboundHandshakeEvent::Handshake(handshake), entity.as_readonly()).unwrap();
    }

    /// Send messages to the server.
    fn send_connection_events(
        bot: Single<(EntityRef, &ClientConnection)>,
        mut messages: ResMut<Messages<ServerboundMessage>>,
        mut commands: Commands,
    ) {
        let (entity, conn) = *bot;

        for message in messages.drain() {
            // Warn if the message isn't for the bot entity.
            if message.target() != entity.id() {
                warn!(
                    "Received a message for a different entity: {} != {}",
                    message.target(),
                    entity.id()
                );
                continue;
            }

            // Send the message to the server.
            if let Err(err) = conn.send(message.event, entity) {
                error!("Failed to send message: {err}");
                commands.write_message(AppExit::error());
                return;
            }
        }
    }

    /// Receive messages from the server.
    fn recv_connection_events(
        bot: Single<(EntityRef, &ClientConnection)>,
        mut messages: MessageWriter<ClientboundMessage>,
        mut commands: Commands,
    ) {
        let (entity, conn) = *bot;

        loop {
            match conn.receive(entity) {
                Ok(Some(event)) => {
                    // Write the message to the world.
                    messages.write(ClientboundMessage::new(entity.id(), event));
                }
                Ok(None) => break,

                Err(err) => {
                    error!("Failed to receive message: {err}");
                    commands.write_message(AppExit::error());
                    return;
                }
            }
        }
    }
}
