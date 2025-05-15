#[cfg(not(feature = "std"))]
use alloc::{borrow::Cow, boxed::Box};
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_nbt::prelude::FrogNbt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A score component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ScoreComponent {
    /// The score objective to display.
    pub score: ScoreObjectiveComponent,
}

/// A scoreboard objective.
#[derive(Debug, Clone, PartialEq, Eq, Hash, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ScoreObjectiveComponent {
    /// The name of the score holder or a selector.
    #[frog(tag = "string")]
    pub name: Cow<'static, str>,
    /// The name of the score objective.
    #[frog(tag = "string")]
    pub objective: Cow<'static, str>,
}
