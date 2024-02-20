use async_zip::base::read::{WithEntry, ZipEntryReader};
use bevy::prelude::*;
use futures_lite::{io::Cursor, AsyncReadExt};

use crate::ResourcePackLoaderError;

/// A resource pack's `pack.mcmeta` file.
///
/// This file is used to define the properties of a resource pack.
#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect)]
pub struct PackMcMeta {}

impl PackMcMeta {
    /// Parses the `pack.mcmeta` file for a given resource pack.
    ///
    /// # Errors
    /// - If the file cannot be read
    /// - If the file cannot be parsed
    pub(crate) async fn _parse(
        entry: &mut ZipEntryReader<'_, Cursor<&[u8]>, WithEntry<'_>>,
    ) -> Result<Self, ResourcePackLoaderError> {
        let mut contents = String::new();
        entry.read_to_string(&mut contents).await?;

        Ok(PackMcMeta {})
    }
}
