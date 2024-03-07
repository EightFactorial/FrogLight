use std::sync::Arc;

use bevy_app::App;
use bevy_ecs::{reflect::ReflectResource, system::Resource};
use bevy_reflect::Reflect;
use derive_more::Deref;
use froglight_protocol::traits::Version;
use parking_lot::RwLock;

mod inner;
pub(crate) use inner::InnerRegistry;

use super::traits::BlockRegistration;

#[cfg(feature = "inspector")]
mod egui;

mod versions;

#[doc(hidden)]
pub(super) fn build(_app: &mut App) {
    // TODO: Initialize block registries
}

/// A registry containing all of the blocks in the game.
#[derive(Debug, Default, Clone, Deref, Reflect, Resource)]
#[reflect(Resource)]
pub struct BlockRegistry<V: Version>
where
    InnerRegistry<V>: Default,
{
    #[reflect(ignore)]
    pub(crate) inner: Arc<RwLock<InnerRegistry<V>>>,
}

impl<V: Version> BlockRegistry<V>
where
    InnerRegistry<V>: Default,
    V: BlockRegistration,
{
    /// Register for reflection, and initialize the block registry.
    #[allow(dead_code)]
    fn initialize(app: &mut App) {
        // Register the block registry for reflection
        app.register_type::<BlockRegistry<V>>();

        // Create the block registry and register all vanilla blocks
        let registry = Self::default();
        V::register_blocks(&mut registry.inner.write());
        app.insert_resource(registry);

        // Optionally register the block registry inside the inspector
        #[cfg(feature = "inspector")]
        Self::egui_register(app);
    }
}
