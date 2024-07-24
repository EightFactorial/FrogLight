use bevy_asset::{Asset, ReflectAsset};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

/// A map of string keys to translations.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Reflect, Asset, Serialize, Deserialize, Deref, DerefMut,
)]
#[reflect(Default, Asset, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageMap {
    map: HashMap<String, String>,
}
