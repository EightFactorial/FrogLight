use yansi::{Color as YansiColor, Style as YansiStyle};

use crate::yansi::{Attribute, Style};

/// Enum representing a Minecraft color.
///
/// **Note:** The colors below are, unlike [`yansi`](YansiColor)'s, always the
/// same regardless of the terminal. This is because each color is represented
/// by hardcoded RGB values instead of terminal color codes.
///
/// See the [`minecraft.wiki`](https://minecraft.wiki/w/Formatting_codes) for more details on Minecraft's formatting codes.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash)]
pub enum Color {
    /// Minecraft's standard "black" color.
    ///
    /// Formatting code `§0` - Hexadecimal `#000000`
    Black,
    /// Minecraft's standard "dark_blue" color.
    ///
    /// Formatting code `§1` - Hexadecimal `#0000AA`
    DarkBlue,
    /// Minecraft's standard "dark_green" color.
    ///
    /// Formatting code `§2` - Hexadecimal `#00AA00`
    DarkGreen,
    /// Minecraft's standard "dark_aqua" color.
    ///
    /// Formatting code `§3` - Hexadecimal `#00AAAA`
    DarkAqua,
    /// Minecraft's standard "dark_red" color.
    ///
    /// Formatting code `§4` - Hexadecimal `#AA0000`
    DarkRed,
    /// Minecraft's standard "dark_purple" color.
    ///
    /// Formatting code `§5` - Hexadecimal `#AA00AA`
    DarkPurple,
    /// Minecraft's standard "gold" color.
    ///
    /// Formatting code `§6` - Hexadecimal `#FFAA00`
    Gold,
    /// Minecraft's standard "gray" color.
    ///
    /// Formatting code `§7` - Hexadecimal `#AAAAAA`
    Gray,
    /// Minecraft's standard "dark_gray" color.
    ///
    /// Formatting code `§8` - Hexadecimal `#555555`
    DarkGray,
    /// Minecraft's standard "blue" color.
    ///
    /// Formatting code `§9` - Hexadecimal `#5555FF`
    Blue,
    /// Minecraft's standard "green" color.
    ///
    /// Formatting code `§a` - Hexadecimal `#55FF55`
    Green,
    /// Minecraft's standard "aqua" color.
    ///
    /// Formatting code `§b` - Hexadecimal `#55FFFF`
    Aqua,
    /// Minecraft's standard "red" color.
    ///
    /// Formatting code `§c` - Hexadecimal `#FF5555`
    Red,
    /// Minecraft's standard "light_purple" color.
    ///
    /// Formatting code `§d` - Hexadecimal `#FF55FF`
    LightPurple,
    /// Minecraft's standard "yellow" color.
    ///
    /// Formatting code `§e` - Hexadecimal `#FFFF55`
    Yellow,
    /// Minecraft's standard "white" color.
    ///
    /// Formatting code `§f` - Hexadecimal `#FFFFFF`
    White,
}

impl Color {
    /// Returns a [`Style`] with a foreground color of `self`.
    ///
    /// See [`yansi::Color::foreground()`] for more details.
    #[must_use]
    pub const fn foreground(self) -> Style { Style(self.into_yansi().foreground()) }

    /// Returns a [`Style`] with a background color of `self`.
    ///
    /// See [`yansi::Color::background()`] for more details.
    #[must_use]
    pub const fn background(self) -> Style { Style(self.into_yansi().background()) }

    /// Returns a styled value derived from self with the background set to
    /// value.
    ///
    /// This method should be used rarely. Instead, prefer to use color-specific
    /// builder methods like `on_red()` and `on_green()`, which have the same
    /// functionality but are pithier.
    #[must_use]
    pub const fn bg(self, value: Color) -> Style { Style(self.into_yansi().bg(value.into_yansi())) }

    /// Convert this [`Color`] into a [`yansi::Color`].
    #[must_use]
    pub const fn into_yansi(self) -> YansiColor {
        match self {
            Self::Black => YansiColor::Rgb(0x00, 0x00, 0x00),
            Self::DarkBlue => YansiColor::Rgb(0x00, 0x00, 0xAA),
            Self::DarkGreen => YansiColor::Rgb(0x00, 0xAA, 0x00),
            Self::DarkAqua => YansiColor::Rgb(0x00, 0xAA, 0xAA),
            Self::DarkRed => YansiColor::Rgb(0xAA, 0x00, 0x00),
            Self::DarkPurple => YansiColor::Rgb(0xAA, 0x00, 0xAA),
            Self::Gold => YansiColor::Rgb(0xFF, 0xAA, 0x00),
            Self::Gray => YansiColor::Rgb(0xAA, 0xAA, 0xAA),
            Self::DarkGray => YansiColor::Rgb(0x55, 0x55, 0x55),
            Self::Blue => YansiColor::Rgb(0x55, 0x55, 0xFF),
            Self::Green => YansiColor::Rgb(0x55, 0xFF, 0x55),
            Self::Aqua => YansiColor::Rgb(0x55, 0xFF, 0xFF),
            Self::Red => YansiColor::Rgb(0xFF, 0x55, 0x55),
            Self::LightPurple => YansiColor::Rgb(0xFF, 0x55, 0xFF),
            Self::Yellow => YansiColor::Rgb(0xFF, 0xFF, 0x55),
            Self::White => YansiColor::Rgb(0xFF, 0xFF, 0xFF),
        }
    }
}

impl Color {
    /// Returns a [`Style`] with the foreground color set to
    /// [`Color::Primary`](YansiColor::Primary).
    #[expect(non_upper_case_globals, reason = "Mimic Enum Variant")]
    pub const Primary: Style = Style(YansiColor::Primary.foreground());

    /// Returns a [`Style`] with the foreground color set to
    /// [`Color::Fixed`](YansiColor::Fixed).
    #[must_use]
    pub const fn fixed(color: u8) -> Style { Style(YansiColor::Fixed(color).foreground()) }

    /// Returns a [`Style`] with the foreground color set to
    /// [`Color::Rgb`](YansiColor::Rgb).
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Style { Style(YansiColor::Rgb(r, g, b).foreground()) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::Primary`](YansiColor::Primary).
    #[must_use]
    pub const fn on_primary(self) -> Style { Style(self.into_yansi().on_primary()) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::Fixed`](YansiColor::Fixed).
    #[must_use]
    pub const fn on_fixed(self, color: u8) -> Style { Style(self.into_yansi().on_fixed(color)) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::Rgb`](YansiColor::Rgb).
    #[must_use]
    pub const fn on_rgb(self, r: u8, g: u8, b: u8) -> Style {
        Style(self.into_yansi().on_rgb(r, g, b))
    }

    // ---------------------------------------------------------------------------------------------

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Black`].
    #[must_use]
    pub const fn on_black(self) -> Style { self.bg(Self::Black) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkBlue`].
    #[must_use]
    pub const fn on_dark_blue(self) -> Style { self.bg(Self::DarkBlue) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkGreen`].
    #[must_use]
    pub const fn on_dark_green(self) -> Style { self.bg(Self::DarkGreen) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkAqua`].
    #[must_use]
    pub const fn on_dark_aqua(self) -> Style { self.bg(Self::DarkAqua) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkRed`].
    #[must_use]
    pub const fn on_dark_red(self) -> Style { self.bg(Self::DarkRed) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkPurple`].
    #[must_use]
    pub const fn on_dark_purple(self) -> Style { self.bg(Self::DarkPurple) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Gold`].
    #[must_use]
    pub const fn on_gold(self) -> Style { self.bg(Self::Gold) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Gray`].
    #[must_use]
    pub const fn on_gray(self) -> Style { self.bg(Self::Gray) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkGray`].
    #[must_use]
    pub const fn on_dark_gray(self) -> Style { self.bg(Self::DarkGray) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Blue`].
    #[must_use]
    pub const fn on_blue(self) -> Style { self.bg(Self::Blue) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Green`].
    #[must_use]
    pub const fn on_green(self) -> Style { self.bg(Self::Green) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Aqua`].
    #[must_use]
    pub const fn on_aqua(self) -> Style { self.bg(Self::Aqua) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Red`].
    #[must_use]
    pub const fn on_red(self) -> Style { self.bg(Self::Red) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::LightPurple`].
    #[must_use]
    pub const fn on_light_purple(self) -> Style { self.bg(Self::LightPurple) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Yellow`].
    #[must_use]
    pub const fn on_yellow(self) -> Style { self.bg(Self::Yellow) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::White`].
    #[must_use]
    pub const fn on_white(self) -> Style { self.bg(Self::White) }
}

impl Color {
    /// Enables the styling Attribute value.
    ///
    /// This method should be used rarely. Instead, prefer to use
    /// attribute-specific builder methods like `bold()` and `underline()`,
    /// which have the same functionality but are pithier.
    #[must_use]
    pub const fn attr(self, value: Attribute) -> Style { Style(self.into_yansi().attr(value)) }

    /// Makes text <b>bold</b>.
    ///
    /// Formatting code `§l`
    #[must_use]
    pub const fn bold(self) -> Style { self.attr(Attribute::Bold) }

    /// Display text with a <s>strike</s> through it.
    ///
    /// Formatting code `§m`
    #[must_use]
    pub const fn strike(self) -> Style { self.attr(Attribute::Strike) }

    /// <u>Underline</u> text.
    ///
    /// Formatting code `§n`
    #[must_use]
    pub const fn underline(self) -> Style { self.attr(Attribute::Underline) }

    /// Display text in <i>italics</i>.
    ///
    /// Formatting code `§o`
    #[must_use]
    pub const fn italic(self) -> Style { self.attr(Attribute::Italic) }
}

// -------------------------------------------------------------------------------------------------

impl From<Color> for YansiColor {
    #[inline]
    fn from(value: Color) -> Self { value.into_yansi() }
}

impl From<Color> for Style {
    #[inline]
    fn from(value: Color) -> Self { value.foreground() }
}
impl From<Color> for YansiStyle {
    #[inline]
    fn from(value: Color) -> Self { value.foreground().into_yansi() }
}
