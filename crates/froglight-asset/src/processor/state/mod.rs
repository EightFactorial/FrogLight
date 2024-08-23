//! State processors for asset loading, processing, and spawning.

use bevy_app::App;

mod loading;

mod processing;
pub use processing::*;

mod spawning;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    loading::build(app);
    processing::build(app);
    spawning::build(app);
}
