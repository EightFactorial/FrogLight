use std::io::{Read, Seek};

use bevy::{asset::LoadContext, prelude::*};
use zip::ZipArchive;

use crate::resourcepacks::meta::ResourcePackMetaContainer;

use super::ResourcePackLoaderError;

/// Reads the pack.mcmeta file from the zip archive.
pub(super) fn read_mcmeta(
    zip: &mut ZipArchive<impl Read + Seek>,
    load_context: &mut LoadContext,
) -> Result<ResourcePackMetaContainer, ResourcePackLoaderError> {
    // Get the pack.mcmeta file from the zip archive.
    let Ok(file) = zip.by_name("pack.mcmeta") else {
        #[cfg(any(debug_assertions, feature = "debug"))]
        warn!("No pack.mcmeta found in {}", load_context.asset_path());
        return Ok(None.into());
    };

    // Deserialize the mcmeta file.
    match serde_json::from_reader(file) {
        Ok(mcmeta) => Ok(mcmeta),
        Err(err) => {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!(
                "Failed to read {}/pack.mcmeta, {err}",
                load_context.asset_path()
            );
            Err(err.into())
        }
    }
}
