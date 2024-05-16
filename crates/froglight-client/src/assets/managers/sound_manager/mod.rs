use bevy::prelude::*;
use froglight_assets::assets::SoundDefinitions;

mod event;
pub use event::SoundEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<SoundManager>().register_type::<SoundManager>().add_event::<SoundEvent>();
}

/// A [`Resource`] for managing sound effects.
#[derive(Debug, Default, Clone, Resource, Deref, DerefMut, Reflect)]
#[reflect(Default, Resource)]
pub struct SoundManager(pub SoundDefinitions);
