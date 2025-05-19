use core::str::FromStr;

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;

/// A set of predefined colors for text styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, Clone, PartialEq, Hash))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub enum PresetColor {
    /// Hex `#000000`
    Black,
    /// Hex `#0000AA`
    DarkBlue,
    /// Hex `#00AA00`
    DarkGreen,
    /// Hex `#00AAAA`
    DarkAqua,
    /// Hex `#AA0000`
    DarkRed,
    /// Hex `#AA00AA`
    DarkPurple,
    /// Hex `#FFAA00`
    Gold,
    /// Hex `#AAAAAA`
    Gray,
    /// Hex `#555555`
    DarkGray,
    /// Hex `#5555FF`
    Blue,
    /// Hex `#55FF55`
    Green,
    /// Hex `#55FFFF`
    Aqua,
    /// Hex `#FF5555`
    Red,
    /// Hex `#FF55FF`
    LightPurple,
    /// Hex `#FFFF55`
    Yellow,
    /// Hex `#FFFFFF`
    White,
}

impl PresetColor {
    /// Get the [`PresetColor`] from a string.
    ///
    /// # Errors
    /// Returns the input if it does not match any of the preset colors.
    pub fn try_from_name(string: &str) -> Result<Self, &str> {
        match string {
            "black" => Ok(Self::Black),
            "dark_blue" => Ok(Self::DarkBlue),
            "dark_green" => Ok(Self::DarkGreen),
            "dark_aqua" => Ok(Self::DarkAqua),
            "dark_red" => Ok(Self::DarkRed),
            "dark_purple" => Ok(Self::DarkPurple),
            "gold" => Ok(Self::Gold),
            "gray" => Ok(Self::Gray),
            "dark_gray" => Ok(Self::DarkGray),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            "aqua" => Ok(Self::Aqua),
            "red" => Ok(Self::Red),
            "light_purple" => Ok(Self::LightPurple),
            "yellow" => Ok(Self::Yellow),
            "white" => Ok(Self::White),
            other => Err(other),
        }
    }

    /// Get the [`PresetColor`] as a string.
    #[must_use]
    pub const fn as_name(&self) -> &'static str {
        match self {
            Self::Black => "black",
            Self::DarkBlue => "dark_blue",
            Self::DarkGreen => "dark_green",
            Self::DarkAqua => "dark_aqua",
            Self::DarkRed => "dark_red",
            Self::DarkPurple => "dark_purple",
            Self::Gold => "gold",
            Self::Gray => "gray",
            Self::DarkGray => "dark_gray",
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Aqua => "aqua",
            Self::Red => "red",
            Self::LightPurple => "light_purple",
            Self::Yellow => "yellow",
            Self::White => "white",
        }
    }

    /// Get the [`PresetColor`] from a decimal number.
    ///
    /// Returns `None` if the number does not match any of the presets.
    #[must_use]
    #[expect(clippy::unreadable_literal)]
    pub const fn try_from_decimal(color: u32) -> Option<Self> {
        match color {
            0x000000 => Some(Self::Black),
            0x0000AA => Some(Self::DarkBlue),
            0x00AA00 => Some(Self::DarkGreen),
            0x00AAAA => Some(Self::DarkAqua),
            0xAA0000 => Some(Self::DarkRed),
            0xAA00AA => Some(Self::DarkPurple),
            0xFFAA00 => Some(Self::Gold),
            0xAAAAAA => Some(Self::Gray),
            0x555555 => Some(Self::DarkGray),
            0x5555FF => Some(Self::Blue),
            0x55FF55 => Some(Self::Green),
            0x55FFFF => Some(Self::Aqua),
            0xFF5555 => Some(Self::Red),
            0xFF55FF => Some(Self::LightPurple),
            0xFFFF55 => Some(Self::Yellow),
            0xFFFFFF => Some(Self::White),
            _ => None,
        }
    }

    /// Get the [`PresetColor`] as a decimal number.
    #[must_use]
    #[expect(clippy::unreadable_literal)]
    pub const fn as_decimal(self) -> u32 {
        match self {
            Self::Black => 0x000000,
            Self::DarkBlue => 0x0000AA,
            Self::DarkGreen => 0x00AA00,
            Self::DarkAqua => 0x00AAAA,
            Self::DarkRed => 0xAA0000,
            Self::DarkPurple => 0xAA00AA,
            Self::Gold => 0xFFAA00,
            Self::Gray => 0xAAAAAA,
            Self::DarkGray => 0x555555,
            Self::Blue => 0x5555FF,
            Self::Green => 0x55FF55,
            Self::Aqua => 0x55FFFF,
            Self::Red => 0xFF5555,
            Self::LightPurple => 0xFF55FF,
            Self::Yellow => 0xFFFF55,
            Self::White => 0xFFFFFF,
        }
    }

    /// Get the [`PresetColor`] from a hex string.
    ///
    /// # Errors
    /// Returns the input if it does not match any of the preset colors.
    pub fn try_from_hex(string: &str) -> Result<Self, &str> {
        match string {
            "#000000" => Ok(Self::Black),
            "#0000AA" | "#0000aa" => Ok(Self::DarkBlue),
            "#00AA00" | "#00aa00" => Ok(Self::DarkGreen),
            "#00AAAA" | "#00aaaa" => Ok(Self::DarkAqua),
            "#AA0000" | "#aa0000" => Ok(Self::DarkRed),
            "#AA00AA" | "#aa00aa" => Ok(Self::DarkPurple),
            "#FFAA00" | "#ffaa00" => Ok(Self::Gold),
            "#AAAAAA" | "#aaaaaa" => Ok(Self::Gray),
            "#555555" => Ok(Self::DarkGray),
            "#5555FF" | "#5555ff" => Ok(Self::Blue),
            "#55FF55" | "#55ff55" => Ok(Self::Green),
            "#55FFFF" | "#55ffff" => Ok(Self::Aqua),
            "#FF5555" | "#ff5555" => Ok(Self::Red),
            "#FF55FF" | "#ff55ff" => Ok(Self::LightPurple),
            "#FFFF55" | "#ffff55" => Ok(Self::Yellow),
            "#FFFFFF" | "#ffffff" => Ok(Self::White),
            other => Err(other),
        }
    }

    /// Get the [`PresetColor`] as a hex string.
    #[must_use]
    pub const fn as_hex_upper(&self) -> &'static str {
        match self {
            Self::Black => "#000000",
            Self::DarkBlue => "#0000AA",
            Self::DarkGreen => "#00AA00",
            Self::DarkAqua => "#00AAAA",
            Self::DarkRed => "#AA0000",
            Self::DarkPurple => "#AA00AA",
            Self::Gold => "#FFAA00",
            Self::Gray => "#AAAAAA",
            Self::DarkGray => "#555555",
            Self::Blue => "#5555FF",
            Self::Green => "#55FF55",
            Self::Aqua => "#55FFFF",
            Self::Red => "#FF5555",
            Self::LightPurple => "#FF55FF",
            Self::Yellow => "#FFFF55",
            Self::White => "#FFFFFF",
        }
    }

    /// Get the [`PresetColor`] as a hex string.
    #[must_use]
    pub const fn as_hex_lower(&self) -> &'static str {
        match self {
            Self::Black => "#000000",
            Self::DarkBlue => "#0000aa",
            Self::DarkGreen => "#00aa00",
            Self::DarkAqua => "#00aaaa",
            Self::DarkRed => "#aa0000",
            Self::DarkPurple => "#aa00aa",
            Self::Gold => "#ffaa00",
            Self::Gray => "#aaaaaa",
            Self::DarkGray => "#555555",
            Self::Blue => "#5555ff",
            Self::Green => "#55ff55",
            Self::Aqua => "#55ffff",
            Self::Red => "#ff5555",
            Self::LightPurple => "#ff55ff",
            Self::Yellow => "#ffff55",
            Self::White => "#ffffff",
        }
    }

    /// Get the [`PresetColor`] as a hexadecimal number.
    #[must_use]
    #[expect(clippy::unreadable_literal)]
    pub const fn as_hex(&self) -> u32 {
        match self {
            PresetColor::Black => 0x000000,
            PresetColor::DarkBlue => 0x0000AA,
            PresetColor::DarkGreen => 0x00AA00,
            PresetColor::DarkAqua => 0x00AAAA,
            PresetColor::DarkRed => 0xAA0000,
            PresetColor::DarkPurple => 0xAA00AA,
            PresetColor::Gold => 0xFFAA00,
            PresetColor::Gray => 0xAAAAAA,
            PresetColor::DarkGray => 0x555555,
            PresetColor::Blue => 0x5555FF,
            PresetColor::Green => 0x55FF55,
            PresetColor::Aqua => 0x55FFFF,
            PresetColor::Red => 0xFF5555,
            PresetColor::LightPurple => 0xFF55FF,
            PresetColor::Yellow => 0xFFFF55,
            PresetColor::White => 0xFFFFFF,
        }
    }
}

impl FromStr for PresetColor {
    // TODO: Create a proper error type
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') && s.len() == 7 {
            Self::try_from_hex(s).map_err(|_| ())
        } else {
            Self::try_from_name(s).map_err(|_| ())
        }
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl serde::Serialize for PresetColor {
    fn serialize<S>(&self, _ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        todo!()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PresetColor {
    fn deserialize<D>(_de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        todo!()
    }
}

// -------------------------------------------------------------------------------------------------
//
// TODO: Tests
