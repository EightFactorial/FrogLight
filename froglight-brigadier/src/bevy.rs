//! TODO

use bevy_app::{App, Plugin};

use crate::graph::CommandGraph;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrigadierPlugin;

impl Plugin for BrigadierPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CommandGraph>().register_type::<CommandGraph>();
    }
}
