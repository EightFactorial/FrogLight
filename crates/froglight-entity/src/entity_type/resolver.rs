//! TODO

use froglight_common::version::Version;

use super::{storage::EntityTypeStorage, traits::EntityTypeTrait};

/// A trait for resolving entity types from global entity type IDs.
pub trait EntityTypeResolver<V: Version> {
    /// The possible entity types that can be resolved.
    type EntityEnum: Sized;

    /// Register all known [`EntityTypeTrait`]s
    /// with the given [`EntityTypeStorage`].
    fn register(storage: &mut EntityTypeStorage<V>);

    /// Resolve the entity type for the given [`EntityTypeTrait`].
    fn resolve(block: &dyn EntityTypeTrait<V>) -> Option<Self::EntityEnum>;
}
