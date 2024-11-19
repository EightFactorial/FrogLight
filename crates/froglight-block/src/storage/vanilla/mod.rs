use bevy_app::App;
use bevy_reflect::Reflect;
use froglight_protocol::versions::v1_21_0::V1_21_0;

use super::{ReflectBlockBuilder, BlockStorageArc};

mod v1_21_0;

/// A builder for vanilla block storage.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct VanillaBuilder;

impl VanillaBuilder {
    pub(super) fn build(app: &mut App) {
        app.register_type::<Self>();
        app.register_type_data::<Self, ReflectBlockBuilder<V1_21_0>>();
        app.init_resource::<BlockStorageArc<V1_21_0>>();
    }
}
