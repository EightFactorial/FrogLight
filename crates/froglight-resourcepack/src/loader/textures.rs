use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy::{asset::LoadContext, prelude::*, render::texture::Image};
use froglight_core::data::ResourceKey;
use futures_lite::AsyncReadExt;
use image::io::Reader as ImageReader;

use crate::{loader::ResourcePackLoaderError, settings::ResourcePackLoaderSettings};

pub(super) async fn load_texture(
    resource_key: &ResourceKey,
    entry: &mut ZipEntryReader<'_, futures_lite::io::Cursor<&[u8]>, WithEntry<'_>>,
    load_context: &mut LoadContext<'_>,
    settings: &ResourcePackLoaderSettings,
) -> Result<Option<Handle<Image>>, ResourcePackLoaderError> {
    let manager = settings.0.as_ref().unwrap();

    {
        let textures = manager.texture_assets.read();
        if textures.contains_key(resource_key) {
            #[cfg(feature = "logging")]
            trace!(
                "Skipping `{resource_key}` from `{}` as it already exists",
                load_context.path().display()
            );
            return Ok(None);
        }
    }

    // Read the entire file into memory.
    let mut data = Vec::new();
    entry.read_to_end(&mut data).await?;

    // Decode the image.
    let dyn_img = ImageReader::new(std::io::Cursor::new(data)).with_guessed_format()?.decode()?;
    let image = Image::from_dynamic(dyn_img, false);

    // Load the texture into the asset manager.
    // Store the strong handle in the ResourcePackManager, and return a weak handle.
    let handle = load_context.labeled_asset_scope(resource_key.to_string(), |_| image);
    let weak = handle.clone_weak();

    // Insert the texture into the texture assets if it doesn't exist.
    let mut textures = manager.texture_assets.write();
    textures.insert(resource_key.clone(), handle);

    Ok(Some(weak))
}
