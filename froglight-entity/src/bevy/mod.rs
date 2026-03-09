//! TODO

use bevy_app::{App, Plugin};

use crate::prelude::*;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EntityBundle>();
        crate::generated::register_types(app);
    }
}
