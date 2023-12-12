use std::{
    ffi::OsStr,
    io::{Read, Seek},
};

use bevy::{asset::LoadContext, prelude::*, utils::HashMap};
use mc_rs_core::ResourceLocation;
use zip::ZipArchive;

use crate::pack::{asset::model::Model, ResourcePackLoader, ResourcePackLoaderError};

/// Reads all models from the zip archive.
pub(crate) fn read_models(
    zip: &mut ZipArchive<impl Read + Seek>,
    load_context: &mut LoadContext,
) -> Result<HashMap<ResourceLocation, Model>, ResourcePackLoaderError> {
    #[cfg(any(debug_assertions, feature = "debug"))]
    trace!("Loading models: {}", load_context.asset_path());

    let mut models = HashMap::new();

    // Iterate over all files in the zip archive.
    for file_index in 0..zip.len() {
        let Ok(file) = zip.by_index(file_index) else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!(
                "Failed to read file {} in {}",
                file_index,
                load_context.asset_path()
            );
            continue;
        };

        // Skip all directories.
        if file.is_dir() {
            continue;
        }

        // Try to get the resource location from the file path.
        let Some(file_path) = file.enclosed_name() else {
            continue;
        };

        // Get the file extension.
        let Some(ext) = file_path
            .extension()
            .and_then(OsStr::to_str)
            .map(String::from)
        else {
            continue;
        };

        // Skip all files that are json models.
        if !matches!(ext.as_str(), "json") {
            continue;
        }

        // Get the resource location from the file path.
        let Some(key) = ResourcePackLoader::path_to_resource_location(file_path, "models") else {
            continue;
        };

        match serde_json::from_reader(file) {
            Ok(model) => {
                // Add the texture to the hashmap.
                models.insert(key, model);
            }
            Err(err) => {
                #[cfg(any(debug_assertions, feature = "debug"))]
                error!(
                    "Failed to parse model {} in {}: {}",
                    key,
                    load_context.asset_path(),
                    err
                );
                continue;
            }
        }
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    {
        trace!("{}:\n{models:#?}", load_context.asset_path());

        debug!(
            "Loaded {} models from {}",
            models.len(),
            load_context.asset_path()
        );
    }

    Ok(models)
}
