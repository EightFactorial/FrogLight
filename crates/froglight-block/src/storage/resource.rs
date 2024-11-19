use std::sync::Arc;

use bevy_ecs::{
    reflect::{AppTypeRegistry, ReflectResource},
    system::Resource,
    world::{FromWorld, World},
};
use bevy_reflect::Reflect;
use derive_more::derive::{Deref, DerefMut};
use froglight_protocol::traits::Version;
use parking_lot::RwLock;

use super::{BlockStorage, ReflectBlockBuilder};

/// A storage container for blocks.
///
/// This is a shared wrapper around a [`BlockStorage`]
/// and can be cloned cheaply.
#[derive(Clone, Deref, DerefMut, Reflect, Resource)]
#[reflect(Resource)]
pub struct BlockStorageArc<V: Version>(Arc<RwLock<BlockStorage<V>>>);

impl<V: Version> FromWorld for BlockStorageArc<V> {
    fn from_world(world: &mut World) -> Self {
        let mut storage = BlockStorage::<V>::new_empty();

        let registry = world.resource::<AppTypeRegistry>().clone();
        let registry = registry.read();

        // Collect and sort the block builders.
        let mut builders: Vec<_> =
            registry.iter().filter_map(|r| r.data::<ReflectBlockBuilder<V>>()).collect();
        builders.sort();

        // Run the block builders.
        for builder in &builders {
            builder.build(&mut storage, world, &builders);
        }

        Self(Arc::new(RwLock::new(storage)))
    }
}
