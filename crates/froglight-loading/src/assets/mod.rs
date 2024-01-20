//! Assets for the loading screen.
use bevy::{asset::embedded_asset, prelude::*};

#[doc(hidden)]
pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "loading_art.png");
}
