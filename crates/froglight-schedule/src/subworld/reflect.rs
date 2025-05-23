use bevy_ecs::{prelude::*, world::DeferredWorld};
use bevy_reflect::FromType;
use froglight_common::prelude::Identifier;

/// A trait for synchronizing a main [`World`] with a
/// [`SubWorld`](super::SubWorld).
pub trait SubWorldSync {
    /// Initialize a [`SubWorld`](super::SubWorld) using the main [`World`].
    fn initialize(ident: &Identifier, world: &mut World, sub: &mut World);

    /// A function to run when adding a [`SubWorld`](super::SubWorld) to an
    /// [`Entity`]. The [`Entity`] is guaranteed to have a
    /// [`SubWorld`](super::SubWorld) [`Component`].
    ///
    /// This also runs when the [`Entity`] is spawned.
    #[expect(unused_variables)]
    fn on_add(entity: Entity, world: &mut DeferredWorld) {}

    /// A function to run when removing a [`SubWorld`](super::SubWorld) from an
    /// [`Entity`]. The [`Entity`] is guaranteed to have a
    /// [`SubWorld`](super::SubWorld) [`Component`].
    ///
    /// This also runs when the [`Entity`] is despawned.
    #[expect(unused_variables)]
    fn on_remove(entity: Entity, world: &mut DeferredWorld) {}
}

// -------------------------------------------------------------------------------------------------

/// A [`TypeData`](bevy_reflect::TypeData)-compatible
/// container for a [`SubWorldSync`] implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[expect(clippy::struct_field_names)]
pub struct ReflectSubWorldSync {
    init_fn: fn(&Identifier, &mut World, &mut World),
    add_fn: fn(Entity, &mut DeferredWorld),
    remove_fn: fn(Entity, &mut DeferredWorld),
}

impl<T: SubWorldSync> FromType<T> for ReflectSubWorldSync {
    fn from_type() -> Self {
        Self { init_fn: T::initialize, add_fn: T::on_add, remove_fn: T::on_remove }
    }
}

impl ReflectSubWorldSync {
    /// Initialize a [`SubWorld`](super::SubWorld) using the main [`World`].
    #[inline]
    pub fn init(&self, ident: &Identifier, world: &mut World, sub: &mut World) {
        (self.init_fn)(ident, world, sub);
    }

    /// A function to run when adding a [`SubWorld`](super::SubWorld) to an
    /// [`Entity`].
    ///
    /// This also runs when the [`Entity`] is spawned.
    #[inline]
    pub fn on_add(&self, entity: Entity, world: &mut DeferredWorld) {
        (self.add_fn)(entity, world);
    }

    /// A function to run when removing a [`SubWorld`](super::SubWorld) from an
    /// [`Entity`].
    ///
    /// This also runs when the [`Entity`] is despawned.
    #[inline]
    pub fn on_remove(&self, entity: Entity, world: &mut DeferredWorld) {
        (self.remove_fn)(entity, world);
    }
}
