use bevy_app::App;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use froglight_components::resourcekey::ResourceKey;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<ParticleDefinition>(); }

/// A particle definition
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct ParticleDefinition {
    /// A list of textures that the particle can use
    pub textures: Vec<ResourceKey>,
}
