use std::{any::TypeId, sync::Arc};

use bevy_app::App;
use bevy_ecs::{
    reflect::{AppTypeRegistry, ReflectResource},
    system::Resource,
    world::{FromWorld, World},
};
use bevy_log::debug;
use bevy_reflect::{GetTypeRegistration, Reflect};
use derive_more::Deref;
use froglight_protocol::versions::v1_20_0::V1_20_0;
use parking_lot::RwLock;

use super::traits::BlockRegistration;

#[cfg(feature = "inspector")]
mod egui;

mod inner;
pub use inner::InnerBlockRegistry;

#[cfg(test)]
mod tests;

mod versions;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.init_resource::<BlockRegistry<V1_20_0>>(); }

/// A registry containing all of the blocks in the game.
#[derive(Debug, Clone, Deref, Reflect, Resource)]
#[reflect(Resource)]
pub struct BlockRegistry<V: BlockRegistration> {
    #[reflect(ignore)]
    pub(crate) inner: Arc<RwLock<InnerBlockRegistry<V>>>,
}

impl<V: BlockRegistration> BlockRegistry<V> {
    /// Creates a new empty block registry.
    #[must_use]
    pub fn new_empty() -> Self {
        Self { inner: Arc::new(RwLock::new(InnerBlockRegistry::default())) }
    }

    /// Creates a new block registry with all of the default blocks.
    #[must_use]
    pub fn new_default() -> Self {
        // Create an empty block registry and add all vanilla blocks
        let mut inner = InnerBlockRegistry::default();
        V::register_default(&mut inner);

        Self { inner: Arc::new(RwLock::new(inner)) }
    }
}

impl<V: BlockRegistration> FromWorld for BlockRegistry<V> {
    fn from_world(world: &mut World) -> Self {
        // Add a type registration for the block registry if it doesn't exist
        if let Some(registry) = world.get_resource::<AppTypeRegistry>() {
            let registry_exists = registry.read().get(TypeId::of::<Self>()).is_none();
            if registry_exists {
                // Create the type registration
                #[allow(unused_mut)]
                let mut registration = Self::get_type_registration();

                // Add the `InspectorEguiImpl` to the type registration
                #[cfg(feature = "inspector")]
                registration.insert(Self::inspector_egui_impl());

                // Add the registration to the app type registry
                registry.write().add_registration(registration);

                // Register all blocks with version specific type data
                V::register_reflect(world);
            }
        }

        debug!("Initializing BlockRegistry<{:?}>", V::default());

        // Create a new block registry with all of the default blocks
        Self::new_default()
    }
}
