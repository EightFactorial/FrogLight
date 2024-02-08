use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy::prelude::*;
use futures_lite::{io::Cursor, AsyncReadExt};

use crate::loader::ResourcePackLoaderError;

/// An asset's `.mcmeta` file.
///
/// This file is used to define the properties of an asset.
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub struct AssetMcMeta {}

impl AssetMcMeta {
    /// Parses the `.mcmeta` file for a given asset.
    #[allow(clippy::missing_errors_doc)]
    pub(crate) async fn _parse(
        entry: &mut ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
    ) -> Result<Self, ResourcePackLoaderError> {
        let mut contents = String::new();
        entry.read_to_string(&mut contents).await?;

        Ok(AssetMcMeta {})
    }
}
