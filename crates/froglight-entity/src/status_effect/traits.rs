use downcast_rs::DowncastSync;
use froglight_common::{identifier::Identifier, version::Version};
use froglight_text::prelude::IntegerColor;

use super::StatusEffectCategory;
use crate::maybe::MaybeReflect;

/// A static status effect
pub trait StaticStatusEffect: 'static {
    /// Get a static reference to the status effect.
    ///
    /// Useful when working internally with generic status effects.
    ///
    /// ```rust
    /// use froglight_entity::status_effect::{StaticStatusEffect, generated::Slowness};
    ///
    /// assert_eq!(Slowness::as_static(), &Slowness);
    /// ```
    fn as_static() -> &'static Self;
}

// -------------------------------------------------------------------------------------------------

/// An status effect.
pub trait StatusEffectTrait<V: Version>: DowncastSync + MaybeReflect {
    /// The identifier of the status effect.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{effect::Slowness, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static identifier through `StatusEffectTrait`.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectTrait<V1_21_4>>::identifier(Slowness::as_static()),
    ///         "minecraft:slowness"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `StatusEffectExt` trait.
    ///     assert_eq!(<Slowness as StatusEffectExt<V1_21_4>>::IDENTIFIER, "minecraft:slowness");
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static identifier through the `EntityType` trait.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectTrait<V1_21_5>>::identifier(Slowness::as_static()),
    ///         "minecraft:slowness"
    ///     );
    ///
    ///     // Accessing the constant identifier through the `StatusEffectExt` trait.
    ///     assert_eq!(<Slowness as StatusEffectExt<V1_21_5>>::IDENTIFIER, "minecraft:slowness");
    /// }
    /// ```
    fn identifier(&self) -> &'static Identifier;

    /// The color of the status effect.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{effect::Slowness, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static color through `StatusEffectTrait`.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectTrait<V1_21_4>>::color(Slowness::as_static()),
    ///         froglight_text::prelude::IntegerColor::new(9154528)
    ///     );
    ///
    ///     // Accessing the constant color through the `StatusEffectExt` trait.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectExt<V1_21_4>>::COLOR,
    ///         froglight_text::prelude::IntegerColor::new(9154528)
    ///     );
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static color through `StatusEffectTrait`.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectTrait<V1_21_5>>::color(Slowness::as_static()),
    ///         froglight_text::prelude::IntegerColor::new(9154528)
    ///     );
    ///
    ///     // Accessing the constant color through the `StatusEffectExt` trait.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectExt<V1_21_5>>::COLOR,
    ///         froglight_text::prelude::IntegerColor::new(9154528)
    ///     );
    /// }
    /// ```
    fn color(&self) -> IntegerColor;

    /// The [`StatusEffectCategory`] of the status effect.
    ///
    /// ```rust
    /// use froglight_entity::prelude::{effect::Slowness, *};
    ///
    /// #[cfg(feature = "v1_21_4")]
    /// {
    ///     use froglight_common::version::V1_21_4;
    ///
    ///     // Accessing the static category through `StatusEffectTrait`.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectTrait<V1_21_4>>::category(Slowness::as_static()),
    ///         StatusEffectCategory::Harmful
    ///     );
    ///
    ///     // Accessing the constant category through the `StatusEffectExt` trait.
    ///     assert_eq!(<Slowness as StatusEffectExt<V1_21_4>>::CATEGORY, StatusEffectCategory::Harmful);
    /// }
    ///
    /// #[cfg(feature = "v1_21_5")]
    /// {
    ///     use froglight_common::version::V1_21_5;
    ///
    ///     // Accessing the static category through `StatusEffectTrait`.
    ///     assert_eq!(
    ///         <Slowness as StatusEffectTrait<V1_21_5>>::category(Slowness::as_static()),
    ///         StatusEffectCategory::Harmful
    ///     );
    ///
    ///     // Accessing the constant category through the `StatusEffectExt` trait.
    ///     assert_eq!(<Slowness as StatusEffectExt<V1_21_5>>::CATEGORY, StatusEffectCategory::Harmful);
    /// }
    /// ```
    fn category(&self) -> StatusEffectCategory;
}

// -------------------------------------------------------------------------------------------------

/// An extension of the [`StatusEffectTrait`] trait.
pub trait StatusEffectExt<V: Version>: StatusEffectTrait<V> + StaticStatusEffect {
    /// The [`Identifier`] of the status effect.
    const IDENTIFIER: &'static str;

    /// The [`IntegerColor`] of the status effect.
    const COLOR: IntegerColor;
    /// The [`StatusEffectCategory`] of the status effect.
    const CATEGORY: StatusEffectCategory;
}
