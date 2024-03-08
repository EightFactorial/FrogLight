use std::{any::TypeId, sync::Arc};

use bevy_app::App;
use bevy_ecs::{
    reflect::{AppTypeRegistry, ReflectResource},
    system::Resource,
    world::{FromWorld, World},
};
use bevy_reflect::{GetTypeRegistration, Reflect};
use derive_more::Deref;
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
#[derive(Debug, Clone, Deref, Reflect, Resource)]
#[reflect(Resource)]
pub struct BlockRegistry<V: BlockRegistration> {
    #[reflect(ignore)]
    pub(crate) inner: Arc<RwLock<InnerRegistry<V>>>,
}

impl<V: BlockRegistration> FromWorld for BlockRegistry<V> {
    fn from_world(world: &mut World) -> Self {
        // Add a type registration for the block registry if it doesn't exist
        if let Some(registry) = world.get_resource::<AppTypeRegistry>() {
            let registry_exists = registry.read().get(TypeId::of::<Self>()).is_none();

            if !registry_exists {
                // Create the type registration
                #[allow(unused_mut)]
                let mut registration = Self::get_type_registration();

                // Add the `InspectorEguiImpl` to the type registration
                #[cfg(feature = "inspector")]
                {
                    registration.insert(Self::inspector_egui_impl());
                }

                // Add the registration to the app type registry
                registry.write().add_registration(registration);
            }
        }

        // Create an empty block registry and add all vanilla blocks
        let mut inner = InnerRegistry::default();
        V::register_blocks(&mut inner);

        // Return the block registry
        Self { inner: Arc::new(RwLock::new(inner)) }
    }
}
