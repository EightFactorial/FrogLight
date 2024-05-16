use bevy::prelude::*;
use froglight_network::common::ResourceKey;
use hashbrown::HashMap;

mod event;
pub use event::ParticleEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<ParticleManager>()
        .register_type::<ParticleManager>()
        .add_event::<ParticleEvent>();
}

/// A [`Resource`] for managing particles.
#[derive(Debug, Default, Clone, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ParticleManager {
    /// Particles
    pub particles: HashMap<ResourceKey, ()>,
}
