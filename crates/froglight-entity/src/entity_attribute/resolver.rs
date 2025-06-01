//! TODO

use froglight_common::version::Version;

use super::{EntityAttributeStorage, EntityAttributeTrait};

/// A trait for resolving entity attribute from global attribute ids.
pub trait EntityAttributeResolver<V: Version> {
    /// The possible attributes that can be resolved.
    type AttributeEnum: Sized;

    /// Register all known [`EntityAttributeTrait`]s
    /// with the given [`EntityAttributeStorage`].
    fn register(storage: &mut EntityAttributeStorage<V>);

    /// Resolve the attribute for the given [`EntityAttributeTrait`].
    fn resolve(block: &dyn EntityAttributeTrait<V>) -> Option<Self::AttributeEnum>;
}
