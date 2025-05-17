//! TODO
#![allow(unreachable_pub)]

use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
};
use froglight_brigadier::plugin::BrigadierPlugin;

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

    app.run()
}

// -------------------------------------------------------------------------------------------------
