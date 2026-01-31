use indexmap::IndexMap;
use miette::Result;

use crate::{
    common::{Version, VersionStorage},
    config::VersionPair,
    helper::BytecodeEmulator,
    source::JarData,
};

pub struct BlockData {
    pub blocks: IndexMap<String, String>,
    pub block_families: IndexMap<String, Vec<String>>,
}

impl BlockData {
    /// Get the [`BlockData`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `BlockData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`BlockData`] for the given [`Version`].
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let blocks = IndexMap::new();
        let block_families = IndexMap::new();

        JarData::get_for(version, storage, async |data| {
            let _emu =
                BytecodeEmulator::run(data, "net/minecraft/world/level/block/Blocks", "<clinit>")?;

            Ok(())
        })
        .await?;

        tracing::debug!("Found {} blocks for \"{}\"", blocks.len(), version.as_str());
        tracing::debug!(
            "Found {} block families for \"{}\"",
            block_families.len(),
            version.as_str()
        );

        Ok(BlockData { blocks, block_families })
    }
}

// -------------------------------------------------------------------------------------------------

pub async fn generate(version: &VersionPair, storage: &mut VersionStorage) -> Result<()> {
    BlockData::get_for(&version.real, storage, async |_data| Ok(())).await
}
