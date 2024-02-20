use std::sync::Arc;

use bevy::prelude::*;

pub(crate) mod inner;
use inner::AssetManagerInner;

/// A manager for resource packs and their assets.
#[derive(Debug, Default, Clone, Deref, Resource, Reflect)]
#[reflect(Resource)]
pub struct AssetManager(Arc<AssetManagerInner>);

impl From<AssetManagerInner> for AssetManager {
    fn from(inner: AssetManagerInner) -> Self { Self(Arc::new(inner)) }
}

impl From<&Arc<AssetManagerInner>> for AssetManager {
    fn from(inner: &Arc<AssetManagerInner>) -> Self { Self(Arc::clone(inner)) }
}
