use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut};

/// A counter for the current tick.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Resource, Reflect, Deref, DerefMut)]
#[reflect(Debug, Default, PartialEq, Hash, Resource)]
pub struct CurrentTick(u128);

impl CurrentTick {
    /// A [`System`] that increments the [`CurrentTick`].
    pub fn increment_tick(mut tick: ResMut<Self>) { tick.increment(); }

    /// Increment the [`CurrentTick`].
    pub fn increment(&mut self) { self.0 = self.0.wrapping_add(1); }
}
