#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;
#[cfg(all(not(feature = "std"), feature = "serde"))]
use alloc::string::String;
use core::str::FromStr;
#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::From;
use froglight_nbt::prelude::*;

mod integer;
pub use integer::IntegerColor;

mod preset;
pub use preset::PresetColor;

/// A color used for text styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum TextColor {
    /// A color represented as a hexadecimal string.
    Integer(IntegerColor),
    /// A color represented by its name.
    Preset(PresetColor),
}

impl TextColor {
    /// The default [`TextColor`] of a text's root component.
    pub const ROOT: Self = Self::Preset(PresetColor::White);

    /// Returns `true` if the color is a [`PresetColor`].
    #[must_use]
    pub const fn is_preset(&self) -> bool { matches!(self, Self::Preset(_)) }

    /// Returns `true` if the color is an [`IntegerColor`].
    #[must_use]
    pub const fn is_integer(&self) -> bool { matches!(self, Self::Integer(_)) }

    /// Returns the color as a name or hexadecimal string.
    #[must_use]
    pub fn as_named_string(&self) -> Cow<'static, str> {
        match self {
            Self::Integer(color) => Cow::Owned(color.as_hex_lower()),
            Self::Preset(color) => Cow::Borrowed(color.as_name()),
        }
    }

    /// Returns the color as a hexadecimal string.
    #[must_use]
    pub fn as_hex_string(&self) -> Cow<'static, str> {
        match self {
            Self::Integer(color) => Cow::Owned(color.as_hex_lower()),
            Self::Preset(color) => Cow::Borrowed(color.as_hex_lower()),
        }
    }

    /// Returns the color as a hexadecimal number.
    #[must_use]
    pub fn as_integer(&self) -> u32 {
        match self {
            Self::Integer(color) => **color,
            Self::Preset(color) => *IntegerColor::from_preset(color),
        }
    }

    /// Returns the color as an RGB tuple.
    #[must_use]
    pub fn as_rgb(&self) -> (u8, u8, u8) {
        let integer = self.as_integer();
        let r = ((integer >> 16) & 0xFF) as u8;
        let g = ((integer >> 8) & 0xFF) as u8;
        let b = (integer & 0xFF) as u8;
        (r, g, b)
    }
}

impl FromStr for TextColor {
    // TODO: Create a proper error type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') && s.len() == 7 {
            // Handle cases like `#0000FF`, `#123456`, etc.
            // Prefer using PresetColors over IntegerColors
            if let Ok(color) = PresetColor::try_from_hex(s) {
                Ok(Self::Preset(color))
            } else if let Ok(color) = IntegerColor::try_from_hex(&s[1..]) {
                Ok(Self::Integer(color))
            } else {
                Err(())
            }
        } else if let Ok(color) = PresetColor::try_from_name(s) {
            // Handle cases like `black`, `dark_blue`, etc.
            Ok(Self::Preset(color))
        } else {
            Err(())
        }
    }
}

// -------------------------------------------------------------------------------------------------

impl FromTag for TextColor {
    fn from_tag(_tag: &NbtTag) -> Result<Self, NbtError> { todo!() }
}

impl IntoTag for TextColor {
    fn into_tag(&self) -> Result<NbtTag, NbtError> { todo!() }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl serde::Serialize for TextColor {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        ser.serialize_str(&self.as_named_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for TextColor {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let s = String::deserialize(de)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("valid preset or hexadecimal color"))
    }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
