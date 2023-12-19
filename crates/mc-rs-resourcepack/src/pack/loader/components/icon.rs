use std::io::{Cursor, Read, Seek};

use bevy::{asset::LoadContext, prelude::*};
use image::io::Reader as ImageReader;
use zip::ZipArchive;

use crate::pack::ResourcePackLoaderError;

/// Reads the pack.png file from the zip archive.
pub(crate) fn read_icon(
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
    let dyn_image = ImageReader::new(Cursor::new(file.bytes().collect::<Result<Vec<_>, _>>()?))
        .with_guessed_format()?
        .decode()?;
    let image = Image::from_dynamic(dyn_image, false);

    // Add the image to the asset server.
    let icon = load_context.labeled_asset_scope(String::from("pack_icon"), |_| image);

    Ok(Some(icon))
}
