#[cfg(feature = "reflect")]
use bevy_reflect::prelude::*;
use froglight_text::prelude::PresetColor;

/// The category of a status effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum StatusEffectCategory {
    /// A beneficial status effect such as resistance or attack damage.
    Beneficial,
    /// A harmful status effect such as poison or slowness.
    Harmful,
    /// A neutral status effect such as glowing, bad omen, or trial omen.
    Neutral,
}

impl StatusEffectCategory {
    /// Returns `true` if the status effect is
    /// [`StatusEffectCategory::Beneficial`].
    #[inline]
    #[must_use]
    pub const fn is_beneficial(&self) -> bool { matches!(self, Self::Beneficial) }

    /// Returns `true` if the status effect is
    /// [`StatusEffectCategory::Harmful`].
    #[inline]
    #[must_use]
    pub const fn is_harmful(&self) -> bool { matches!(self, Self::Harmful) }

    /// Returns `true` if the status effect is
    /// [`StatusEffectCategory::Neutral`].
    #[inline]
    #[must_use]
    pub const fn is_neutral(&self) -> bool { matches!(self, Self::Neutral) }

    /// The [`PresetColor`] associated with the effect category.
    #[must_use]
    pub const fn formatting_color(&self) -> PresetColor {
        match self {
            Self::Harmful => PresetColor::Red,
            Self::Beneficial | Self::Neutral => PresetColor::Blue,
        }
    }
}
