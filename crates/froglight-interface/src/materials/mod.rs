//! Custom [`Material`]s and [`UiMaterial`]s

use bevy::prelude::*;

mod gaussian_node;
pub use gaussian_node::GaussianNode;

#[doc(hidden)]
pub(crate) fn build(_app: &mut App) {
    // gaussian_node::build(app);
}

#[doc(hidden)]
pub(crate) fn finish(_app: &mut App) {
    // gaussian_node::finish(app);
}
