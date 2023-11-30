use std::{
    ffi::OsStr,
    io::{Read, Seek},
};

use bevy::{
    asset::LoadContext,
    prelude::*,
    render::texture::{CompressedImageFormats, ImageSampler, ImageType},
    utils::HashMap,
};
use mc_rs_core::ResourceLocation;
use zip::ZipArchive;

use crate::pack::{ResourcePackLoader, ResourcePackLoaderError};

/// Reads all textures from the zip archive.
pub(crate) fn read_textures(
    zip: &mut ZipArchive<impl Read + Seek>,
    load_context: &mut LoadContext,
) -> Result<HashMap<ResourceLocation, Handle<Image>>, ResourcePackLoaderError> {
    #[cfg(any(debug_assertions, feature = "debug"))]
    trace!("Loading textures: {}", load_context.asset_path());

    let mut textures = HashMap::new();

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

        // Skip all files that are not images.
        if !matches!(ext.as_str(), "png" | "jpg" | "jpeg") {
            continue;
        }

        // Get the resource location from the file path.
        let Some(key) = ResourcePackLoader::path_to_resource_location(file_path, "textures") else {
            continue;
        };

        // Load the image from the file.
        let image = Image::from_buffer(
            &file.bytes().collect::<Result<Vec<_>, _>>()?,
            ImageType::Extension(&ext),
            CompressedImageFormats::all(),
            true,
            ImageSampler::default(),
        )?;

        // Add the image to the asset server.
        let handle = load_context.labeled_asset_scope(key.to_string(), |_| image);

        // Add the texture to the hashmap.
        textures.insert(key, handle);
    }

    #[cfg(any(debug_assertions, feature = "debug"))]
    {
        trace!("{}:\n{textures:#?}", load_context.asset_path());

        debug!(
            "Loaded {} textures from {}",
            textures.len(),
            load_context.asset_path()
        );
    }

    Ok(textures)
}
