use bevy_ecs::component::Component;
#[cfg(feature = "reflect")]
use bevy_ecs::reflect::ReflectComponent;
use bevy_math::Vec2;
#[cfg(feature = "reflect")]
use bevy_reflect::Reflect;
use derive_more::{Deref, DerefMut, From, Into};

/// Represents the size of an entity.
#[derive(Debug, Clone, Copy, Deref, DerefMut, From, Into, Component)]
#[cfg_attr(feature = "reflect", derive(Reflect))]
#[cfg_attr(feature = "reflect", reflect(Component))]
pub struct EntitySize(Vec2);

impl EntitySize {
    /// Creates a new [`EntitySize`].
    #[must_use]
    pub const fn new(width: f32, height: f32) -> Self { Self(Vec2::new(width, height)) }

    /// Returns the width of the entity.
    #[must_use]
    pub const fn width(&self) -> f32 { self.0.x }

    /// Returns the height of the entity.
    #[must_use]
    pub const fn height(&self) -> f32 { self.0.y }
}

impl From<(f32, f32)> for EntitySize {
    fn from((width, height): (f32, f32)) -> Self { Self::new(width, height) }
}
impl From<EntitySize> for (f32, f32) {
    fn from(value: EntitySize) -> Self { (value.width(), value.height()) }
}
