//! Interface systems and resources.

pub mod loading_screen;

pub mod uiscale;

/// The virtual window width.
///
/// All Ui should be designed for this width,
/// the scale will be adjusted to fit the actual window size.
pub const WINDOW_VIRTUAL_WIDTH: u32 = 380;

/// The virtual window height.
///
/// All Ui should be designed for this height,
/// the scale will be adjusted to fit the actual window size.
pub const WINDOW_VIRTUAL_HEIGHT: u32 = 240;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy::app::App) {
    loading_screen::build(app);
    uiscale::build(app);
}
