#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;

#[cfg(feature = "reflect")]
pub trait MaybeReflect: Reflect {}
#[cfg(feature = "reflect")]
impl<T: Reflect> MaybeReflect for T {}

#[cfg(not(feature = "reflect"))]
pub trait MaybeReflect {}
#[cfg(not(feature = "reflect"))]
impl<T> MaybeReflect for T {}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "bevy")]
pub trait MaybeComponent: Component {}
#[cfg(feature = "bevy")]
impl<T: Component> MaybeComponent for T {}

#[cfg(not(feature = "bevy"))]
pub trait MaybeComponent {}
#[cfg(not(feature = "bevy"))]
impl<T> MaybeComponent for T {}
