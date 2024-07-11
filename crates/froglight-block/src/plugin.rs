use bevy_app::{App, Plugin};
use froglight_protocol::versions::v1_21_0::V1_21_0;

use crate::BlockRegistry;

/// The `Block` Froglight plugin.
///
/// Adds support for converting blocks to and from ids.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        crate::definitions::build(app);

        app.init_resource::<BlockRegistry<V1_21_0>>();
    }
}
