#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
use core::any::TypeId;
#[cfg(feature = "std")]
use std::sync::Arc;

#[cfg(feature = "bevy")]
use bevy_ecs::prelude::*;
#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_common::prelude::*;
use froglight_utils::storage::prelude::*;

use super::{StatusEffectExt, StatusEffectResolver, StatusEffectTrait};

/// A dynamic storage for status effects.
///
/// Allows for the registration and retrieval of status effects at runtime.
#[repr(transparent)]
#[derive(Clone, AppStorage)]
#[storage(index(ident = "GlobalStatusEffectId", inner = "u8"), bevy = "bevy", reflect = "reflect")]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Clone))]
pub struct StatusEffectStorage<V: Version>(
    IndexedLocalStorage<dyn StatusEffectTrait<V>, GlobalStatusEffectId>,
);

impl<V: Version> AppStatusEffectStorage<V> {
    /// Create a new [`AppStatusEffectStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: StatusEffectResolver<V> {
        Self::from_storage(StatusEffectStorage::new())
    }
}

impl<V: Version> StatusEffectStorage<V> {
    /// Create a new [`StatusEffectStorage`] with the [`Vanilla`] types
    /// registered.
    #[must_use]
    pub fn new() -> Self
    where Vanilla: StatusEffectResolver<V> {
        let mut storage = Self::new_empty();
        <Vanilla as StatusEffectResolver<V>>::register(&mut storage);
        storage
    }

    /// Create a new [`StatusEffectStorage`] with no registered status effects.
    #[must_use]
    pub const fn new_empty() -> Self { Self(IndexedLocalStorage::new()) }

    /// Get the [`StatusEffectTrait`] for the given [`GlobalStatusEffectId`].
    ///
    /// Handy for storing many status effect types and bulk operations.
    ///
    /// Returns `None` if no status effect with the given id was registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::{prelude::*, status_effect::GlobalStatusEffectId};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new status effect storage.
    ///     let storage = StatusEffectStorage::<V1_21_4>::new();
    ///
    ///     // Get the trait with the global id of `0`.
    ///     let effect = storage.get_trait(GlobalStatusEffectId::new_unchecked(0)).unwrap();
    ///     assert_eq!(effect.identifier(), "minecraft:speed");
    ///
    ///     // Get the trait with the global id of `1`.
    ///     let effect = storage.get_trait(GlobalStatusEffectId::new_unchecked(1)).unwrap();
    ///     assert_eq!(effect.identifier(), "minecraft:slowness");
    /// }
    /// ```
    #[must_use]
    pub fn get_trait(
        &self,
        effect_id: GlobalStatusEffectId,
    ) -> Option<&'static dyn StatusEffectTrait<V>> {
        self.0.get_index(effect_id).map(|val| val.inner())
    }

    /// Get a status effect for the given status effect id.
    ///
    /// Returns `None` if no status effect with the given id was registered,
    /// or the status effect does not exist in the resolver.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_common::vanilla::Vanilla;
    /// use froglight_entity::{prelude::*, status_effect::GlobalStatusEffectId};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///     use froglight_entity::status_effect::generated::v1_21_4::VersionStatusEffect;
    ///
    ///     // Create a new status effect storage.
    ///     let storage = StatusEffectStorage::<V1_21_4>::new();
    ///
    ///     // Get the status effect with the global id of `0`.
    ///     let effect = storage.get_typed::<Vanilla>(GlobalStatusEffectId::new_unchecked(0));
    ///     if let Some(VersionStatusEffect::Speed(speed)) = &effect {
    ///         assert_eq!(StatusEffectTrait::<V1_21_4>::identifier(speed), "minecraft:speed");
    ///     } else if effect.is_some() {
    ///         panic!("StatusEffect was not `Speed`, but {:?}!", effect.unwrap());
    ///     }
    ///
    ///     // Get the status effect with the global id of `5`.
    ///     let effect = storage.get_typed::<Vanilla>(GlobalStatusEffectId::new_unchecked(5));
    ///     if let Some(VersionStatusEffect::InstantHealth(instanthealth)) = &effect {
    ///         assert_eq!(
    ///             StatusEffectTrait::<V1_21_4>::identifier(instanthealth),
    ///             "minecraft:instant_health"
    ///         );
    ///     } else if effect.is_some() {
    ///         panic!("StatusEffect was not `InstantHealth`, but {:?}!", effect.unwrap());
    ///     }
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_typed<R: StatusEffectResolver<V>>(
        &self,
        effect_id: GlobalStatusEffectId,
    ) -> Option<R::EffectEnum> {
        self.get_trait(effect_id).and_then(R::resolve)
    }

    /// Get the [`GlobalStatusEffectId`] for the given status effect.
    ///
    /// Returns `None` if the status effect was not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::prelude::*;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new status effect storage.
    ///     let storage = StatusEffectStorage::<V1_21_4>::new();
    ///
    ///     // Get the `GlobalStatusEffectId` of `Speed`.
    ///     let global_id = storage.get_global_id::<effect::Speed>().unwrap();
    ///     assert_eq!(*global_id, 0);
    ///
    ///     // Get the `GlobalStatusEffectId` of `Slowness`.
    ///     let global_id = storage.get_global_id::<effect::Slowness>().unwrap();
    ///     assert_eq!(*global_id, 1);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn get_global_id<E: StatusEffectTrait<V>>(&self) -> Option<GlobalStatusEffectId> {
        self.get_global_type_id(&TypeId::of::<E>())
    }

    /// Get the [`GlobalStatusEffectId`] for the given status effect.
    ///
    /// Returns `None` if the status effect was not registered.
    #[must_use]
    pub fn get_global_type_id(&self, type_id: &TypeId) -> Option<GlobalStatusEffectId> {
        self.0.get_index_of(type_id)
    }

    /// Register a status effect with the storage.
    ///
    /// This is required for converting between global ids and status effect
    /// types.
    ///
    /// # Note
    /// The order in which status effects are registered is important.
    ///
    /// If a status effect is registered out of order, all following status
    /// effects will have their global ids shifted incorrectly.
    ///
    /// # Example
    ///
    /// ```rust
    /// use froglight_entity::{prelude::*, status_effect::GlobalStatusEffectId};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Create a new status effect storage with the vanilla effects registered.
    ///     let storage = StatusEffectStorage::<V1_21_4>::new();
    ///
    ///     // Since `Haste` is already registered, we can get its global id.
    ///     assert_eq!(
    ///         storage.get_global_id::<effect::Haste>(),
    ///         Some(GlobalStatusEffectId::new_unchecked(2))
    ///     );
    ///
    ///     // Create a new empty status effect storage.
    ///     let mut storage = StatusEffectStorage::<V1_21_4>::new_empty();
    ///
    ///     // Since `Haste` is not registered, it does not have a global id.
    ///     assert_eq!(storage.get_global_id::<effect::Haste>(), None);
    ///
    ///     // Register the `Haste` status effect, now we can get its global id.
    ///     storage.register::<effect::Haste>();
    ///     assert_eq!(
    ///         storage.get_global_id::<effect::Haste>(),
    ///         Some(GlobalStatusEffectId::new_unchecked(0))
    ///     );
    /// }
    /// ```
    pub fn register<E: StatusEffectExt<V>>(&mut self) {
        self.0.store(TypeId::of::<E>(), E::as_static());
    }
}

// -------------------------------------------------------------------------------------------------

impl<V: Version> Default for AppStatusEffectStorage<V>
where Vanilla: StatusEffectResolver<V>
{
    fn default() -> Self { Self::new() }
}

impl<V: Version> Default for StatusEffectStorage<V>
where Vanilla: StatusEffectResolver<V>
{
    fn default() -> Self { Self::new() }
}

// -------------------------------------------------------------------------------------------------

impl From<usize> for GlobalStatusEffectId {
    #[cfg(debug_assertions)]
    fn from(id: usize) -> Self {
        Self(u8::try_from(id).expect("GlobalStatusEffectId is too large!"))
    }

    #[inline]
    #[cfg(not(debug_assertions))]
    #[expect(clippy::cast_possible_truncation)]
    fn from(id: usize) -> Self { Self(id as u8) }
}

impl From<GlobalStatusEffectId> for usize {
    fn from(id: GlobalStatusEffectId) -> Self { usize::from(id.0) }
}
