use core::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[cfg(feature = "bevy")]
use bevy_ecs::{component::Component, reflect::ReflectComponent};
#[cfg(feature = "bevy")]
use bevy_reflect::{Reflect, std_traits::ReflectDefault};
// Note: Re-export from `bevy_transform`
pub use bevy_transform::components::Transform;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Component, Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Default, Clone, PartialEq, Component))]
pub struct PreviousTransform(pub Transform);

// -------------------------------------------------------------------------------------------------

impl AsRef<Transform> for PreviousTransform {
    fn as_ref(&self) -> &Transform { &self.0 }
}
impl AsMut<Transform> for PreviousTransform {
    fn as_mut(&mut self) -> &mut Transform { &mut self.0 }
}

impl Borrow<Transform> for PreviousTransform {
    fn borrow(&self) -> &Transform { &self.0 }
}
impl BorrowMut<Transform> for PreviousTransform {
    fn borrow_mut(&mut self) -> &mut Transform { &mut self.0 }
}

impl Deref for PreviousTransform {
    type Target = Transform;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl DerefMut for PreviousTransform {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<Transform> for PreviousTransform {
    fn from(value: Transform) -> Self { Self(value) }
}
impl From<PreviousTransform> for Transform {
    fn from(value: PreviousTransform) -> Self { value.0 }
}
