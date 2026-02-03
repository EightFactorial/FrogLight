use std::fmt::Write;

use convert_case::{Case, Casing};
use indexmap::{IndexMap, IndexSet};
use miette::Result;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    common::{DATA, Version, VersionStorage, WORKSPACE_DIR},
    config::{ConfigBundle, VersionPair},
    helper::{BytecodeEmulator, ModuleBuilder, VersionHelper},
    source::JarData,
};

#[derive(Debug, Clone)]
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

/// Generate global block data.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<()>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async {
            // Collect the `BlockData` for all versions.
            let global_blocks = VersionHelper::for_all(config, async |version| {
                let pinned = DATA.pin_owned();
                let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);
                let mut storage = storage.write().await;
                BlockData::get_for(&version.real, &mut storage, async |data| Ok(data.clone())).await
            })
            .await?;

            // Deduplicate and sort the block types
            let mut blocks = IndexSet::new();
            for versioned in global_blocks {
                for (name, _id) in versioned.blocks {
                    blocks.insert(name.to_case(Case::Pascal));
                }
            }
            blocks.sort_unstable();

            // Start building the module
            let path = WORKSPACE_DIR.join("froglight-block/src/generated");
            let mut module = ModuleBuilder::new("block", path);

            // Generate the content
            let mut content = String::new();
            content.push_str("generate! {\n    @blocks\n");
            for identifier in blocks {
                writeln!(content, "    {identifier},").unwrap();
            }
            content.push('}');

            // Finalize and build the module
            module
                .with_docs(
                    "Block types for all [`Version`](froglight_common::version::Version)s.

@generated",
                )
                .with_content(&content);

            module.build().await
        })
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(err) => miette::bail!("Failed to generate global block data: {err}"),
    }
}

/// Generate `Version`-specific block data.
pub async fn generate(
    version: &VersionPair,
    storage: &mut VersionStorage,
    _config: &ConfigBundle,
) -> Result<()> {
    BlockData::get_for(&version.real, storage, async |_data| Ok(())).await
}
