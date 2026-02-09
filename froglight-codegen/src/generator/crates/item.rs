use std::fmt::Write;

use cafebabe::{
    bytecode::Opcode,
    constant_pool::{LiteralConstant, Loadable, MemberRef},
};
use convert_case::{Case, Casing};
use indexmap::{IndexMap, map::Entry};
use miette::Result;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    common::{DATA, Version, VersionStorage, WORKSPACE_DIR},
    config::{ConfigBundle, VersionPair},
    generator::crates::block::BlockData,
    helper::{ModuleBuilder, VersionHelper},
    source::JarData,
};

#[derive(Debug, Clone)]
pub struct ItemData {
    pub items: IndexMap<String, ItemSettings>,
}

#[derive(Debug, Default, Clone)]
pub struct ItemSettings {
    pub block: bool,
    pub ident: String,
}

impl ItemData {
    /// Get the [`ItemData`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `ItemData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`ItemData`] for the given [`Version`].
    #[allow(clippy::too_many_lines, reason = "Necessary for parsing block and entity bytecode")]
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut items = IndexMap::new();

        let block_data =
            BlockData::get_for(version, storage, async |data| Ok(data.clone())).await?;
        JarData::get_for(version, storage, async |data| {
            let code =
                data.get_class_method_code("net/minecraft/world/item/Items", "<clinit>").unwrap();

            let mut name = None;
            let mut constant = None;
            let mut block = None;
            for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
                match op {
                    Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(s)))
                        if constant.is_none() =>
                    {
                        constant = Some(format!("minecraft:{s}"));
                        block = Some(false);
                    }
                    Opcode::Getstatic(MemberRef { class_name, name_and_type })
                        if constant.is_none()
                            && class_name == "net/minecraft/world/level/block/Blocks" =>
                    {
                        if let Some(settings) = block_data.blocks.get(name_and_type.name.as_ref()) {
                            name = Some(name_and_type.name.to_string());
                            constant = Some(settings.ident.clone());
                            block = Some(true);
                        } else {
                            miette::bail!(
                                "Unknown block reference in Items <clinit>: {}",
                                name_and_type.name
                            );
                        }
                    }
                    Opcode::Getstatic(MemberRef { class_name, name_and_type })
                        if constant.is_none()
                            && class_name == "net/minecraft/world/entity/EntityType" =>
                    {
                        let code = data
                            .get_class_method_code(
                                "net/minecraft/world/entity/EntityType",
                                "<clinit>",
                            )
                            .unwrap();

                        // Find the string constant used to initialize the entity type
                        let mut extracted = None;
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
                                    extracted = Some(s);
                                }
                                Opcode::Putstatic(MemberRef {
                                    class_name: constant_class,
                                    name_and_type: constant_field,
                                }) if constant_class == class_name
                                    && constant_field.name == name_and_type.name =>
                                {
                                    break;
                                }
                                _ => {}
                            }
                        }

                        if let Some(extracted) = extracted {
                            constant = Some(format!("minecraft:{extracted}",));
                            block = Some(false);
                        } else {
                            miette::bail!(
                                "Failed to extract entity constant for \"{}\" in Items <clinit>",
                                name_and_type.name
                            );
                        }
                    }

                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/item/Items"
                            && name_and_type.descriptor == "Lnet/minecraft/world/item/Item;" =>
                    {
                        let (Some(constant), Some(block)) = (constant.take(), block.take()) else {
                            tracing::warn!(
                                "Putstatic without preceding constant in Items <clinit>: {}",
                                name_and_type.name
                            );
                            continue;
                        };

                        items.insert(
                            name.take().unwrap_or_else(|| name_and_type.name.to_case(Case::Pascal)),
                            ItemSettings { ident: constant, block },
                        );
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/item/Items"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/item/WeatheringCopperItems;" =>
                    {
                        let (Some(constant), Some(block)) = (constant.take(), block.take()) else {
                            tracing::warn!(
                                "Putstatic without preceding constant in Items <clinit>: {}",
                                name_and_type.name
                            );
                            continue;
                        };

                        let key =
                            name.take().unwrap_or_else(|| name_and_type.name.to_case(Case::Pascal));
                        items.insert(key.clone(), ItemSettings { ident: constant.clone(), block });

                        items.insert(
                            format!("Exposed{key}"),
                            ItemSettings { ident: constant.replace(':', ":exposed_"), block },
                        );
                        items.insert(
                            format!("Weathered{key}"),
                            ItemSettings { ident: constant.replace(':', ":weathered_"), block },
                        );
                        items.insert(
                            format!("Oxidized{key}"),
                            ItemSettings { ident: constant.replace(':', ":oxidized_"), block },
                        );
                        items.insert(
                            format!("Waxed{key}"),
                            ItemSettings { ident: constant.replace(':', ":waxed_"), block },
                        );
                        items.insert(
                            format!("WaxedExposed{key}"),
                            ItemSettings { ident: constant.replace(':', ":waxed_exposed_"), block },
                        );
                        items.insert(
                            format!("WaxedWeathered{key}"),
                            ItemSettings {
                                ident: constant.replace(':', ":waxed_weathered_"),
                                block,
                            },
                        );
                        items.insert(
                            format!("WaxedOxidized{key}"),
                            ItemSettings {
                                ident: constant.replace(':', ":waxed_oxidized_"),
                                block,
                            },
                        );
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/item/Items" =>
                    {
                        tracing::warn!(
                            "Unexpected static in Items <clinit>: {}",
                            name_and_type.descriptor
                        );
                    }

                    _ => {}
                }
            }

            Ok(())
        })
        .await?;

        tracing::debug!("Found {} items for \"{}\"", items.len(), version.as_str());

        Ok(ItemData { items })
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate global item data.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<()>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async move {
            // Collect the `ItemData` for all versions.
            let global_items = VersionHelper::for_all(config, async |version| {
                let pinned = DATA.pin_owned();
                let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);
                let mut storage = storage.write().await;
                ItemData::get_for(&version.real, &mut storage, async |data| Ok(data.clone())).await
            })
            .await?;

            // Deduplicate and sort the item types
            let mut items = IndexMap::new();
            for versioned in global_items {
                for (name, settings) in versioned.items {
                    match items.entry(name.to_case(Case::Pascal)) {
                        Entry::Vacant(entry) => {
                            entry.insert(settings.block);
                        }
                        Entry::Occupied(entry) => {
                            if entry.get() != &settings.block {
                                miette::bail!("Inconsistent block status for item \"{name}\"");
                            }
                        }
                    }
                }
            }
            items.sort_unstable_keys();

            // Start building the module
            let path = WORKSPACE_DIR.join("froglight-item/src/generated");
            let mut module = ModuleBuilder::new("item", path);

            // Generate the content
            let mut content = String::new();
            content.push_str("\ngenerate! {\n    @items\n");
            for (identifier, block) in items {
                if block {
                    writeln!(content, "    @block {identifier},").unwrap();
                } else {
                    writeln!(content, "    {identifier},").unwrap();
                }
            }
            content.push('}');

            // Finalize and build the module
            module
                .with_docs(
                    "Item types for all [`Version`](froglight_common::version::Version)s.

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

/// Generate `Version`-specific item data.
pub async fn generate(version: &VersionPair, storage: &mut VersionStorage) -> Result<()> {
    ItemData::get_for(&version.real, storage, async |data| {
        let path = WORKSPACE_DIR.join("froglight-item/src/generated");
        let mut module = ModuleBuilder::new_after_marker(path).await?;

        module
            .with_submodule(&version.base.as_feature(), async |module, settings| {
                let mut content = String::new();

                let version_type = version.base.as_feature().to_ascii_uppercase();
                content.push_str("\nuse froglight_common::version::");
                content.push_str(&version_type);
                content.push_str(";\n\n#[allow(clippy::wildcard_imports, reason = \"Generated code\")]\nuse crate::{generated::item::*, item::ComponentData};\n\n");

                content.push_str("generate! {\n    @version ");
                content.push_str(&version_type);
                content.push_str(",\n");

                for (index, (item, settings)) in data.items.iter().enumerate() {
                    content.push_str("    ");
                    content.push_str(&item.to_case(Case::Pascal));
                    content.push_str(" => { ident: \"");
                    content.push_str(&settings.ident);
                    content.push_str("\", global: ");
                    content.push_str(&index.to_string());
                    content.push_str(" }");
                    if index != data.items.len() - 1 {
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
