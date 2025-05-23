#[cfg(feature = "bevy")]
use bevy_ecs::{bundle::Bundle, world::EntityWorldMut};
use downcast_rs::DowncastSync;
use froglight_common::{identifier::Identifier, version::Version};
use glam::Vec3;

/// A static entity type
pub trait StaticEntityType: 'static {
    /// Get a static reference to the entity type.
    ///
    /// Useful when working internally with generic entity types.
    ///
    /// ```rust
    /// use froglight_entity::entity_type::{StaticEntityType, generated::Cat};
    ///
    /// assert_eq!(Cat::as_static(), &Cat);
    /// ```
    fn as_static() -> &'static Self;
}

// -------------------------------------------------------------------------------------------------

/// An entity type.
pub trait EntityTypeTrait<V: Version>: DowncastSync + MaybeReflect {
    /// The identifier of the entity type.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity::Cat, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Cat as EntityTypeTrait<V1_21_4>>::identifier(Cat::as_static()),
    ///         "minecraft:cat"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityTypeExt` trait.
    ///     assert_eq!(<Cat as EntityTypeExt<V1_21_4>>::IDENTIFIER, "minecraft:cat");
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Cat as EntityTypeTrait<V1_21_5>>::identifier(Cat::as_static()),
    ///         "minecraft:cat"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityTypeExt` trait.
    ///     assert_eq!(<Cat as EntityTypeExt<V1_21_5>>::IDENTIFIER, "minecraft:cat");
    /// }
    /// ```
    fn identifier(&self) -> &'static Identifier;

    /// The spawn group of the entity type.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity::Cat, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Cat as EntityTypeTrait<V1_21_4>>::spawn_group(Cat::as_static()),
    ///         "minecraft:creature"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityTypeExt` trait.
    ///     assert_eq!(<Cat as EntityTypeExt<V1_21_4>>::SPAWN_GROUP, "minecraft:creature");
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Cat as EntityTypeTrait<V1_21_5>>::spawn_group(Cat::as_static()),
    ///         "minecraft:creature"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityTypeExt` trait.
    ///     assert_eq!(<Cat as EntityTypeExt<V1_21_5>>::SPAWN_GROUP, "minecraft:creature");
    /// }
    /// ```
    fn spawn_group(&self) -> &'static Identifier;

    /// The dimensions of the entity type.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity::Cat, *};
    /// use glam::Vec3;
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Cat as EntityTypeTrait<V1_21_4>>::dimensions(Cat::as_static()),
    ///         Vec3::new(0.6, 0.7, 0.35)
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityTypeExt` trait.
    ///     assert_eq!(<Cat as EntityTypeExt<V1_21_4>>::DIMENSIONS, Vec3::new(0.6, 0.7, 0.35));
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Cat as EntityTypeTrait<V1_21_5>>::dimensions(Cat::as_static()),
    ///         Vec3::new(0.6, 0.7, 0.35)
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityTypeExt` trait.
    ///     assert_eq!(<Cat as EntityTypeExt<V1_21_5>>::DIMENSIONS, Vec3::new(0.6, 0.7, 0.35));
    /// }
    /// ```
    fn dimensions(&self) -> Vec3;

    /// Whether the entity type is immune to fire.
    fn fire_immunity(&self) -> bool;

    /// Insert a [`Bundle`] of entity data into the given
    /// [`Entity`](bevy_ecs::entity::Entity).
    #[cfg(feature = "bevy")]
    fn insert_bundle(&self, entity: &mut EntityWorldMut);
}

// -------------------------------------------------------------------------------------------------

/// An extension of the [`EntityTypeTrait`] trait.
pub trait EntityTypeExt<V: Version>:
    EntityTypeTrait<V> + StaticEntityType + MaybeComponent
{
    /// The type of [`Bundle`] to insert into an
    /// [`Entity`](bevy_ecs::entity::Entity).
    #[cfg(feature = "bevy")]
    type BundleType: Bundle;
    /// The [`Bundle`] to insert into an [`Entity`](bevy_ecs::entity::Entity).
    #[cfg(feature = "bevy")]
    const BUNDLE: Self::BundleType;

    /// The identifier of the entity type.
    const IDENTIFIER: &'static str;
    /// The spawn group of the entity type.
    const SPAWN_GROUP: &'static str;

    /// The dimensions of the entity type.
    ///
    /// Where `x` and `y` are the width and height,
    /// and `z` is the eye-height.
    const DIMENSIONS: Vec3;
    /// Whether the entity type is immune to fire.
    const FIRE_IMMUNITY: bool;
}

// -------------------------------------------------------------------------------------------------

use sealed::{MaybeComponent, MaybeReflect};
mod sealed {
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

    #[cfg(feature = "bevy")]
    pub trait MaybeComponent: Component {}
    #[cfg(feature = "bevy")]
    impl<T: Component> MaybeComponent for T {}

    #[cfg(not(feature = "bevy"))]
    pub trait MaybeComponent {}
    #[cfg(not(feature = "bevy"))]
    impl<T> MaybeComponent for T {}
}
