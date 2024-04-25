//! Different kinds of assets

mod resourcepack;
pub use resourcepack::*;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { resourcepack::build(app); }
