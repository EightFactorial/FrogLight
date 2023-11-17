use std::io::{Read, Seek};

use bevy::{
    asset::LoadContext,
    prelude::*,
    render::texture::{CompressedImageFormats, ImageSampler, ImageType},
};
use zip::ZipArchive;

use crate::loader::ResourcePackLoaderError;

/// Reads the pack.png file from the zip archive.
pub fn read_icon(
    zip: &mut ZipArchive<impl Read + Seek>,
    load_context: &mut LoadContext,
) -> Result<Option<Handle<Image>>, ResourcePackLoaderError> {
    #[cfg(any(debug_assertions, feature = "debug"))]
    trace!("Loading pack icon: {}", load_context.asset_path());

    // Get the pack.png file from the zip archive.
    let Ok(file) = zip.by_name("pack.png") else {
        #[cfg(any(debug_assertions, feature = "debug"))]
        warn!("No pack.png found in {}", load_context.asset_path());
        return Ok(None);
    };

    // Load the image from the file.
    let image = Image::from_buffer(
        &file.bytes().collect::<Result<Vec<_>, _>>()?,
        ImageType::Extension("png"),
        CompressedImageFormats::all(),
        false,
        ImageSampler::Default,
    )?;

    // Add the image to the asset server.
    let icon = load_context.labeled_asset_scope(String::from("pack_icon"), |_| image);

    Ok(Some(icon))
}
