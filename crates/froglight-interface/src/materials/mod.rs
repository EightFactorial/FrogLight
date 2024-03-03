//! Custom [`Material`]s and [`UiMaterial`]s

use bevy::prelude::*;

mod gaussian_node;
pub use gaussian_node::GaussianNode;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) { gaussian_node::build(app); }

#[doc(hidden)]
pub(crate) fn finish(app: &mut App) { gaussian_node::finish(app); }
