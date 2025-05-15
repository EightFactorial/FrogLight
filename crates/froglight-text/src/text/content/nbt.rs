#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_nbt::prelude::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A NBT value component.
#[derive(Debug, Clone, PartialEq, Deref, DerefMut, From, Into, FrogNbt)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct ValueComponent {
    /// The source of the data.
    #[frog(default, skip_if = Option::is_none)]
    pub source: Option<ValueComponentSource>,
}

/// A NBT value component source.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq))]
pub struct ValueComponentSource {
    /// The source of the data.
    pub source: ValueSourceKind,
    /// The path used to look up data from the source.
    pub path: Cow<'static, str>,

    /// Whether to interpret the returned NBT values as [`FormattedText`]s.
    pub interpret: bool,
    // /// The separator to use when displaying multiple tags.
    // pub separator: FormattedText,
}

/// The type and location of the data source.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
pub enum ValueSourceKind {
    /// Coordinates to the block entity which contains the data.
    Block(Cow<'static, str>),
    /// A selector used to query entities for data.
    Entity(Cow<'static, str>),
    /// The path to the command storage which contains the data.
    Storage(Cow<'static, str>),
}

// -------------------------------------------------------------------------------------------------

impl FromCompound for ValueComponentSource {
    fn from_compound(_compound: &NbtCompound) -> Result<Self, NbtError> { todo!() }
}

impl IntoCompound for ValueComponentSource {
    fn into_compound(&self) -> Result<NbtCompound, NbtError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl Serialize for ValueComponentSource {
    fn serialize<S>(&self, _ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        todo!()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ValueComponentSource {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
