//! Interface systems and resources.

pub mod loading_screen;

pub mod uiscale;

/// The window scale width.
///
/// Used to calculate the [`UiScale`](bevy::ui::UiScale).
pub const SCALE_WIDTH: u32 = 380;

/// The window scale width as a `f32`.
///
/// When used as the width of a ui element, it
/// will never clip off the screen.
#[allow(clippy::cast_precision_loss)]
pub const SCALE_WIDTH_F32: f32 = SCALE_WIDTH as f32;

/// The window scale height.
///
/// Used to calculate the [`UiScale`](bevy::ui::UiScale).
pub const SCALE_HEIGHT: u32 = 240;

/// The window scale height as a `f32`.
///
/// When used as the height of a ui element, it
/// will never clip off the screen.
#[allow(clippy::cast_precision_loss)]
pub const SCALE_HEIGHT_F32: f32 = SCALE_HEIGHT as f32;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy::app::App) {
    loading_screen::build(app);
    uiscale::build(app);
}
