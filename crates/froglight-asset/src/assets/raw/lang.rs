//! [`SingleLanguageMap`] and related types.

use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_derive::{Deref, DerefMut};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

use crate::assets::SerdeJsonLoader;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<SingleLanguageMap>();
    app.init_asset_loader::<SerdeJsonLoader<SingleLanguageMap>>();

    app.register_type::<SingleLanguageMap>()
        .register_type::<Handle<SingleLanguageMap>>()
        .register_type_data::<Handle<SingleLanguageMap>, ReflectHandle>();
}

/// A map of strings for a single language.
#[derive(
    Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Serialize, Deserialize, Asset, Reflect,
)]
#[reflect(Default, Serialize, Deserialize, Asset)]
#[serde(transparent)]
pub struct SingleLanguageMap(HashMap<String, String>);

impl From<HashMap<String, String>> for SingleLanguageMap {
    fn from(map: HashMap<String, String>) -> Self { SingleLanguageMap(map) }
}
impl From<SingleLanguageMap> for HashMap<String, String> {
    fn from(map: SingleLanguageMap) -> Self { map.0 }
}
