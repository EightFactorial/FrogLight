#[cfg(not(feature = "std"))]
use alloc::{format, string::String};
use core::num::ParseIntError;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::PresetColor;

/// A color represented by a [`u32`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IntegerColor(u32);

impl IntegerColor {
    /// Create an [`IntegerColor`] from a [`u32`].
    #[inline]
    #[must_use]
    pub const fn new(color: u32) -> Self { Self(color) }

    /// Create an [`IntegerColor`] from a [`PresetColor`].
    #[inline]
    #[must_use]
    pub const fn from_preset(preset: &PresetColor) -> Self { Self::new(preset.as_hex()) }

    /// Attempt to create an [`IntegerColor`] from a hexadecimal string.
    ///
    /// # Note
    /// This function does not work on strings that start with `#`.
    ///
    /// # Errors
    /// Returns an error if the string is not a valid hexadecimal number.
    pub const fn try_from_hex(string: &str) -> Result<Self, ParseIntError> {
        match u32::from_str_radix(string, 16) {
            Ok(color) => Ok(Self::new(color)),
            Err(e) => Err(e),
        }
    }

    /// Create a hexadecimal string from an [`IntegerColor`].
    #[must_use]
    pub fn as_hex_upper(&self) -> String { format!("#{:X}", self.0) }

    /// Create a hexadecimal string from an [`IntegerColor`].
    #[must_use]
    pub fn as_hex_lower(&self) -> String { format!("#{:x}", self.0) }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
