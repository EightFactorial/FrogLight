use bevy::prelude::*;
use froglight::prelude::*;

/// Send messages to the server.
pub(super) fn send_messages(
    bot: Single<(EntityRef, &ClientConnection)>,
    mut messages: ResMut<Messages<ServerboundMessage>>,
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
            return;
        }
    }
}

/// Receive messages from the server.
pub(super) fn receive_messages(
    bot: Single<(EntityRef, &ClientConnection)>,
    mut messages: MessageWriter<ClientboundMessage>,
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
                error!("Failed to receive message, {err}");
                break;
            }
        }
    }
}

pub(super) fn poll_connection(
    mut bot: Single<(Entity, &mut ClientConnection)>,
    mut commands: Commands,
) {
    let (entity, bot) = &mut *bot;
    match bot.poll_task() {
        None => {}
        Some(Ok(())) => {
            warn!("Connection task completed, disconnecting...");
            commands.entity(*entity).remove::<ClientConnection>();
        }
        Some(Err(err)) => {
            error!("Connection task failed, disconnecting...");
            error!("{err}");
            commands.write_message(AppExit::error());
        }
    }
}
