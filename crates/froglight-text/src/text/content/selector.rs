#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, boxed::Box};
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use froglight_nbt::prelude::FrogNbt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An entity selector component.
#[derive(Debug, Clone, PartialEq, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct SelectorComponent {
    /// The selector used to identify the entities.
    #[frog(tag = "string")]
    pub selector: Cow<'static, str>,
    // /// The separator used when multiple selections are present.
    // #[frog(default = default_separator, tag = Compound)]
    // pub separator: FormattedText,
}
