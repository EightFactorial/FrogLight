use bevy::prelude::*;

use crate::{
    configs::{keybinds::Keybinds, settings::Settings},
    traits::config::{ConfigFile, ResourceConfig},
};

pub(super) fn setup(app: &mut App) {
    Settings::add_systems(app);
    Keybinds::add_systems(app);

    app.insert_resource(Keybinds::load());
}
