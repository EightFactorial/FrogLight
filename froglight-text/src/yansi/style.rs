use yansi::{Attribute, Color as YansiColor, Style as YansiStyle};

use crate::yansi::Color;

/// A set of styling options.
///
/// See [`yansi::Style`] for more details.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Style(pub(super) YansiStyle);

impl Style {
    const DEFAULT: Self = Self(YansiStyle::new());

    /// Returns a new style with no foreground or background, no attributes
    /// or quirks, and [`Condition::DEFAULT`].
    ///
    /// This is the default returned by [`Default::default()`].
    #[inline]
    #[must_use]
    pub const fn new() -> Self { Self::DEFAULT }

    /// Returns a new [`Style`] with the foreground color set to `color`.
    #[inline]
    #[must_use]
    pub const fn fg(self, color: Color) -> Self { self.fg_yansi(color.into_yansi()) }

    /// Returns a new [`Style`] with the background color set to `color`.
    #[inline]
    #[must_use]
    pub const fn bg(self, color: Color) -> Self { self.bg_yansi(color.into_yansi()) }

    /// Returns a new [`Style`] with the foreground color set to `color`.
    #[inline]
    #[must_use]
    pub const fn fg_yansi(self, color: YansiColor) -> Self { Self(self.0.fg(color)) }

    /// Returns a new [`Style`] with the background color set to `color`.
    #[inline]
    #[must_use]
    pub const fn bg_yansi(self, color: YansiColor) -> Self { Self(self.0.bg(color)) }

    /// Convert a [`yansi::Style`] into a [`Style`].
    #[inline]
    #[must_use]
    pub const fn from_yansi(style: YansiStyle) -> Self { Self(style) }

    /// Get a reference to the inner [`yansi::Style`].
    #[inline]
    #[must_use]
    pub const fn as_yansi(&self) -> &YansiStyle { &self.0 }

    /// Convert this [`Style`] into a [`yansi::Style`].
    #[inline]
    #[must_use]
    pub fn into_yansi(self) -> YansiStyle { self.0 }
}

impl Style {
    /// Returns a [`Style`] with the foreground color set to
    /// [`Color::Primary`](YansiColor::Primary).
    #[must_use]
    pub const fn primary() -> Style { Style(YansiColor::Primary.foreground()) }

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
    pub const fn on_primary(self) -> Style { Style(self.0.on_primary()) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::Fixed`](YansiColor::Fixed).
    #[must_use]
    pub const fn on_fixed(self, color: u8) -> Style { Style(self.0.on_fixed(color)) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::Rgb`](YansiColor::Rgb).
    #[must_use]
    pub const fn on_rgb(self, r: u8, g: u8, b: u8) -> Style { Style(self.0.on_rgb(r, g, b)) }

    // ---------------------------------------------------------------------------------------------

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Black`].
    #[must_use]
    pub const fn black(self) -> Style { self.fg(Color::Black) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::DarkBlue`].
    #[must_use]
    pub const fn dark_blue(self) -> Style { self.fg(Color::DarkBlue) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::DarkGreen`].
    #[must_use]
    pub const fn dark_green(self) -> Style { self.fg(Color::DarkGreen) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::DarkAqua`].
    #[must_use]
    pub const fn dark_aqua(self) -> Style { self.fg(Color::DarkAqua) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::DarkRed`].
    #[must_use]
    pub const fn dark_red(self) -> Style { self.fg(Color::DarkRed) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::DarkPurple`].
    #[must_use]
    pub const fn dark_purple(self) -> Style { self.fg(Color::DarkPurple) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Gold`].
    #[must_use]
    pub const fn gold(self) -> Style { self.fg(Color::Gold) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Gray`].
    #[must_use]
    pub const fn gray(self) -> Style { self.fg(Color::Gray) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::DarkGray`].
    #[must_use]
    pub const fn dark_gray(self) -> Style { self.fg(Color::DarkGray) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Blue`].
    #[must_use]
    pub const fn blue(self) -> Style { self.fg(Color::Blue) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Green`].
    #[must_use]
    pub const fn green(self) -> Style { self.fg(Color::Green) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Aqua`].
    #[must_use]
    pub const fn aqua(self) -> Style { self.fg(Color::Aqua) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Red`].
    #[must_use]
    pub const fn red(self) -> Style { self.fg(Color::Red) }

    /// Returns `self` with the [`fg()`](Self::fg) set to
    /// [`Color::LightPurple`].
    #[must_use]
    pub const fn light_purple(self) -> Style { self.fg(Color::LightPurple) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::Yellow`].
    #[must_use]
    pub const fn yellow(self) -> Style { self.fg(Color::Yellow) }

    /// Returns `self` with the [`fg()`](Self::fg) set to [`Color::White`].
    #[must_use]
    pub const fn white(self) -> Style { self.fg(Color::White) }

    // ---------------------------------------------------------------------------------------------

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Black`].
    #[must_use]
    pub const fn on_black(self) -> Style { self.bg(Color::Black) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkBlue`].
    #[must_use]
    pub const fn on_dark_blue(self) -> Style { self.bg(Color::DarkBlue) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkGreen`].
    #[must_use]
    pub const fn on_dark_green(self) -> Style { self.bg(Color::DarkGreen) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkAqua`].
    #[must_use]
    pub const fn on_dark_aqua(self) -> Style { self.bg(Color::DarkAqua) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkRed`].
    #[must_use]
    pub const fn on_dark_red(self) -> Style { self.bg(Color::DarkRed) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkPurple`].
    #[must_use]
    pub const fn on_dark_purple(self) -> Style { self.bg(Color::DarkPurple) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Gold`].
    #[must_use]
    pub const fn on_gold(self) -> Style { self.bg(Color::Gold) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Gray`].
    #[must_use]
    pub const fn on_gray(self) -> Style { self.bg(Color::Gray) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::DarkGray`].
    #[must_use]
    pub const fn on_dark_gray(self) -> Style { self.bg(Color::DarkGray) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Blue`].
    #[must_use]
    pub const fn on_blue(self) -> Style { self.bg(Color::Blue) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Green`].
    #[must_use]
    pub const fn on_green(self) -> Style { self.bg(Color::Green) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Aqua`].
    #[must_use]
    pub const fn on_aqua(self) -> Style { self.bg(Color::Aqua) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Red`].
    #[must_use]
    pub const fn on_red(self) -> Style { self.bg(Color::Red) }

    /// Returns `self` with the [`bg()`](Self::bg) set to
    /// [`Color::LightPurple`].
    #[must_use]
    pub const fn on_light_purple(self) -> Style { self.bg(Color::LightPurple) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::Yellow`].
    #[must_use]
    pub const fn on_yellow(self) -> Style { self.bg(Color::Yellow) }

    /// Returns `self` with the [`bg()`](Self::bg) set to [`Color::White`].
    #[must_use]
    pub const fn on_white(self) -> Style { self.bg(Color::White) }
}

impl Style {
    /// Enables the styling Attribute value.
    ///
    /// This method should be used rarely. Instead, prefer to use
    /// attribute-specific builder methods like `bold()` and `underline()`,
    /// which have the same functionality but are pithier.
    #[must_use]
    pub const fn attr(self, value: Attribute) -> Style { Style(self.0.attr(value)) }

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

impl From<Style> for YansiStyle {
    #[inline]
    fn from(style: Style) -> Self { style.0 }
}
impl From<YansiStyle> for Style {
    #[inline]
    fn from(style: YansiStyle) -> Self { Self(style) }
}
