use bevy_app::App;
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<FontDefinition>(); }

/// A font definition
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Reflect)]
#[reflect(Default, Serialize, Deserialize)]

pub struct FontDefinition {}
