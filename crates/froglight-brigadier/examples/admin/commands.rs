use bevy::prelude::*;
use froglight_brigadier::{function::WorldValueRef, prelude::*};
use froglight_entity::prelude::*;

use crate::permissions::{PlayerPermissions, PlayerRole};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        // Add a command to quit the application
        app.add_command("quit", |builder| {
            builder.command(|entity, mut world| {
                let mut world = world.value();

                let name = username_or_id(entity, &world);
                let role = player_role(entity, &world);

                if role >= PlayerRole::Admin {
                    warn!("{name} has requested a shutdown!");
                    world.send_event(AppExit::Success);
                } else {
                    warn!("{name} attempted to stop the server!");
                }
            });
        });
    }
}

/// Get the username of a player, or fallback to the entity id.
fn username_or_id(entity: Entity, world: &WorldValueRef) -> String {
    match world.get::<PlayerProfile>(entity) {
        Some(profile) => profile.username.to_string(),
        None => format!("Entity {entity}"),
    }
}

/// Get the player's role.
fn player_role(entity: Entity, world: &WorldValueRef) -> PlayerRole {
    if let Some(uuid) = world.get::<PlayerUuid>(entity) {
        world.resource::<PlayerPermissions>().get(&**uuid).copied().unwrap_or_default()
    } else {
        PlayerRole::default()
    }
}
