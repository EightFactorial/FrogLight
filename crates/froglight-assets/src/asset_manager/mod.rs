use std::sync::Arc;

use bevy_derive::Deref;
use bevy_ecs::system::Resource;

mod inner;
use inner::AssetManagerInner;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { app.init_resource::<AssetManager>(); }

/// A manager for all assets loaded from
/// [`ResourcePacks`](crate::assets::resourcepack::ResourcePack).
#[derive(Debug, Default, Clone, Deref, Resource)]
pub struct AssetManager(Arc<AssetManagerInner>);
