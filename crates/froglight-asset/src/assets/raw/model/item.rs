use bevy_app::App;
use bevy_asset::{Asset, AssetApp, Handle, ReflectAsset, ReflectHandle};
use bevy_reflect::{prelude::ReflectDefault, Reflect, ReflectDeserialize, ReflectSerialize};
use serde::{Deserialize, Serialize};

use crate::assets::SerdeJsonLoader;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_asset::<ItemModelDefinition>();
    app.init_asset_loader::<SerdeJsonLoader<ItemModelDefinition>>();

    app.register_type::<ItemModelDefinition>()
        .register_type::<Handle<ItemModelDefinition>>()
        .register_type_data::<Handle<ItemModelDefinition>, ReflectHandle>();
}

/// A definition for an item model.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Asset, Reflect)]
#[reflect(Default, Asset, Serialize, Deserialize)]
pub struct ItemModelDefinition;
