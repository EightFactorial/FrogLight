use bevy::prelude::*;
use froglight_network::common::ResourceKey;

/// A sound event.
///
/// Plays the sound with the given [`ResourceKey`],
/// if it exists in the [`SoundManager`].
#[derive(Debug, Clone, PartialEq, Event)]
pub struct SoundEvent {
    /// The key of the sound to play.
    pub key: ResourceKey,

    /// The position of the sound.
    ///
    /// If `None`, the sound will be played at the listener's position.
    pub position: Option<Vec3>,
}
