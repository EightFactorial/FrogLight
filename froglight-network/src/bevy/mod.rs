//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;

mod client;
pub use client::{ClientConnection, ClientDespawn};

mod event;
pub use event::{ClientboundMessage, ServerboundMessage};

pub mod version;
pub use version::NetworkVersion;

/// A [`Plugin`] that adds [`ClientboundMessage`] and [`ServerboundMessage`]s
/// and provides systems for sending and receiving them.
///
/// # Note
///
/// This plugin does not add any systems by default. This is to allow
/// users to choose when to send and receive messages, as well as when to poll
/// connections for completion.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ClientboundMessage>().add_message::<ClientboundMessage>();
        app.register_type::<ServerboundMessage>().add_message::<ServerboundMessage>();
        app.register_type::<ClientDespawn>();
    }
}

impl NetworkPlugin {
    /// A [`System`] that sends [`ServerboundMessage`]s to the server.
    ///
    /// Not added by default, must be added manually.
    pub fn serverbound_messages(
        query: Query<(EntityRef, &ClientConnection)>,
        mut messages: ResMut<Messages<ServerboundMessage>>,
    ) {
        for message in messages.drain() {
            #[allow(clippy::manual_let_else, reason = "Not if tracing is enabled")]
            let (entity, conn) = match query.get(message.target()) {
                Ok((entity, conn)) => (entity, conn),

                #[cfg(feature = "tracing")]
                Err(bevy_ecs::query::QueryEntityError::NotSpawned(_)) => {
                    tracing::error!(target: "froglight_network", "Failed to send message, target entity does not exist");
                    continue;
                }
                #[cfg(feature = "tracing")]
                Err(bevy_ecs::query::QueryEntityError::QueryDoesNotMatch(entity, _)) => {
                    tracing::error!(target: "froglight_network", "Failed to send message, target entity {entity} does not have a ClientConnection");
                    continue;
                }
                _ => continue,
            };

            #[allow(unused_variables, reason = "Used if tracing is enabled")]
            if let Err(err) = conn.send(message.event, entity) {
                #[cfg(feature = "tracing")]
                tracing::error!(target: "froglight_network", "Failed to send message, {err}");
            }
        }
    }

    /// A [`System`] that receives [`ClientboundMessage`]s from the server.
    ///
    /// Not added by default, must added manually.
    pub fn clientbound_messages(
        query: Query<(EntityRef, &ClientConnection)>,
        mut writer: MessageWriter<ClientboundMessage>,
    ) {
        for (entity, conn) in &query {
            loop {
                match conn.receive(entity) {
                    Ok(Some(event)) => {
                        writer.write(ClientboundMessage::new(entity.id(), event));
                    }
                    Ok(None) => break,

                    #[allow(unused_variables, reason = "Used if tracing is enabled")]
                    Err(err) => {
                        #[cfg(feature = "tracing")]
                        tracing::error!(target: "froglight_network", "Failed to receive message, {err}");
                        break;
                    }
                }
            }
        }
    }

    /// A [`System`] that polls [`ClientConnection`]s for completion.
    ///
    /// Not added by default, must added manually.
    pub fn poll_connections(
        mut query: Query<(Entity, &mut ClientConnection)>,
        mut commands: Commands,
    ) {
        for (entity, mut conn) in &mut query {
            #[allow(unused_variables, reason = "Used if tracing is enabled")]
            let Some(result) = conn.poll_task() else { continue };

            #[cfg(feature = "tracing")]
            match result {
                Ok(()) => {
                    tracing::info!(target: "froglight_network", "Connection task completed, disconnecting...");
                }
                Err(err) => {
                    tracing::error!(target: "froglight_network", "Connection task failed, disconnecting...");
                    tracing::error!(target: "froglight_network", "{err}");
                }
            }

            commands.entity(entity).remove::<ClientConnection>();
        }
    }
}
