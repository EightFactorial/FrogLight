use core::ops::RangeInclusive;

use downcast_rs::DowncastSync;
use froglight_common::{identifier::Identifier, version::Version};

use crate::maybe::{MaybeComponent, MaybeReflect};

/// A static entity attribute
pub trait StaticEntityAttribute: 'static {
    /// Get a static reference to the entity attribute.
    ///
    /// Useful when working internally with generic attributes.
    ///
    /// ```rust
    /// use froglight_entity::entity_attribute::{StaticEntityAttribute, generated::AttackSpeed};
    ///
    /// assert_eq!(AttackSpeed::as_static(), &AttackSpeed);
    /// ```
    fn as_static() -> &'static Self;
}

// -------------------------------------------------------------------------------------------------

/// An entity attribute.
pub trait EntityAttributeTrait<V: Version>: DowncastSync + MaybeReflect {
    /// The identifier of the entity attribute.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity_attr::JumpStrength, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <JumpStrength as EntityAttributeTrait<V1_21_4>>::identifier(JumpStrength::as_static()),
    ///         "minecraft:jump_strength"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityAttributeExt` trait.
    ///     assert_eq!(
    ///         <JumpStrength as EntityAttributeExt<V1_21_4>>::IDENTIFIER,
    ///         "minecraft:jump_strength"
    ///     );
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <JumpStrength as EntityAttributeTrait<V1_21_5>>::identifier(JumpStrength::as_static()),
    ///         "minecraft:jump_strength"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `EntityAttributeExt` trait.
    ///     assert_eq!(
    ///         <JumpStrength as EntityAttributeExt<V1_21_5>>::IDENTIFIER,
    ///         "minecraft:jump_strength"
    ///     );
    /// }
    /// ```
    fn identifier(&self) -> &'static Identifier;

    /// The identifier of the entity attribute.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity_attr::Luck, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static key through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <Luck as EntityAttributeTrait<V1_21_4>>::translation_key(Luck::as_static()),
    ///         "minecraft.attribute.name.luck"
    ///     );
    ///
    ///     // Accessing the constant key through the `EntityAttributeExt` trait.
    ///     assert_eq!(
    ///         <Luck as EntityAttributeExt<V1_21_4>>::TRANSLATION_KEY,
    ///         "minecraft.attribute.name.luck"
    ///     );
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static key through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Luck as EntityAttributeTrait<V1_21_5>>::translation_key(Luck::as_static()),
    ///         "minecraft.attribute.name.luck"
    ///     );
    ///
    ///     // Accessing the constant key through the `EntityAttributeExt` trait.
    ///     assert_eq!(
    ///         <Luck as EntityAttributeExt<V1_21_5>>::TRANSLATION_KEY,
    ///         "minecraft.attribute.name.luck"
    ///     );
    /// }
    /// ```
    fn translation_key(&self) -> &'static str;

    /// The default value of the entity attribute.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity_attr::Gravity, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static default through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <Gravity as EntityAttributeTrait<V1_21_4>>::default_value(Gravity::as_static()),
    ///         0.08
    ///     );
    ///
    ///     // Accessing the constant default through the `EntityAttributeExt` trait.
    ///     assert_eq!(<Gravity as EntityAttributeExt<V1_21_4>>::DEFAULT, 0.08);
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static default through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <Gravity as EntityAttributeTrait<V1_21_5>>::default_value(Gravity::as_static()),
    ///         0.08
    ///     );
    ///
    ///     // Accessing the constant default through the `EntityAttributeExt` trait.
    ///     assert_eq!(<Gravity as EntityAttributeExt<V1_21_5>>::DEFAULT, 0.08);
    /// }
    /// ```
    fn default_value(&self) -> f64;

    /// The valid range of values for the entity attribute.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{entity_attr::Armor, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static range through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <Armor as EntityAttributeTrait<V1_21_4>>::valid_range(Armor::as_static()),
    ///         0.0..=30.0
    ///     );
    ///
    ///     // Accessing the constant range through the `EntityAttributeExt` trait.
    ///     assert_eq!(<Armor as EntityAttributeExt<V1_21_4>>::RANGE, 0.0..=30.0);
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static range through `EntityAttributeTrait`.
    ///     assert_eq!(
    ///         <Armor as EntityAttributeTrait<V1_21_5>>::valid_range(Armor::as_static()),
    ///         0.0..=30.0
    ///     );
    ///
    ///     // Accessing the constant range through the `EntityAttributeExt` trait.
    ///     assert_eq!(<Armor as EntityAttributeExt<V1_21_5>>::RANGE, 0.0..=30.0);
    /// }
    /// ```
    fn valid_range(&self) -> RangeInclusive<f64>;
}

// -------------------------------------------------------------------------------------------------

/// An extension of the [`EntityAttributeTrait`] trait.
pub trait EntityAttributeExt<V: Version>:
    EntityAttributeTrait<V> + StaticEntityAttribute + MaybeComponent
{
    /// The [`Identifier`] of the entity attribute.
    const IDENTIFIER: &'static str;
    /// The translation key for the entity attribute.
    const TRANSLATION_KEY: &'static str;

    /// The default value of the attribute.
    const DEFAULT: f64;
    /// The range of valid values for the attribute.
    const RANGE: RangeInclusive<f64>;
}
