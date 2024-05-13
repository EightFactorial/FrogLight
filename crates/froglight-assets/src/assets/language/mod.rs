use bevy_app::App;
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{std_traits::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<LanguageFile>(); }

/// A language file
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut, Reflect,
)]
#[reflect(Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageFile(pub HashMap<String, String>);
