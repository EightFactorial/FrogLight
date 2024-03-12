use std::{any::TypeId, sync::Arc};

use bevy_app::App;
use bevy_ecs::{
    prelude::ReflectResource,
    reflect::AppTypeRegistry,
    system::Resource,
    world::{FromWorld, World},
};
use bevy_log::debug;
use bevy_reflect::{GetTypeRegistration, Reflect};
use derive_more::Deref;
use froglight_protocol::versions::v1_20_0::V1_20_0;
use parking_lot::RwLock;

mod inner;
pub use inner::InnerBiomeRegistry;

use super::traits::BiomeRegistration;

#[cfg(feature = "inspector")]
mod egui;

#[cfg(test)]
mod tests;

mod versions;
pub use versions::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.init_resource::<BiomeRegistry<V1_20_0>>(); }

/// A registry containing all of the blocks in the game.
#[derive(Debug, Clone, Deref, Reflect, Resource)]
#[reflect(Resource)]
pub struct BiomeRegistry<V: BiomeRegistration> {
    #[reflect(ignore)]
    pub(crate) inner: Arc<RwLock<InnerBiomeRegistry<V>>>,
}

impl<V: BiomeRegistration> BiomeRegistry<V> {
    /// Creates a new empty biomes registry.
    #[must_use]
    pub fn new_empty() -> Self {
        Self { inner: Arc::new(RwLock::new(InnerBiomeRegistry::default())) }
    }

    /// Creates a new biomes registry with all of the default biomes.
    #[must_use]
    pub fn new_default() -> Self {
        // Create an empty biomes registry and add all vanilla biomes
        let mut inner = InnerBiomeRegistry::default();
        V::register_default(&mut inner);

        Self { inner: Arc::new(RwLock::new(inner)) }
    }
}

impl<V: BiomeRegistration> FromWorld for BiomeRegistry<V> {
    fn from_world(world: &mut World) -> Self {
        // Add a type registration for the biome registry if it doesn't exist
        if let Some(registry) = world.get_resource::<AppTypeRegistry>() {
            let registry_exists = registry.read().get(TypeId::of::<Self>()).is_none();
            if registry_exists {
                debug!("Registering BiomeRegistry<{:?}>", V::default());

                // Create the type registration
                #[allow(unused_mut)]
                let mut registration = Self::get_type_registration();

                // Add the `InspectorEguiImpl` to the type registration
                #[cfg(feature = "inspector")]
                {
                    debug!("Adding InspectorEguiImpl for BiomeRegistry<{:?}>", V::default());
                    registration.insert(Self::inspector_egui_impl());
                }

                // Add the registration to the app type registry
                registry.write().add_registration(registration);
            }
        }

        // Create a new biome registry with all of the default biomes
        Self::new_default()
    }
}
