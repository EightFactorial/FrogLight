//! A demonstration of how to add commands and retrieve data from the ECS.
//!
//! If you are writing a more serious application where permissions matter,
//! consider checking for permissions before executing `run_command`.

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use froglight_brigadier::plugin::{BrigadierCommands, BrigadierPlugin};
use froglight_entity::prelude::PlayerProfile;
use uuid::Uuid;

mod commands;
use commands::CommandsPlugin;

mod permissions;
use permissions::PermissionsPlugin;

fn main() -> AppExit {
    let mut app = App::new();
    // Add the required plugins and the `BrigadierPlugin`
    app.add_plugins((MinimalPlugins, BrigadierPlugin::default()));
    app.add_plugins(LogPlugin { level: Level::INFO, ..default() });

    // Add our custom `PermissionsPlugin` and `CommandsPlugin`
    app.add_plugins(PermissionsPlugin::new("admin.toml"));
    app.add_plugins(CommandsPlugin);

    // Add systems that spawn player entities and run commands
    app.add_systems(
        Update,
        (default_player, moderator_player, admin_player, owner_player).chain().run_if(run_once),
    );

    app.run()
}

// -------------------------------------------------------------------------------------------------

/// Spawn and run commands as "DefaultPlayer"
fn default_player(mut commands: Commands) {
    let profile = PlayerProfile::new("DefaultPlayer", Uuid::nil());
    let mut entity = commands.spawn(profile);

    entity.run_command("announce");
    entity.run_command("add 1 2");
    entity.run_command("reflect short u8");
    entity.run_command("quit");
}

/// Spawn and run commands as "ModeratorPlayer"
fn moderator_player(mut commands: Commands) {
    let uuid = Uuid::parse_str("16fe7587-31f5-4cd7-bdaa-d40dde387829").unwrap();
    let profile = PlayerProfile::new("ModeratorPlayer", uuid);
    let mut entity = commands.spawn(profile);

    entity.run_command("announce");
    entity.run_command("add 10 10");
    entity.run_command("reflect short u8");
    entity.run_command("reflect short i8");
    entity.run_command("quit");
}

/// Spawn and run commands as "AdminPlayer"
fn admin_player(mut commands: Commands) {
    let uuid = Uuid::parse_str("3415d071-2c51-4d37-b659-5df3687fbe7a").unwrap();
    let profile = PlayerProfile::new("AdminPlayer", uuid);
    let mut entity = commands.spawn(profile);

    entity.run_command("announce");
    entity.run_command("add 0 0");
    entity.run_command("reflect short u32");
    entity.run_command("reflect short String");
    entity.run_command("quit");
}

/// Spawn and run commands as "OwnerPlayer"
fn owner_player(mut commands: Commands) {
    let uuid = Uuid::parse_str("36982e9b-5e6c-405c-a18d-1cfbf5e8e57e").unwrap();
    let profile = PlayerProfile::new("OwnerPlayer", uuid);
    let mut entity = commands.spawn(profile);

    entity.run_command("announce");
    entity.run_command("add 100 23");
    entity.run_command("reflect short Uuid");
    entity.run_command("reflect short PlayerRole");
    entity.run_command("quit");
}
