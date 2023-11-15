use bevy::prelude::*;

use crate::{
    configs::{keybinds::Keybinds, settings::Settings},
    traits::config::{ConfigFile, ResourceConfig},
};

pub mod resourcepacks;

pub(super) fn setup(app: &mut App) {
    Settings::add_systems(app);
    Keybinds::add_systems(app);
    app.insert_resource(Keybinds::load());

    resourcepacks::setup(app);
}
