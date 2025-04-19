//! [`TextFormatting`] and [`TextColor`]

#[cfg(feature = "bevy")]
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut, From, Into};
use froglight_common::prelude::Identifier;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// The formatting of a [`FormattedText`](super::FormattedText) component.
#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Deserialize, Serialize))]
pub struct TextFormatting {
    /// The font of the text.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    font: Option<Identifier>,
    /// The color of the text.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    color: Option<TextColor>,
    /// Whether the text is bold.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    bold: Option<bool>,
    /// Whether the text is italic.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    italic: Option<bool>,
    /// Whether the text is underlined.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    underlined: Option<bool>,
    /// Whether the text is strikedthrough.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    strikethrough: Option<bool>,
    /// Whether the text is obfuscated.
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "Option::is_none"))]
    obfuscated: Option<bool>,
}

// Is this a valid implementation? No, probably not.
impl PartialEq for TextFormatting {
    fn eq(&self, other: &Self) -> bool {
        let font = self.font == other.font || self.font.is_none() || other.font.is_none();
        let color = self.color == other.color || self.color.is_none() || other.color.is_none();
        let bold = self.bold == other.bold || self.bold.is_none() || other.bold.is_none();
        let italic = self.italic == other.italic || self.italic.is_none() || other.italic.is_none();

        let underlined = self.underlined == other.underlined
            || self.underlined.is_none()
            || other.underlined.is_none();

        let strikethrough = self.strikethrough == other.strikethrough
            || self.strikethrough.is_none()
            || other.strikethrough.is_none();

        let obfuscated = self.obfuscated == other.obfuscated
            || self.obfuscated.is_none()
            || other.obfuscated.is_none();

        font && color && bold && italic && underlined && strikethrough && obfuscated
    }
}

impl Default for TextFormatting {
    fn default() -> Self { Self::DEFAULT }
}

impl TextFormatting {
    /// [`TextFormatting`] with the default settings.
    pub const DEFAULT: Self = Self {
        font: Some(Self::DEFAULT_FONT),
        color: Some(Self::DEFAULT_COLOR),
        bold: Some(false),
        italic: Some(false),
        underlined: Some(false),
        strikethrough: Some(false),
        obfuscated: Some(false),
    };
    /// The default color used for text.
    pub const DEFAULT_COLOR: TextColor = TextColor::White;
    /// The default font used for text.
    pub const DEFAULT_FONT: Identifier = Identifier::const_new("minecraft:default");
    /// [`TextFormatting`] with no formatting.
    ///
    /// Should be used when inheriting from another [`TextFormatting`].
    pub const EMPTY: Self = Self {
        font: None,
        color: None,
        bold: None,
        italic: None,
        underlined: None,
        strikethrough: None,
        obfuscated: None,
    };

    /// [`TextFormatting`] with no formatting.
    ///
    /// Should be used when inheriting from another [`TextFormatting`].
    #[must_use]
    pub const fn empty() -> Self { Self::EMPTY }

    /// Create a new [`TextFormatting`] with all uninitialized fields
    /// set to the default values.
    #[must_use]
    pub fn or_default(&self) -> Self { self.inherit(&Self::default()) }

    /// Create a new [`TextFormatting`] that inherits from the given parent.
    ///
    /// This guarantees that all fields are initialized.
    #[must_use]
    pub fn inherit(&self, parent: &Self) -> Self {
        let font =
            self.font.as_ref().map_or_else(|| parent.font.clone(), |font| Some(font.clone()));
        let color =
            self.color.as_ref().map_or_else(|| parent.color.clone(), |color| Some(color.clone()));

        Self {
            font: font.or(Some(Self::DEFAULT_FONT)),
            color: color.or(Some(TextColor::White)),
            bold: self.bold.or(parent.bold).or(Some(false)),
            italic: self.italic.or(parent.italic).or(Some(false)),
            underlined: self.underlined.or(parent.underlined).or(Some(false)),
            strikethrough: self.strikethrough.or(parent.strikethrough).or(Some(false)),
            obfuscated: self.obfuscated.or(parent.obfuscated).or(Some(false)),
        }
    }

    /// Create a new [`TextFormatting`] returns only the differences.
    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        /// Returns `None` if both options are None or equal.
        fn xor<'a, T: PartialEq>(a: Option<&'a T>, b: Option<&'a T>) -> Option<&'a T> {
            match (a, b) {
                (Some(a), Some(b)) if a == b => None,
                _ => a.or(b),
            }
        }

        Self {
            font: xor(self.font.as_ref(), other.font.as_ref()).cloned(),
            color: xor(self.color.as_ref(), other.color.as_ref()).cloned(),
            bold: xor(self.bold.as_ref(), other.bold.as_ref()).copied(),
            italic: xor(self.italic.as_ref(), other.italic.as_ref()).copied(),
            underlined: xor(self.underlined.as_ref(), other.underlined.as_ref()).copied(),
            strikethrough: xor(self.strikethrough.as_ref(), other.strikethrough.as_ref()).copied(),
            obfuscated: xor(self.obfuscated.as_ref(), other.obfuscated.as_ref()).copied(),
        }
    }

    /// Set the font of the [`TextFormatting`].
    #[inline]
    #[must_use]
    pub fn with_font(mut self, font: Identifier) -> Self {
        self.font = Some(font);
        self
    }

    /// Set the color of the [`TextFormatting`].
    #[inline]
    #[must_use]
    pub fn with_color(mut self, color: TextColor) -> Self {
        self.color = Some(color);
        self
    }

    /// Set whether the [`TextFormatting`] is bold.
    #[inline]
    #[must_use]
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = Some(bold);
        self
    }

    /// Set whether the [`TextFormatting`] is italic.
    #[inline]
    #[must_use]
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = Some(italic);
        self
    }

    /// Set whether the [`TextFormatting`] is underlined.
    #[inline]
    #[must_use]
    pub fn with_underlined(mut self, underlined: bool) -> Self {
        self.underlined = Some(underlined);
        self
    }

    /// Set whether the [`TextFormatting`] is strikethrough.
    #[inline]
    #[must_use]
    pub fn with_strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    /// Set whether the [`TextFormatting`] is obfuscated.
    #[inline]
    #[must_use]
    pub fn with_obfuscated(mut self, obfuscated: bool) -> Self {
        self.obfuscated = Some(obfuscated);
        self
    }
}

// -------------------------------------------------------------------------------------------------

/// The font color of a [`FormattedText`](super::FormattedText) component.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
pub enum TextColor {
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
    /// A custom hexadecimal color.
    Custom(SmolStr),
}

impl TextColor {
    /// Creates a new [`TextColor`] from a named color or a hexadecimal color.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_player::chat::text::formatting::TextColor;
    ///
    /// let color = TextColor::from_color("black").unwrap();
    /// assert_eq!(color, TextColor::Black);
    ///
    /// let color = TextColor::from_color("#000000").unwrap();
    /// assert_eq!(color, TextColor::Black);
    ///
    /// let color = TextColor::from_color("#FF0000").unwrap();
    /// assert_eq!(color, TextColor::Custom("#FF0000".into()));
    ///
    /// // Invalid color
    /// assert_eq!(TextColor::from_color("invalid"), None);
    /// ```
    #[must_use]
    pub fn from_color(color: impl Into<SmolStr>) -> Option<Self> {
        let color: SmolStr = color.into();
        match color.as_str() {
            "black" => Some(Self::Black),
            "dark_blue" => Some(Self::DarkBlue),
            "dark_green" => Some(Self::DarkGreen),
            "dark_aqua" => Some(Self::DarkAqua),
            "dark_red" => Some(Self::DarkRed),
            "dark_purple" => Some(Self::DarkPurple),
            "gold" => Some(Self::Gold),
            "gray" => Some(Self::Gray),
            "dark_gray" => Some(Self::DarkGray),
            "blue" => Some(Self::Blue),
            "green" => Some(Self::Green),
            "aqua" => Some(Self::Aqua),
            "red" => Some(Self::Red),
            "light_purple" => Some(Self::LightPurple),
            "yellow" => Some(Self::Yellow),
            "white" => Some(Self::White),
            _ => Self::from_hex_string(color),
        }
    }

    /// Creates a new [`TextColor`] from a hexadecimal color.
    ///
    /// Returns `None` if the color is not a valid hexadecimal color.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_player::chat::text::formatting::TextColor;
    ///
    /// let color = TextColor::from_hex_string("#000000").unwrap();
    /// assert_eq!(color, TextColor::Black);
    ///
    /// let color = TextColor::from_hex_string("#0000AA").unwrap();
    /// assert_eq!(color, TextColor::DarkBlue);
    ///
    /// let color = TextColor::from_hex_string("#FFFFFF").unwrap();
    /// assert_eq!(color, TextColor::White);
    ///
    /// // Invalid color
    /// assert_eq!(TextColor::from_hex_string("invalid"), None);
    #[must_use]
    pub fn from_hex_string(color: impl Into<SmolStr>) -> Option<Self> {
        let color: SmolStr = color.into();

        if color.starts_with('#')
            && color.len() == 7
            && color.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        {
            Some(match color.as_str() {
                "#000000" => Self::Black,
                "#0000AA" | "#0000aa" => Self::DarkBlue,
                "#00AA00" | "#00aa00" => Self::DarkGreen,
                "#00AAAA" | "#00aaaa" => Self::DarkAqua,
                "#AA0000" | "#aa0000" => Self::DarkRed,
                "#AA00AA" | "#aa00aa" => Self::DarkPurple,
                "#FFAA00" | "#ffaa00" => Self::Gold,
                "#AAAAAA" | "#aaaaaa" => Self::Gray,
                "#555555" => Self::DarkGray,
                "#5555FF" | "#5555ff" => Self::Blue,
                "#55FF55" | "#55ff55" => Self::Green,
                "#55FFFF" | "#55ffff" => Self::Aqua,
                "#FF5555" | "#ff5555" => Self::Red,
                "#FF55FF" | "#ff55ff" => Self::LightPurple,
                "#FFFF55" | "#ffff55" => Self::Yellow,
                "#FFFFFF" | "#ffffff" => Self::White,
                _ => Self::Custom(color),
            })
        } else {
            None
        }
    }

    /// Returns the [`TextColor`] as a named string.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_player::chat::text::formatting::TextColor;
    ///
    /// assert_eq!(TextColor::Black.as_named_str(), "black");
    /// assert_eq!(TextColor::DarkBlue.as_named_str(), "dark_blue");
    /// assert_eq!(TextColor::DarkGreen.as_named_str(), "dark_green");
    /// assert_eq!(TextColor::DarkAqua.as_named_str(), "dark_aqua");
    /// assert_eq!(TextColor::White.as_named_str(), "white");
    ///
    /// let color = TextColor::from_hex_string("#FF0000").unwrap();
    /// assert_eq!(color.as_named_str(), "#FF0000");
    /// ```
    #[must_use]
    pub fn as_named_str(&self) -> &str {
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
            Self::Custom(color) => color.as_ref(),
        }
    }

    /// Returns the [`TextColor`] as a hexadecimal string.
    ///
    /// # Examples
    /// ```rust
    /// use froglight_player::chat::text::formatting::TextColor;
    ///
    /// assert_eq!(TextColor::Black.as_hex_str(), "#000000");
    /// assert_eq!(TextColor::DarkBlue.as_hex_str(), "#0000AA");
    /// assert_eq!(TextColor::DarkGreen.as_hex_str(), "#00AA00");
    /// assert_eq!(TextColor::DarkAqua.as_hex_str(), "#00AAAA");
    /// assert_eq!(TextColor::White.as_hex_str(), "#FFFFFF");
    ///
    /// let color = TextColor::from_hex_string("#FF0000").unwrap();
    /// assert_eq!(color.as_hex_str(), "#FF0000");
    /// ```
    #[must_use]
    pub fn as_hex_str(&self) -> &str {
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
            Self::Custom(color) => color.as_ref(),
        }
    }

    /// Returns the [`IntegerTextColor`] that represents this [`TextColor`].
    #[inline]
    #[must_use]
    pub fn as_integer(&self) -> IntegerTextColor { IntegerTextColor::from_color(self) }
}

// -------------------------------------------------------------------------------------------------

#[cfg(feature = "serde")]
impl Serialize for TextColor {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        ser.serialize_str(self.as_named_str())
    }
}
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for TextColor {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let string = String::deserialize::<D>(de)?;
        TextColor::from_color(string).ok_or_else(|| serde::de::Error::custom("invalid color"))
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`TextColor`] represented by a [`u32`].
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
#[cfg_attr(feature = "bevy", derive(Reflect), reflect(Debug, PartialEq, Hash))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(all(feature = "bevy", feature = "serde"), reflect(Serialize, Deserialize))]
pub struct IntegerTextColor(u32);

impl IntegerTextColor {
    /// Create a [`IntegerTextColor`] from a [`u32`].
    #[must_use]
    pub const fn new(color: u32) -> Self { Self(color) }

    /// Create a [`IntegerTextColor`] from a [`TextColor`].
    #[must_use]
    pub fn from_color(color: &TextColor) -> Self {
        if let Some(color) = color.as_hex_str().strip_prefix('#') {
            let color = u32::from_str_radix(color, 16);
            Self::new(
                color.unwrap_or_else(|_| unreachable!("TextColor contains invalid hexadecimal!")),
            )
        } else {
            unreachable!("TextColor always starts with a '#'!")
        }
    }

    /// Create a [`TextColor`] from an [`IntegerTextColor`].
    ///
    /// Returns `None` if the color is invalid.
    #[inline]
    #[must_use]
    pub fn try_into_color(&self) -> Option<TextColor> {
        TextColor::from_hex_string(format!("{:X}", self.0))
    }
}

// -------------------------------------------------------------------------------------------------

#[test]
fn inheritance() {
    let green = TextFormatting::empty().with_color(TextColor::DarkGreen);
    assert_eq!(green.or_default(), TextFormatting::default().with_color(TextColor::DarkGreen));

    let blue = TextFormatting::empty().with_color(TextColor::DarkBlue);
    assert_eq!(blue.or_default(), TextFormatting::default().with_color(TextColor::DarkBlue));

    let blue_bold = blue.with_bold(true);
    assert_eq!(
        blue_bold.or_default(),
        TextFormatting::default().with_color(TextColor::DarkBlue).with_bold(true)
    );

    let red_obfuscated =
        TextFormatting::empty().with_color(TextColor::DarkRed).with_obfuscated(true);
    assert_eq!(
        red_obfuscated.or_default(),
        TextFormatting::default().with_color(TextColor::DarkRed).with_obfuscated(true)
    );

    let red_obfuscated_italic = red_obfuscated.with_italic(true);
    assert_eq!(
        red_obfuscated_italic.or_default(),
        TextFormatting::default()
            .with_color(TextColor::DarkRed)
            .with_obfuscated(true)
            .with_italic(true)
    );
}

#[test]
fn text_color() {
    assert_eq!(serde_json::to_string(&TextColor::Aqua).unwrap(), "\"aqua\"");
    assert_eq!(serde_json::to_string(&TextColor::Black).unwrap(), "\"black\"");
    assert_eq!(serde_json::to_string(&TextColor::DarkBlue).unwrap(), "\"dark_blue\"");
    assert_eq!(serde_json::to_string(&TextColor::DarkGreen).unwrap(), "\"dark_green\"");
    assert_eq!(serde_json::to_string(&TextColor::Gold).unwrap(), "\"gold\"");
    assert_eq!(serde_json::to_string(&TextColor::LightPurple).unwrap(), "\"light_purple\"");
    assert_eq!(serde_json::to_string(&TextColor::Red).unwrap(), "\"red\"");
    assert_eq!(serde_json::to_string(&TextColor::Yellow).unwrap(), "\"yellow\"");

    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#000000\"").unwrap(), TextColor::Black);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#0000AA\"").unwrap(), TextColor::DarkBlue);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#00AA00\"").unwrap(), TextColor::DarkGreen);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#00AAAA\"").unwrap(), TextColor::DarkAqua);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#AA0000\"").unwrap(), TextColor::DarkRed);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#AAAAAA\"").unwrap(), TextColor::Gray);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#FFFF55\"").unwrap(), TextColor::Yellow);
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#FFFFFF\"").unwrap(), TextColor::White);

    let custom = TextColor::Custom("#123456".into());
    assert_eq!(serde_json::to_string(&custom).unwrap(), "\"#123456\"");
    assert_eq!(serde_json::from_str::<'_, TextColor>("\"#123456\"").unwrap(), custom);
}

#[test]
fn integer_color() {
    assert_eq!(*TextColor::Black.as_integer(), 0x000000);
    assert_eq!(*TextColor::DarkBlue.as_integer(), 0x0000AA);
    assert_eq!(*TextColor::DarkGreen.as_integer(), 0x00AA00);
    assert_eq!(*TextColor::DarkAqua.as_integer(), 0x00AAAA);
    assert_eq!(*TextColor::DarkRed.as_integer(), 0xAA0000);
    assert_eq!(*TextColor::DarkPurple.as_integer(), 0xAA00AA);
    assert_eq!(*TextColor::Gold.as_integer(), 0xFFAA00);
    assert_eq!(*TextColor::Gray.as_integer(), 0xAAAAAA);
    assert_eq!(*TextColor::DarkGray.as_integer(), 0x555555);
    assert_eq!(*TextColor::Blue.as_integer(), 0x5555FF);
    assert_eq!(*TextColor::Green.as_integer(), 0x55FF55);
    assert_eq!(*TextColor::Aqua.as_integer(), 0x55FFFF);
    assert_eq!(*TextColor::Red.as_integer(), 0xFF5555);
    assert_eq!(*TextColor::LightPurple.as_integer(), 0xFF55FF);
    assert_eq!(*TextColor::Yellow.as_integer(), 0xFFFF55);
    assert_eq!(*TextColor::White.as_integer(), 0xFFFFFF);
}
