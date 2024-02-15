//! [`Events`](bevy::prelude::Event) used by all `FrogLight` crates.

use bevy::prelude::*;

mod resourcepack;
pub use resourcepack::{
    ResourcePackEndLoadingEvent, ResourcePackFinishLoadingEvent, ResourcePackStartLoadingEvent,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { resourcepack::build(app); }
