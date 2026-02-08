use std::fmt::Write;

use cafebabe::{
    bytecode::Opcode,
    constant_pool::{LiteralConstant, Loadable, MemberRef},
};
use convert_case::{Case, Casing};
use indexmap::{IndexMap, IndexSet};
use miette::Result;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    common::{DATA, Version, VersionStorage, WORKSPACE_DIR},
    config::{ConfigBundle, VersionPair},
    helper::{ModuleBuilder, VersionHelper},
    source::JarData,
};

#[derive(Debug, Clone)]
pub struct BlockData {
    pub blocks: IndexMap<String, BlockSettings>,
    pub block_families: IndexMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct BlockSettings {
    pub ident: String,
    pub states: usize,
    /// Note: Zero-indexed
    pub default_state: usize,
}

impl Default for BlockSettings {
    fn default() -> Self { Self { ident: String::new(), states: 1, default_state: 0 } }
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
        let mut blocks = IndexMap::new();
        let block_families = IndexMap::new();

        JarData::get_for(version, storage, async |data| {
            let code = data
                .get_class_method_code("net/minecraft/world/level/block/Blocks", "<clinit>")
                .unwrap();

            let mut current = BlockSettings::default();
            for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
                match op {
                    Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(s)))
                        if current.ident.is_empty() =>
                    {
                        current.ident = format!("minecraft:{s}");
                    }

                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/level/block/Blocks"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/level/block/Block;" =>
                    {
                        blocks
                            .insert(name_and_type.name.to_string(), core::mem::take(&mut current));
                    }
                    _ => {}
                }
            }

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
            content.push_str("\ngenerate! {\n    @blocks\n");
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
pub async fn generate(version: &VersionPair, storage: &mut VersionStorage) -> Result<()> {
    BlockData::get_for(&version.real, storage, async |data| {
        let path = WORKSPACE_DIR.join("froglight-block/src/generated");
        let mut module = ModuleBuilder::new_after_marker(path).await?;

        module
            .with_submodule(&version.base.as_feature(), async |module, settings| {
                let mut content = String::new();

                let version_type = version.base.as_feature().to_ascii_uppercase();
                content.push_str("\nuse froglight_common::version::");
                content.push_str(&version_type);
                content.push_str(";\n\n#[allow(clippy::wildcard_imports, reason = \"Generated code\")]\nuse crate::generated::block::*;\n\n");

                // Generate the block data macro invocation
                content.push_str("generate! {\n    @version ");
                content.push_str(&version_type);
                content.push_str(",\n");

                let mut global_index = 0usize;
                for (index, (block, settings)) in data.blocks.iter().enumerate() {
                    content.push_str("    ");
                    content.push_str(&block.to_case(Case::Pascal));
                    content.push_str(" => { ident: \"");
                    content.push_str(&settings.ident);
                    content.push_str("\", global: ");
                    content.push_str(&global_index.to_string());
                    content.push_str(" }");
                    if index != data.blocks.len() - 1 {
                        content.push(',');
                    }
                    content.push('\n');
                    global_index += settings.states;
                }

                content.push('}');

                // Generate the block storage macro invocation
                content.push_str("\n\ngenerate! {\n    @version @storage ");
                content.push_str(&version_type);
                content.push_str(",\n    ");

                let mut total_index = 1usize;
                for (index, (ident, block)) in data.blocks.iter().enumerate() {
                    for _ in 0..block.states {
                        content.push_str(&ident.to_case(Case::Pascal));
                        if index == data.blocks.len() - 1 {
                            content.push('\n');
                        } else {
                            content.push_str(", ");
                            if total_index.is_multiple_of(8)  {
                                content.push_str("\n    ");
                            }
                        }
                        total_index += 1;
                    }
                }

                content.push('}');


                module.with_docs("Placeholder").with_content(&content);
                Ok(settings.with_feature(version.base.as_feature()))
            })
            .await?;

        module.build().await
    }).await
}
