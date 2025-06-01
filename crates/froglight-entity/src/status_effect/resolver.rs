//! TODO

use froglight_common::version::Version;

use super::{StatusEffectStorage, StatusEffectTrait};

/// A trait for resolving status effects from global status effect ids.
pub trait StatusEffectResolver<V: Version> {
    /// The possible status effects that can be resolved.
    type EffectEnum: Sized;

    /// Register all known [`StatusEffectTrait`]s
    /// with the given [`StatusEffectStorage`].
    fn register(storage: &mut StatusEffectStorage<V>);

    /// Resolve the status effect for the given [`StatusEffectTrait`].
    fn resolve(block: &dyn StatusEffectTrait<V>) -> Option<Self::EffectEnum>;
}
