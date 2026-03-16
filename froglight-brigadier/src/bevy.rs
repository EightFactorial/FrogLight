//! TODO

use bevy_app::{App, Plugin};

use crate::set::CommandSet;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrigadierPlugin;

impl Plugin for BrigadierPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandSet>().register_type::<CommandSet>();
    }
}
