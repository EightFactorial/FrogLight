use bevy_ecs::prelude::*;
use bevy_reflect::FromType;
use froglight_common::prelude::Identifier;

/// A trait for synchronizing a main [`World`] with a [`SubWorld`]
pub trait SubWorldSync {
    /// Initialize a [`SubWorld`] using the main [`World`].
    fn initialize(ident: &Identifier, world: &mut World, sub: &mut World);
}

// -------------------------------------------------------------------------------------------------

/// A [`TypeData`](bevy_reflect::TypeData)-compatible
/// container for a [`SubWorldSync`] implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ReflectSubWorldSync {
    init_fn: fn(&Identifier, &mut World, &mut World),
}

impl<T: SubWorldSync> FromType<T> for ReflectSubWorldSync {
    fn from_type() -> Self { Self { init_fn: T::initialize } }
}

impl ReflectSubWorldSync {
    /// Initialize a [`SubWorld`] using the main [`World`].
    #[inline]
    pub fn init(&self, ident: &Identifier, world: &mut World, sub: &mut World) {
        (self.init_fn)(ident, world, sub);
    }
}
