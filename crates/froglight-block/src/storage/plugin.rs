use bevy_app::{App, Plugin};

use super::VanillaBuilder;

/// A plugin for adding block storage.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "reflect")]
        {
            app.register_type::<crate::block::Blocks>();
            crate::generated::attribute::register(app);
        }

        VanillaBuilder::build(app);
    }
}
