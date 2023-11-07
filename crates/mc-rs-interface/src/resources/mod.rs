use bevy::prelude::*;

use crate::{configs::keybinds::Keybinds, traits::config::ResourceConfig};

pub(super) fn setup(app: &mut App) { Keybinds::setup_resource(app); }
