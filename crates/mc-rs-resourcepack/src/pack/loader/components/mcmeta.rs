use std::io::{Read, Seek};

#[allow(unused_imports)]
use bevy::{asset::LoadContext, prelude::*};
use zip::ZipArchive;

use crate::pack::{ResourcePackLoaderError, ResourcePackMetaContainer};

/// Reads the pack.mcmeta file from the zip archive.
#[allow(unused_variables)]
pub fn read_mcmeta(
    zip: &mut ZipArchive<impl Read + Seek>,
    load_context: &mut LoadContext,
) -> Result<ResourcePackMetaContainer, ResourcePackLoaderError> {
    #[cfg(any(debug_assertions, feature = "debug"))]
    trace!("Loading pack mcmeta: {}", load_context.asset_path());

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
