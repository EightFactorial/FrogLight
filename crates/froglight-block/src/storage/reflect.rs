use std::{any::TypeId, hash::Hash};

use bevy_ecs::world::World;
use bevy_reflect::{FromType, Reflect};
use froglight_protocol::traits::Version;

use super::BlockStorage;

/// A builder that adds blocks to the [`BlockStorage`].
#[derive(Clone, Reflect)]
pub struct ReflectBlockBuilder<V: Version> {
    /// Whether the builder should be run.
    pub enabled: bool,
    /// The order in which the builders should be run.
    pub order: i32,
    /// The type that created the builder.
    pub creator: TypeId,

    builder: fn(&mut BlockStorage<V>, &mut World, &[&Self]),
}

impl<V: Version> ReflectBlockBuilder<V> {
    /// Add blocks to the [`BlockStorage`].
    pub fn build(&self, storage: &mut BlockStorage<V>, world: &mut World, builders: &[&Self]) {
        (self.builder)(storage, world, builders);
    }
}

impl<V: Version, T: BlockBuilder<V>> FromType<T> for ReflectBlockBuilder<V> {
    fn from_type() -> Self {
        ReflectBlockBuilder {
            enabled: true,
            order: 0,
            builder: T::build,
            creator: TypeId::of::<T>(),
        }
    }
}

/// A trait that adds blocks to the [`BlockStorage`].
pub trait BlockBuilder<V: Version>: 'static {
    /// A function that adds blocks to the [`BlockStorage`].
    fn build(
        storage: &mut BlockStorage<V>,
        world: &mut World,
        builders: &[&ReflectBlockBuilder<V>],
    );
}

// --- ReflectBlockBuilder Impls ---

impl<V: Version> PartialEq for ReflectBlockBuilder<V> {
    fn eq(&self, other: &Self) -> bool {
        self.creator == other.creator && self.enabled == other.enabled && self.order == other.order
    }
}
impl<V: Version> Eq for ReflectBlockBuilder<V> {}

impl<V: Version> PartialOrd for ReflectBlockBuilder<V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.order.cmp(&other.order))
    }
}
impl<V: Version> Ord for ReflectBlockBuilder<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.order.cmp(&other.order) }
}

impl<V: Version> Hash for ReflectBlockBuilder<V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        V::ID.hash(state);
        self.enabled.hash(state);
        self.order.hash(state);
        self.creator.hash(state);
    }
}
