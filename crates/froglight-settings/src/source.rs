use bevy::{
    asset::io::{AssetSource, AssetSourceId},
    prelude::*,
};

use crate::SettingsSource;

/// Register the asset source
#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    // Get the path to the settings file
    let path = app.world.resource::<SettingsSource>().path().to_str().unwrap();

    // Create the asset reader and writer
    let reader = AssetSource::get_default_reader(path.into());
    let writer = AssetSource::get_default_writer(path.into());

    // Create and register the asset source
    let source = AssetSource::build().with_reader(reader).with_writer(writer);
    app.register_asset_source(AssetSourceId::Name("frog".into()), source);
}
