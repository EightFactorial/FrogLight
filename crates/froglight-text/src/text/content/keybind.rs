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

/// A keybind component.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct KeybindComponent {
    /// The identifier of the keybind to display.
    #[frog(tag = "string")]
    pub keybind: Cow<'static, str>,
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
