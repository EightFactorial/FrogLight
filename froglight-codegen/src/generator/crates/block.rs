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
pub struct BlockData {
    pub blocks: IndexMap<String, BlockSettings>,
    pub block_families: IndexMap<String, Vec<String>>,

    pub report: BlockReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockSettings {
    pub ident: String,
    pub attributes: Vec<BlockAttribute>,
    /// Note: Total states (min 1)
    pub states: usize,
    /// Note: Zero-indexed
    pub default: usize,
}

impl Default for BlockSettings {
    fn default() -> Self {
        Self { ident: String::new(), attributes: Vec::new(), states: 1, default: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockAttribute {
    pub ty: String,
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
#[facet(transparent)]
pub struct BlockReport(pub IndexMap<String, BlockReportEntry>);

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct BlockReportEntry {
    #[facet(default)]
    pub definition: Value,
    pub properties: IndexMap<String, Vec<String>>,
    pub states: Vec<BlockReportState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct BlockReportState {
    #[facet(default)]
    pub default: bool,
    pub id: usize,
    pub properties: IndexMap<String, String>,
}

impl BlockReport {
    /// Update the block attribute names and values based on the report data.
    ///
    /// Then update the blocks map to use the attributes and states from the
    /// report.
    fn update_blocks(&mut self, blocks: &mut IndexMap<String, BlockSettings>) -> Result<()> {
        // Update the report attribute names and values
        // {Property: ([Values], [(BlockIndex, BlockPropertyIndex)])}
        let mut property_map = IndexMap::<String, (Vec<String>, Vec<(usize, usize)>)>::new();
        for (b_index, entry) in self.0.values().enumerate() {
            for (bp_index, (property, values)) in entry.properties.iter().enumerate() {
                let mut property = property.clone();

                if values.len() == 2
                    && values.contains(&String::from("true"))
                    && values.contains(&String::from("false"))
                {
                    // Append "Bool" to the property name if it has boolean values
                    property = format!("{}Bool", property.to_case(Case::Pascal));
                } else if self
                    .0
                    .values()
                    .flat_map(|entry| entry.properties.keys())
                    .filter(|p| *p == &property)
                    .count()
                    > 1
                {
                    // Convert the property name to `PascalCase`
                    property = format!("{}_", property.to_case(Case::Pascal));
                    // Append the value names to the property name
                    for value in values {
                        property.push_str(&value.to_case(Case::Pascal));
                    }
                } else {
                    // Convert the property name to `PascalCase`
                    property = property.to_case(Case::Pascal);
                }

                let entry =
                    property_map.entry(property).or_insert_with(|| (values.clone(), Vec::new()));

                entry.1.push((b_index, bp_index));
            }
        }

        // Update the report properties and states
        for (property, (values, indexes)) in &property_map {
            for &(b_index, bp_index) in indexes {
                let entry = self.0.values_mut().nth(b_index).unwrap();
                entry.properties.shift_remove_index(bp_index);
                entry.properties.shift_insert(bp_index, property.clone(), values.clone());
            }
        }

        // Update the blocks map
        for (ty, settings) in blocks {
            let Some(report) = self.0.get(&settings.ident) else {
                miette::bail!("Block \"{}\" ({ty}) not found in report!", settings.ident);
            };

            settings.states = report.states.len();
            settings.default =
                report.states.iter().position(|state| state.default).unwrap_or_default();

            for ((ty, values), (name, _)) in report
                .properties
                .clone()
                .into_iter()
                .zip(report.states.first().unwrap().properties.iter())
            {
                settings.attributes.push(BlockAttribute { ty, name: name.clone(), values });
            }
        }

        Ok(())
    }
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
    #[allow(clippy::too_many_lines, reason = "Necessary for parsing both bytecode and report")]
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut blocks = IndexMap::new();
        let block_families = IndexMap::new();

        JarData::get_for(version, storage, async |data| {
            let code = data
                .get_class_method_code("net/minecraft/world/level/block/Blocks", "<clinit>", None)
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

                    Opcode::Getstatic(MemberRef { class_name, name_and_type })
                        if matches!(
                            class_name.as_ref(),
                            "net/minecraft/references/Blocks" | "net/minecraft/references/BlockIds"
                        ) && current.ident.is_empty() =>
                    {
                        let code = data
                            .get_class_method_code(class_name.as_ref(), "<clinit>", None)
                            .unwrap();

                        // Find the string constant used to initialize the block reference
                        let mut constant = None;
                        for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
                            match op {
                                Opcode::Ldc(Loadable::LiteralConstant(
                                    LiteralConstant::String(s),
                                ))
                                | Opcode::LdcW(Loadable::LiteralConstant(
                                    LiteralConstant::String(s),
                                ))
                                | Opcode::Ldc2W(Loadable::LiteralConstant(
                                    LiteralConstant::String(s),
                                )) => {
                                    constant = Some(s);
                                }
                                Opcode::Putstatic(MemberRef {
                                    class_name: constant_class,
                                    name_and_type: constant_field,
                                }) if constant_class == class_name
                                    && constant_field.name == name_and_type.name =>
                                {
                                    current.ident = format!(
                                        "minecraft:{}",
                                        constant.expect("Expected a string constant")
                                    );
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }

                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/level/block/Blocks"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/level/block/Block;" =>
                    {
                        if current.ident.is_empty() {
                            miette::bail!(
                                "Failed to find block identifier for \"{}\"",
                                name_and_type.name
                            );
                        }

                        blocks
                            .insert(name_and_type.name.to_string(), core::mem::take(&mut current));
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/level/block/Blocks"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/level/block/WeatheringCopperBlocks;" =>
                    {
                        if current.ident.is_empty() {
                            miette::bail!(
                                "Failed to find block identifier for \"{}\"",
                                name_and_type.name
                            );
                        }

                        let original = core::mem::take(&mut current);
                        blocks.insert(name_and_type.name.to_string(), original.clone());
                        blocks.insert(
                            format!("EXPOSED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":exposed_"),
                                ..original.clone()
                            },
                        );
                        blocks.insert(
                            format!("WEATHERED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":weathered_"),
                                ..original.clone()
                            },
                        );
                        blocks.insert(
                            format!("OXIDIZED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":oxidized_"),
                                ..original.clone()
                            },
                        );
                        blocks.insert(
                            format!("WAXED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":waxed_"),
                                ..original.clone()
                            },
                        );
                        blocks.insert(
                            format!("WAXED_EXPOSED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":waxed_exposed_"),
                                ..original.clone()
                            },
                        );
                        blocks.insert(
                            format!("WAXED_WEATHERED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":waxed_weathered_"),
                                ..original.clone()
                            },
                        );
                        blocks.insert(
                            format!("WAXED_OXIDIZED_{}", name_and_type.name),
                            BlockSettings {
                                ident: original.ident.replace(':', ":waxed_oxidized_"),
                                ..original.clone()
                            },
                        );
                    }

                    _ => {}
                }
            }

            Ok(())
        })
        .await?;

        let report = JarFile::get_for(version, storage, async |jar| {
            let path = jar.generated.join("reports/blocks.json");
            let Ok(content) = tokio::fs::read_to_string(path).await else {
                miette::bail!("Failed to read block report for \"{}\"", version.as_str());
            };

            match facet_json::from_str::<BlockReport>(&content) {
                Ok(mut report) => {
                    report.update_blocks(&mut blocks)?;
                    Ok(report)
                }
                Err(err) => miette::bail!(
                    "Failed to parse block report for \"{}\": {err}",
                    version.as_str()
                ),
            }
        })
        .await?;

        tracing::debug!("Found {} blocks for \"{}\"", blocks.len(), version.as_str());
        tracing::debug!(
            "Found {} block families for \"{}\"",
            block_families.len(),
            version.as_str()
        );

        Ok(BlockData { blocks, block_families, report })
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

            // Build the `attribute` module with the collected block attributes
            {
                let path = WORKSPACE_DIR.join("froglight-block/src/generated");
                let mut module = ModuleBuilder::new("attribute", path);

                // Deduplicate and sort the block types
                let mut attributes = IndexSet::new();
                for versioned in &global_blocks {
                    for settings in versioned.blocks.values() {
                        attributes.extend(settings.attributes.clone());
                    }
                }
                attributes.sort_unstable_by_key(|v| v.ty.clone());

                // Generate the content
                let mut content = String::new();
                content.push_str("#![allow(non_camel_case_types, reason = \"Generated code\")]");
                content.push_str("\n\ngenerate! {\n    @attributes\n");

                let attr_length = attributes.len();
                for (index, attribute) in attributes.into_iter().enumerate() {
                    write!(content, "    {} => [ ", attribute.ty).unwrap();

                    let val_length = attribute.values.len();
                    for (index, value) in attribute.values.into_iter().enumerate() {
                        // Change numeric values to `_{value}` and convert to `PascalCase`
                        let ident = if value.parse::<i32>().is_ok() {
                            format!("_{value}")
                        } else {
                            value.to_case(Case::Pascal)
                        };

                        write!(content, "\"{value}\" => {ident}").unwrap();
                        if index != val_length - 1 {
                            content.push_str(", ");
                        }
                    }

                    content.push_str(" ]");
                    if index != attr_length - 1 {
                        content.push(',');
                    }
                    content.push('\n');
                }
                content.push('}');

                // Finalize and build the module
                module
                    .with_docs(
                        "Block attributes for all [`Version`](froglight_common::version::Version)s.

@generated",
                    )
                    .with_content(&content);

                module.build().await?;
            }

            // Build the `block` module with the collected block types
            {
                let path = WORKSPACE_DIR.join("froglight-block/src/generated");
                let mut module = ModuleBuilder::new("block", path);

                // Deduplicate and sort the block types
                let mut blocks = IndexSet::new();
                for versioned in &global_blocks {
                    for (name, _id) in &versioned.blocks {
                        blocks.insert(name.to_case(Case::Pascal));
                    }
                }
                blocks.sort_unstable();

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

                module.build().await?;
            };

            Ok(())
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
                content.push_str(";\n\n#[allow(clippy::wildcard_imports, reason = \"Generated code\")]\nuse crate::generated::{attribute::*, block::*};\n\n");

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
                    content.push_str(", default: ");
                    content.push_str(&settings.default.to_string());

                    content.push_str(",\n        ty: [ ");
                    for (attr_index, attr) in settings.attributes.iter().enumerate() {
                        content.push('"');
                        content.push_str(&attr.name);
                        content.push_str("\" => ");
                        content.push_str(&attr.ty);
                        if attr_index != settings.attributes.len() - 1 {
                            content.push_str(", ");
                        }
                    }

                    content.push_str(" ]\n    }");
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
