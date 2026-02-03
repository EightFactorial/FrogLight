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
pub struct BiomeData {
    pub biomes: IndexMap<String, String>,
}

impl BiomeData {
    /// Get the [`BiomeData`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `BiomeData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`BiomeData`] for the given [`Version`].
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut biomes = IndexMap::new();

        JarData::get_for(version, storage, async |data| {
            let code = data
                .get_class_method_code("net/minecraft/world/level/biome/Biomes", "<clinit>")
                .unwrap();

            let mut constant = None;
            for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
                match op {
                    Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(s))) => {
                        constant = Some(s.as_ref());
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type }) => {
                        if class_name != "net/minecraft/world/level/biome/Biomes" {
                            tracing::warn!(
                                "Unexpected class name in Biomes <clinit>: {class_name}"
                            );
                        }
                        if name_and_type.descriptor != "Lnet/minecraft/resources/ResourceKey;" {
                            tracing::warn!(
                                "Unexpected descriptor in Biomes <clinit>: {}",
                                name_and_type.descriptor
                            );
                        }

                        if let Some(constant) = constant.take() {
                            biomes.insert(
                                name_and_type.name.to_string(),
                                format!("minecraft:{constant}"),
                            );
                        } else {
                            tracing::warn!(
                                "Putstatic without preceding LDC in Biomes <clinit>: {}",
                                name_and_type.name
                            );
                        }
                    }
                    _ => {}
                }
            }

            Ok(())
        })
        .await?;

        tracing::debug!("Found {} biomes for \"{}\"", biomes.len(), version.as_str());

        Ok(BiomeData { biomes })
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate global biome data.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<()>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async move {
            // Collect the `BiomeData` for all versions.
            let global_biomes = VersionHelper::for_all(config, async |version| {
                let pinned = DATA.pin_owned();
                let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);
                let mut storage = storage.write().await;
                BiomeData::get_for(&version.real, &mut storage, async |data| Ok(data.clone())).await
            })
            .await?;

            // Deduplicate and sort the biome types
            let mut biomes = IndexSet::new();
            for versioned in global_biomes {
                for (name, _id) in versioned.biomes {
                    biomes.insert(name.to_case(Case::Pascal));
                }
            }
            biomes.sort_unstable();

            // Start building the module
            let path = WORKSPACE_DIR.join("froglight-biome/src/generated");
            let mut module = ModuleBuilder::new("biome", path);

            // Generate the content
            let mut content = String::new();
            content.push_str("generate! {\n    @biomes\n");
            for identifier in biomes {
                writeln!(content, "    {identifier},").unwrap();
            }
            content.push('}');

            // Finalize and build the module
            module
                .with_docs(
                    "Biome types for all [`Version`](froglight_common::version::Version)s.

@generated",
                )
                .with_content(&content);

            module.build().await
        })
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(err) => miette::bail!("Failed to generate global biome data: {err}"),
    }
}

/// Generate `Version`-specific biome data.
pub async fn generate(
    version: &VersionPair,
    storage: &mut VersionStorage,
    _config: &ConfigBundle,
) -> Result<()> {
    BiomeData::get_for(&version.real, storage, async |_data| Ok(())).await
}
