use bevy::prelude::*;
use froglight_network::common::ResourceKey;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SoundManager>().add_event::<SoundEvent>();
}

/// Manages the sounds in the game.
#[derive(Debug, Default, Clone, Resource)]
pub struct SoundManager {
    // TODO: Collect sound definitions from all resource packs.
}

/// A sound event.
///
/// Plays the sound with the given [`ResourceKey`],
/// if it exists in the [`SoundManager`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event, Deref, DerefMut)]
pub struct SoundEvent(pub ResourceKey);
