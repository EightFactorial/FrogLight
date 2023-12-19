use std::{
    ffi::OsStr,
    io::{Read, Seek},
};

use bevy::{asset::LoadContext, prelude::*, utils::HashMap};
use mc_rs_core::ResourceLocation;
use zip::ZipArchive;

use crate::pack::{ResourcePackLoader, ResourcePackLoaderError};

/// Reads the sounds from a zip archive.
pub(crate) fn read_sounds(
    zip: &mut ZipArchive<impl Read + Seek>,
    load_context: &mut LoadContext,
) -> Result<HashMap<ResourceLocation, Handle<AudioSource>>, ResourcePackLoaderError> {
    #[cfg(any(debug_assertions, feature = "debug"))]
    trace!("Loading sounds: {}", load_context.asset_path());

    let mut sounds = HashMap::new();

    // Iterate over all files in the zip archive.
    for file_index in 0..zip.len() {
        let Ok(mut file) = zip.by_index(file_index) else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get file `{}` of `{}`",
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

        // Skip all files that are not audio files.
        if !matches!(ext.as_str(), "ogg" | "wav") {
            continue;
        }

        // Get the resource location from the file path.
        let Some(key) = ResourcePackLoader::path_to_resource_location(file_path, "sounds") else {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to get ResourceLocation from file path: {}",
                file_path.display()
            );
            continue;
        };

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        // Load the audio from the file.
        let sound = AudioSource {
            bytes: bytes.into(),
        };

        // Add the sound to the asset server.
        let handle = load_context.labeled_asset_scope(key.to_string(), |_| sound);

        // Add the texture to the hashmap.
        sounds.insert(key, handle);
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    {
        trace!("{}:\n{sounds:#?}", load_context.asset_path());

        debug!(
            "Loaded {} sounds from {}",
            sounds.len(),
            load_context.asset_path()
        );
    }

    Ok(sounds)
}
