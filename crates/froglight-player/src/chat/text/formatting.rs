use froglight_common::prelude::Identifier;
use smol_str::SmolStr;

/// The formatting of a [`Text`](super::Text) component.
#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(clippy::struct_excessive_bools)]
pub struct TextFormatting {
    /// The color of the text.
    color: TextColor,
    /// The font of the text.
    font: Identifier,
    /// Whether the text is bold.
    bold: bool,
    /// Whether the text is italic.
    italic: bool,
    /// Whether the text is underlined.
    underlined: bool,
    /// Whether the text is strikedthrough.
    strikethrough: bool,
    /// Whether the text is obfuscated.
    obfuscated: bool,
}

impl Default for TextFormatting {
    fn default() -> Self {
        Self {
            color: TextColor::White,
            font: Identifier::const_new("minecraft:default"),
            bold: false,
            italic: false,
            underlined: false,
            strikethrough: false,
            obfuscated: false,
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// The font color of a [`Text`](super::Text) component.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// use froglight_player::chat::text::TextColor;
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
        let color = color.into();
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
    /// use froglight_player::chat::text::TextColor;
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
        let color = color.into();

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
    /// use froglight_player::chat::text::TextColor;
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
    /// use froglight_player::chat::text::TextColor;
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
}
