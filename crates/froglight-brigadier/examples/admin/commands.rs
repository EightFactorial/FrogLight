use bevy::prelude::*;
use froglight_brigadier::{
    argument::{BrigadierTail, BrigadierWord},
    function::WorldValueRef,
    prelude::*,
};
use froglight_entity::prelude::*;

use crate::permissions::{PlayerPermissions, PlayerRole};

/// A [`Plugin`] that builds and adds commands to the [`App`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        // Add a command to log who is running commands
        app.add_command("announce", |builder| {
            builder.command(|entity, mut world| {
                let world = world.value();
                let (name, _) = player_data(entity, &world);
                info!("");
                info!("{name}:");
            });
        });

        // Add a command that adds two numbers together
        app.add_command("add", |builder| {
            let builder = builder.arg::<u32, _>().arg::<u32, _>();
            builder.command(|_, a, b, _| {
                info!("Adding   -> {a} + {b} = {}", a.saturating_add(b));
            });
        });

        // Add a command that retrieves information from the `TypeRegistry`
        app.add_command("reflect", |builder| {
            let builder = builder.arg::<BrigadierWord, _>().arg::<BrigadierTail, _>();
            builder.command(|entity, lookup_type, type_name, mut world| {
                let world = world.value();

                // Check if the player has permission to use this command
                let (name, role) = player_data(entity, &world);
                if role < PlayerRole::Moderator {
                    warn!("{name} attempted to use the \"reflect\" command!");
                    return;
                }

                // Get the type registration data
                let registry = world.resource::<AppTypeRegistry>().read();
                let type_data = match lookup_type.as_str() {
                    "name" | "short" => registry.get_with_short_type_path(&type_name),
                    "path" | "long" | "full" => registry.get_with_type_path(&type_name),
                    unk => {
                        error!("Unknown registry lookup: `{unk}`");
                        return;
                    }
                };

                // Print the type information
                if let Some(type_data) = type_data {
                    let type_path = type_data.type_info().type_path();
                    let type_kind = type_data.type_info().kind();
                    let type_id = type_data.type_id();

                    info!("TypeInfo -> {type_kind:?} \"{type_path}\", {type_id:?}");
                } else {
                    error!("Type not found in registry: `{type_name}`");
                }
            });
        });

        // Add a command to quit the application
        app.add_command("quit", |builder| {
            builder.command(|entity, mut world| {
                let mut world = world.value();
                let (name, role) = player_data(entity, &world);

                if role >= PlayerRole::Admin {
                    info!("{name} has requested a shutdown!");
                    world.send_event(AppExit::Success);
                } else {
                    warn!("{name} attempted to stop the server!");
                }
            });
        });
    }
}

/// Get the username and role of the player
fn player_data(entity: Entity, world: &WorldValueRef) -> (String, PlayerRole) {
    match world.get::<PlayerProfile>(entity) {
        Some(profile) => {
            let role = world.resource::<PlayerPermissions>().get(&*profile.uuid);
            (profile.username.to_string(), role.copied().unwrap_or_default())
        }
        None => (format!("Entity {entity}"), PlayerRole::default()),
    }
}
