//! Shader modules for rendering.

use bevy::prelude::*;

mod postprocess;
pub use postprocess::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { postprocess::build(app); }
