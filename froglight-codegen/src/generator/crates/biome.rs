use std::fmt::Write;

use cafebabe::{
    bytecode::Opcode,
    constant_pool::{LiteralConstant, Loadable, MemberRef},
};
use convert_case::{Case, Casing};
use facet::Facet;
use facet_value::Value;
use indexmap::{IndexMap, IndexSet};
use miette::Result;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    common::{DATA, Version, VersionStorage, WORKSPACE_DIR},
    config::{ConfigBundle, VersionPair},
    helper::{ModuleBuilder, VersionHelper},
    source::{JarData, JarFile},
};

#[derive(Debug, Clone)]
pub struct BiomeData {
    pub biomes: IndexMap<String, BiomeSettings>,
}

#[derive(Debug, Default, Clone)]
pub struct BiomeSettings {
    pub ident: String,
    pub temperature: f64,
    pub downfall: f64,
    pub precipitation: bool,
    pub attr: Vec<String>,
    pub feat: Vec<Vec<String>>,
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
                                BiomeSettings {
                                    ident: format!("minecraft:{constant}"),
                                    ..Default::default()
                                },
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

        JarFile::get_for(version, storage, async |file| {
            let directory = file.generated.join("data/minecraft/worldgen/biome");
            for biome in biomes.values_mut() {
                let mut path = directory.join(biome.ident.trim_start_matches("minecraft:"));
                path.set_extension("json");

                let Ok(content) = tokio::fs::read_to_string(&path).await else {
                    tracing::warn!("Failed to read biome file: {}", path.display());
                    continue;
                };

                match facet_json::from_str::<BiomeJson>(&content) {
                    Ok(json) => {
                        biome.temperature = json.temperature;
                        biome.downfall = json.downfall;
                        biome.precipitation = json.has_precipitation;
                        biome.feat = json.features;
                    }
                    Err(err) => {
                        miette::bail!("Failed to parse \"{}\" JSON, {err}", biome.ident);
                    }
                }
            }

            Ok(())
        })
        .await?;

        tracing::debug!("Found {} biomes for \"{}\"", biomes.len(), version.as_str());

        Ok(BiomeData { biomes })
    }
}

#[derive(Debug, Clone, Facet)]
struct BiomeJson {
    #[facet(default)]
    attributes: Value,
    downfall: f64,
    effects: Value,
    features: Vec<Vec<String>>,
    has_precipitation: bool,
    temperature: f64,
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
            content.push_str("\ngenerate! {\n    @biomes\n");
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
pub async fn generate(version: &VersionPair, storage: &mut VersionStorage) -> Result<()> {
    BiomeData::get_for(&version.real, storage, async |data| {
        let path = WORKSPACE_DIR.join("froglight-biome/src/generated");
        let mut module = ModuleBuilder::new_after_marker(path).await?;

        module
            .with_submodule(&version.base.as_feature(), async |module, settings| {
                let mut content = String::new();

                let version_type = version.base.as_feature().to_ascii_uppercase();
                content.push_str("\nuse froglight_common::version::");
                content.push_str(&version_type);
                content.push_str(";\n\n#[allow(clippy::wildcard_imports, reason = \"Generated code\")]\nuse crate::{\n    biome::{BiomeAttributeSet, BiomeFeatureSet},\n    generated::biome::*,\n};\n\n");

                content.push_str("generate! {\n    @version ");
                content.push_str(&version_type);
                content.push_str(",\n");

                for (index, (biome, settings)) in data.biomes.iter().enumerate() {
                    content.push_str("    ");
                    content.push_str(&biome.to_case(Case::Pascal));
                    content.push_str(" => { ident: \"");
                    content.push_str(&settings.ident);
                    content.push_str("\", global: ");
                    content.push_str(&index.to_string());
                    writeln!(content, ", prop: {{ foliage: {}, grass: {}, water: {}, precip: {}, temp: {}f32, downfall: {}f32 }},", 0, 0, 0, settings.precipitation, settings.temperature, settings.downfall).unwrap();
                    content.push_str("        attr: BiomeAttributeSet::empty(), feat: BiomeFeatureSet::empty() }");
                    if index != data.biomes.len() - 1 {
                        content.push(',');
                    }
                    content.push('\n');
                }

                content.push('}');
                module.with_docs("Placeholder").with_content(&content);
                Ok(settings.with_feature(version.base.as_feature()))
            })
            .await?;

        module.build().await
    })
    .await
}
