use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use parking_lot::{Mutex, MutexGuard};

/// A [`World`] that can be used in a [`Function`].
#[expect(private_bounds)]
#[derive(Default, Clone, Reflect)]
#[reflect(opaque, from_reflect = false)]
pub struct WorldRef<RefType: WorldRefType = Full>(Arc<Mutex<World>>, PhantomData<RefType>);

trait WorldRefType: Copy + Reflect + Send + Sync + 'static {}

/// A [`WorldRef`] that contains a full [`World`].
#[derive(Clone, Copy, Reflect)]
pub struct Full;
impl WorldRefType for Full {}

/// A [`WorldRef`] that contains an empty [`World`].
#[derive(Default, Clone, Copy, Reflect)]
pub struct Empty;
impl WorldRefType for Empty {}

impl WorldRef<Empty> {
    /// Create a new [`WorldRef`]
    #[inline]
    #[must_use]
    pub fn new() -> WorldRef<Empty> { Self::default() }

    /// Temporarily use a [`World`] in a [`WorldRef`].
    pub fn with_world<R: Sized>(
        &mut self,
        world: &mut World,
        mut f: impl FnMut(&mut WorldRef<Full>) -> R,
    ) -> R {
        std::mem::swap(&mut *self.0.lock(), world);
        let result = f(&mut self.full());
        std::mem::swap(&mut *self.0.lock(), world);
        result
    }

    /// Mark a [`WorldRef`] as [`Full`].
    #[must_use]
    fn full(&self) -> WorldRef<Full> { WorldRef(self.0.clone(), PhantomData) }
}

impl WorldRef<Full> {
    /// Get a reference to the [`World`].
    #[must_use]
    pub fn value(&mut self) -> WorldValueRef<'_> { WorldValueRef(self.0.lock()) }
}

/// A guard for a [`World`] reference.
///
/// See [`WorldRef`] and [`MutexGuard`] for more information.
pub struct WorldValueRef<'a>(MutexGuard<'a, World>);

impl Deref for WorldValueRef<'_> {
    type Target = World;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for WorldValueRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
