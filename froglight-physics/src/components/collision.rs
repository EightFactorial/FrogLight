use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};

#[allow(unused_imports, reason = "May be used depending on features")]
use crate::prelude::*;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "bevy", derive(Component, Reflect))]
    #[cfg_attr(feature = "bevy", require(PreviousWorldCollision))]
    #[cfg_attr(feature = "bevy", reflect(opaque, Debug, Default, Clone, PartialEq, Hash, Component))]
    pub struct WorldCollision: u8 {
        const HORIZONTAL = 0b0000_0001;
        const VERTICAL = 0b0001_0000;
    }
}

impl Default for WorldCollision {
    #[inline]
    fn default() -> Self { Self::empty() }
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Hash, Component))]
pub struct PreviousWorldCollision(pub WorldCollision);

impl Deref for PreviousWorldCollision {
    type Target = WorldCollision;

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PreviousWorldCollision {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Borrow<WorldCollision> for PreviousWorldCollision {
    #[inline]
    fn borrow(&self) -> &WorldCollision { &self.0 }
}
impl BorrowMut<WorldCollision> for PreviousWorldCollision {
    #[inline]
    fn borrow_mut(&mut self) -> &mut WorldCollision { &mut self.0 }
}
