use std::sync::Arc;

use bevy_app::App;
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::Reflect;
use derive_more::Deref;

mod inner;
use inner::AtlasManagerInner;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    inner::build(app);

    app.register_type::<AtlasManager>().init_resource::<AtlasManager>();
}

/// A manager for texture atlases.
#[derive(Debug, Default, Clone, Deref, Resource, Reflect)]
#[reflect(Resource)]
pub struct AtlasManager(Arc<AtlasManagerInner>);

impl From<AtlasManagerInner> for AtlasManager {
    fn from(inner: AtlasManagerInner) -> Self { Self(Arc::new(inner)) }
}

impl From<&Arc<AtlasManagerInner>> for AtlasManager {
    fn from(inner: &Arc<AtlasManagerInner>) -> Self { Self(Arc::clone(inner)) }
}
