//! TODO

use core::ops::{Deref, DerefMut};

use bevy_ecs::{entity::EntityHashSet, prelude::*};
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

/// A marker [`Component`] similar to [`Disabled`].
///
/// Temporarily disables [`SessionInstance`]s from being ticked by
/// [`TickSchedule`](crate::schedule::TickSchedule).
///
/// [`Disabled`]: bevy_ecs::entity_disabling::Disabled
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Debug, Default, Clone, PartialEq, Hash, Component)]
pub struct TickDisabled;

// -------------------------------------------------------------------------------------------------

/// The set of [`Entities`](Entity) that are temporarily disabled from being
/// ticked.
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Resource)]
#[reflect(Debug, Default, Clone, PartialEq, Resource)]
pub struct TickDisabledSet(EntityHashSet);

impl Deref for TickDisabledSet {
    type Target = EntityHashSet;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for TickDisabledSet {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
