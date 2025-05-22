#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
use core::{any::TypeId, marker::PhantomData};
#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
use bevy_platform::hash::NoOpHash;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use derive_more::Deref;
use froglight_common::version::Version;
use indexmap::IndexMap;
use parking_lot::RwLock;

use super::EntityType;

/// A thread-safe dynamic storage for entity types.
///
/// Allows for the registration and retrieval of entity types at runtime.
#[derive(Clone, Deref)]
#[cfg_attr(feature = "bevy", derive(Resource))]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(all(feature = "bevy", feature = "reflect"), reflect(Clone, Resource))]
pub struct AppEntityTypeStorage<V: Version>(Arc<RwLock<EntityTypeStorage<V>>>);

// -------------------------------------------------------------------------------------------------

/// A dynamic storage for entity types.
///
/// Allows for the registration and retrieval of entity types at runtime.
pub struct EntityTypeStorage<V: Version> {
    #[expect(dead_code)]
    traits: IndexMap<TypeId, &'static dyn EntityType<V>, NoOpHash>,
    _phantom: PhantomData<V>,
}

impl<V: Version> EntityTypeStorage<V> {
    /// Create a new [`EntityTypeStorage`] with no registered block types.
    #[must_use]
    pub const fn new_empty() -> Self {
        Self { traits: IndexMap::with_hasher(NoOpHash), _phantom: PhantomData }
    }
}
