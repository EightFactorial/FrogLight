//! TODO

use bevy_app::prelude::*;

use crate::set::GameCommandSet;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrigadierPlugin;

impl Plugin for BrigadierPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameCommandSet>().register_type::<GameCommandSet>();
    }
}
