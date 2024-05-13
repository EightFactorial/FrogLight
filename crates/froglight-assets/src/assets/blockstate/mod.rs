use bevy_app::App;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

mod multipart;

mod variant;
pub use variant::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<BlockStateDefinition>(); }

/// A block state definition
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]
pub struct BlockStateDefinition {}
