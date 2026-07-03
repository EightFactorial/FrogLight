use core::fmt;

use yansi::{Color as YansiColor, Painted as YansiPainted, Style as YansiStyle};

use crate::yansi::{Color, Style};

/// An arbitrary value with a [`Style`] applied to it.
///
/// See [`yansi::Painted`] for more details.
#[repr(transparent)]
pub struct Painted<T: Paint> {
    pub(super) painted: YansiPainted<T>,
}

impl<T: Paint> Painted<T> {
    /// Create a new [`Painted`] with a default [`Style`].
    #[inline]
    #[must_use]
    pub const fn new(value: T) -> Self { Self::wrap(YansiPainted::new(value)) }

    /// Create a new [`Painted`] from a [`yansi::Painted`].
    #[inline]
    #[must_use]
    pub const fn wrap(painted: YansiPainted<T>) -> Self { Self { painted } }

    /// Create a new [`Painted`] from a reference to the current value.
    #[inline]
    #[must_use]
    pub const fn to_ref(&self) -> Painted<&T> {
        Painted::wrap(YansiPainted { value: &self.painted.value, style: self.painted.style })
    }

    /// Get a reference to the inner [`yansi::Painted`].
    #[inline]
    #[must_use]
    pub const fn as_yansi(&self) -> &YansiPainted<T> { &self.painted }

    /// Convert this [`Painted`] into a [`yansi::Painted`].
    #[inline]
    #[must_use]
    pub fn into_yansi(self) -> YansiPainted<T> { self.painted }
}

impl<T: fmt::Display + Paint> fmt::Display for Painted<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <YansiPainted<T> as fmt::Display>::fmt(&self.painted, f)
    }
}
impl<T: fmt::Debug + Paint> fmt::Debug for Painted<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <YansiPainted<T> as fmt::Debug>::fmt(&self.painted, f)
    }
}

impl<T: Into<YansiPainted<T>>> From<T> for Painted<T> {
    #[inline]
    fn from(value: T) -> Self { Self::wrap(value.into()) }
}
impl<T: Paint> From<Painted<T>> for YansiStyle {
    #[inline]
    fn from(painted: Painted<T>) -> Self { painted.painted.style }
}

// -------------------------------------------------------------------------------------------------

/// A trait to apply styling to any value. Implemented for all types.
///
/// Because this trait is implemented for all types, you can use its methods on
/// any type. With the exception of one constructor method, [`Paint::new()`],
/// all methods are called with method syntax:
///
/// ```rust
/// use froglight_text::prelude::Paint;
///
/// "hello".green(); // calls `Paint::<&'static str>::green()`.
/// "hello".strike(); // calls `Paint::<&'static str>::strike()`.
/// 1i32.on_red(); // calls `Paint::<i32>::on_red()`.
/// ```
///
/// ### Further Details
///
/// See the [`yansi`](::yansi) docs for more details and examples.
pub trait Paint: ::yansi::Paint {
    /// Create a new [`Painted`] with a default [`Style`].
    ///
    /// See [`yansi::Paint::new()`] for more details.
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Code size")]
    #[allow(clippy::wrong_self_convention, reason = "Consistency")]
    fn new(self) -> Painted<Self>
    where
        Self: Sized,
    {
        Painted::new(self)
    }

    /// Apply a style wholesale to `self`. Any previous style is replaced.
    ///
    /// See [`yansi::Paint::paint()`] for more details.
    #[inline(always)]
    #[allow(clippy::inline_always, reason = "Code size")]
    fn paint<S: Into<Style>>(&self, style: S) -> Painted<&Self> {
        Painted::wrap(<Self as ::yansi::Paint>::paint(self, style.into()))
    }

    /// Returns a styled value derived from `self` with the foreground set to
    /// `value`.
    ///
    /// See [`yansi::Paint::fg()`] for more details.
    #[must_use]
    fn fg<C: Into<YansiColor>>(&self, value: C) -> Painted<&Self> {
        Painted::wrap(<Self as ::yansi::Paint>::fg(self, value.into()))
    }

    /// Returns a styled value derived from `self` with the foreground set to
    /// `value`.
    ///
    /// See [`yansi::Paint::bg()`] for more details.
    #[must_use]
    fn bg<C: Into<YansiColor>>(&self, value: C) -> Painted<&Self> {
        Painted::wrap(<Self as ::yansi::Paint>::bg(self, value.into()))
    }

    /// Returns `self` with the foreground color set to
    /// [`Color::Primary`](YansiColor::Primary).
    #[must_use]
    fn primary(&self) -> Painted<&Self> { Painted::wrap(::yansi::Paint::primary(self)) }

    /// Returns `self` with the foreground color set to
    /// [`Color::Fixed`](YansiColor::Fixed).
    #[must_use]
    fn fixed(&self, color: u8) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fixed(self, color))
    }

    /// Returns `self` with the foreground color set to
    /// [`Color::Rgb`](YansiColor::Rgb).
    #[must_use]
    fn rgb(&self, r: u8, g: u8, b: u8) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::rgb(self, r, g, b))
    }

    /// Returns `self` with the background color set to
    /// [`Color::Primary`](YansiColor::Primary).
    #[must_use]
    fn on_primary(&self) -> Painted<&Self> { Painted::wrap(::yansi::Paint::on_primary(self)) }

    /// Returns `self` with the background color set to
    /// [`Color::Fixed`](YansiColor::Fixed).
    #[must_use]
    fn on_fixed(&self, color: u8) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::on_fixed(self, color))
    }

    /// Returns `self` with the background color set to
    /// [`Color::Rgb`](YansiColor::Rgb).
    #[must_use]
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::on_rgb(self, r, g, b))
    }

    // ---------------------------------------------------------------------------------------------

    /// Returns `self` with the foreground color set to [`Color::Black`].
    #[must_use]
    fn black(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Black.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::DarkBlue`].
    #[must_use]
    fn dark_blue(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::DarkBlue.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::DarkGreen`].
    #[must_use]
    fn dark_green(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::DarkGreen.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::DarkAqua`].
    #[must_use]
    fn dark_aqua(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::DarkAqua.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::DarkRed`].
    #[must_use]
    fn dark_red(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::DarkRed.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::DarkPurple`].
    #[must_use]
    fn dark_purple(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::DarkPurple.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Gold`].
    #[must_use]
    fn gold(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Gold.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Gray`].
    #[must_use]
    fn gray(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Gray.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::DarkGray`].
    #[must_use]
    fn dark_gray(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::DarkGray.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Blue`].
    #[must_use]
    fn blue(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Blue.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Green`].
    #[must_use]
    fn green(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Green.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Aqua`].
    #[must_use]
    fn aqua(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Aqua.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Red`].
    #[must_use]
    fn red(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Red.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::LightPurple`].
    #[must_use]
    fn light_purple(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::LightPurple.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::Yellow`].
    #[must_use]
    fn yellow(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::Yellow.into_yansi()))
    }

    /// Returns `self` with the foreground color set to [`Color::White`].
    #[must_use]
    fn white(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::fg(self, Color::White.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Black`].
    #[must_use]
    fn on_black(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Black.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::DarkBlue`].
    #[must_use]
    fn on_dark_blue(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::DarkBlue.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::DarkGreen`].
    #[must_use]
    fn on_dark_green(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::DarkGreen.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::DarkAqua`].
    #[must_use]
    fn on_dark_aqua(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::DarkAqua.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::DarkRed`].
    #[must_use]
    fn on_dark_red(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::DarkRed.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::DarkPurple`].
    #[must_use]
    fn on_dark_purple(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::DarkPurple.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Gold`].
    #[must_use]
    fn on_gold(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Gold.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Gray`].
    #[must_use]
    fn on_gray(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Gray.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::DarkGray`].
    #[must_use]
    fn on_dark_gray(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::DarkGray.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Blue`].
    #[must_use]
    fn on_blue(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Blue.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Green`].
    #[must_use]
    fn on_green(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Green.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Aqua`].
    #[must_use]
    fn on_aqua(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Aqua.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Red`].
    #[must_use]
    fn on_red(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Red.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::LightPurple`].
    #[must_use]
    fn on_light_purple(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::LightPurple.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::Yellow`].
    #[must_use]
    fn on_yellow(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::Yellow.into_yansi()))
    }

    /// Returns `self` with the background color set to [`Color::White`].
    #[must_use]
    fn on_white(&self) -> Painted<&Self> {
        Painted::wrap(::yansi::Paint::bg(self, Color::White.into_yansi()))
    }

    // ---------------------------------------------------------------------------------------------

    /// Returns `self` with the text style set to bold.
    #[must_use]
    fn bold(&self) -> Painted<&Self> { Painted::wrap(::yansi::Paint::bold(self)) }

    /// Returns `self` with the text style set to strike-through.
    #[must_use]
    fn strike(&self) -> Painted<&Self> { Painted::wrap(::yansi::Paint::strike(self)) }

    /// Returns `self` with the text style set to underline.
    #[must_use]
    fn underline(&self) -> Painted<&Self> { Painted::wrap(::yansi::Paint::underline(self)) }

    /// Returns `self` with the text style set to italic.
    #[must_use]
    fn italic(&self) -> Painted<&Self> { Painted::wrap(::yansi::Paint::italic(self)) }
}

impl<T: ?Sized> Paint for T {}
