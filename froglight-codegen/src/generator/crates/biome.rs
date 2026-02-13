use std::fmt::Write;

use cafebabe::{
    bytecode::Opcode,
    constant_pool::{LiteralConstant, Loadable, MemberRef},
};
use convert_case::{Case, Casing};
use facet::Facet;
use facet_value::{Value, ValueType};
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
    pub attr: IndexMap<String, Value>,
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
            #[derive(Debug, Clone, Facet)]
            struct BiomeJson {
                #[facet(default)]
                attributes: IndexMap<String, Value>,
                downfall: f64,
                effects: Value,
                features: Vec<Vec<String>>,
                has_precipitation: bool,
                temperature: f64,
            }

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
                        biome.attr = json.attributes;
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

// -------------------------------------------------------------------------------------------------

/// Generate global biome data.
#[expect(clippy::too_many_lines, reason = ":nod:")]
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

            // Deduplicate and sort the biome types and attributes across all versions.
            let mut biomes = IndexSet::<String>::new();
            let mut attributes = IndexMap::<String, (String, Value)>::new();
            for versioned in global_biomes {
                for (name, settings) in versioned.biomes {
                    biomes.insert(name.to_case(Case::Pascal));

                    for (attr, contents) in settings.attr {
                        let attr_name = attr
                            .trim_start_matches("minecraft:")
                            .replace('/', " ")
                            .to_case(Case::Pascal);

                        if attributes.contains_key(&attr_name) {
                            let (_, existing) = attributes.get_mut(&attr_name).unwrap();

                            if contents.value_type() != existing.value_type() {
                                miette::bail!(
                                    "Mismatched attribute types for \"{attr_name}\": {} vs {}",
                                    facet_value::format_value(&contents),
                                    facet_value::format_value(existing)
                                );
                            }

                            match (contents.value_type(), existing.value_type()) {
                                (ValueType::Object, ValueType::Object) => {
                                    let contents = contents.as_object().unwrap();
                                    let existing = existing.as_object_mut().unwrap();
                                    for (field, val) in contents {
                                        existing.insert(field.clone(), val.clone());
                                    }
                                }
                                (a, b) if a != b => {
                                    miette::bail!(
                                        "Mismatched attribute types for \"{attr_name}\": {} vs {}",
                                        facet_value::format_value(&contents),
                                        facet_value::format_value(existing)
                                    );
                                }
                                _ => {}
                            }
                        } else {
                            attributes.insert(attr_name, (attr, contents));
                        }
                    }
                }
            }
            biomes.sort_unstable();
            attributes.sort_unstable_keys();

            // Build the `biome` module
            {
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

                module.build().await?;
            }

            // Build the `attribute` module
            {
                let path = WORKSPACE_DIR.join("froglight-biome/src/generated");
                let mut module = ModuleBuilder::new("attribute", path);

                // Generate the content
                let mut content = String::new();
                content.push_str("\nuse alloc::{string::String, vec::Vec};\n\n");
                content.push_str("use facet::Facet;\n\n");
                content.push_str("generate! {\n    @attributes\n");

                for (attribute, (ident, value)) in attributes {
                    content.push_str("    ");

                    let tag = match value.value_type() {
                        ValueType::Object => "object",
                        ValueType::Array
                        | ValueType::Null
                        | ValueType::Bool
                        | ValueType::Number
                        | ValueType::String
                        | ValueType::Bytes => "newtype",
                        _ => miette::bail!("Unsupported attribute type: {:?}", value.value_type()),
                    };

                    content.push('@');
                    content.push_str(tag);
                    content.push_str(" \"");
                    content.push_str(&ident);
                    content.push_str("\" ");
                    content.push_str(&attribute);
                    content.push(' ');

                    let extra_types = generate_value(&attribute, &value, &mut content)?;
                    if !extra_types.is_empty() {
                        content.push_str(" => { ");
                        for (index, extra) in extra_types.iter().enumerate() {
                            content.push_str(extra);
                            if index != extra_types.len() - 1 {
                                content.push_str(", ");
                            }
                        }
                        content.push_str(" }");
                    }

                    content.push_str(",\n");
                }
                content.push('}');

                // Finalize and build the module
                module
                    .with_docs(
                        "Biome attributes for all [`Version`](froglight_common::version::Version)s.

@generated",
                    )
                    .with_content(&content);

                module.build().await?;
            }

            Ok(())
        })
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(err) => miette::bail!("Failed to generate global biome data: {err}"),
    }
}

fn generate_value(name: &str, value: &Value, content: &mut String) -> Result<Vec<String>> {
    let mut extra = Vec::<String>::new();

    match value.value_type() {
        ValueType::Bool => content.push_str("bool"),
        ValueType::Number => content.push_str("f64"),
        ValueType::String => content.push_str("String"),
        ValueType::Bytes => content.push_str("Vec<u8>"),
        ValueType::Array => {
            let array = value.as_array().unwrap();
            let inner_type = array.as_slice().first().unwrap();
            content.push_str("Vec<");
            let type_name = format!("{name}Item");
            content.push_str(&type_name);
            content.push('>');

            let mut type_content = String::new();
            let extra_types = generate_value(&type_name, inner_type, &mut type_content)?;
            extra.push(format!("{type_name} {type_content}"));
            extra.extend(extra_types);
        }
        ValueType::Object => {
            content.push_str("{ ");
            let object = value.as_object();
            for (index, (field, val)) in object.unwrap().iter().enumerate() {
                // Handle reserved keywords
                let mut field = field.as_str();
                match field {
                    "type" => field = "r#type",
                    "loop" => field = "r#loop",
                    _ => {}
                }

                content.push_str(field);
                content.push_str(": ");
                match val.value_type() {
                    ValueType::Null => {}
                    ValueType::Bool => content.push_str("bool"),
                    ValueType::Number => content.push_str("f64"),
                    ValueType::String => content.push_str("String"),
                    ValueType::Bytes => content.push_str("Vec<u8>"),
                    ValueType::Array => content.push_str("Vec<Object>"),
                    ValueType::Object => {
                        let type_name = format!("{name}{}", field.to_case(Case::Pascal));
                        content.push_str(&type_name);

                        let mut type_content = String::new();
                        let extra_types = generate_value(&type_name, val, &mut type_content)?;
                        extra.push(format!("{type_name} {type_content}"));
                        extra.extend(extra_types);
                    }
                    ValueType::DateTime | ValueType::Uuid | ValueType::QName => {
                        miette::bail!("Unsupported attribute value type: {:?}", val.value_type());
                    }
                }
                if index != object.unwrap().len() - 1 {
                    content.push_str(", ");
                }
            }
            content.push_str(" }");
        }
        _ => unreachable!(),
    }

    Ok(extra)
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
