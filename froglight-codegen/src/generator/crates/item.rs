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
    pub display: String,
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
            let code = data
                .get_class_method_code("net/minecraft/world/item/Items", "<clinit>", None)
                .unwrap();

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
                            && class_name == "net/minecraft/world/level/block/Blocks"
                            && matches!(
                                name_and_type.descriptor.as_ref(),
                                "Lnet/minecraft/world/level/block/Block;"
                                    | "Lnet/minecraft/world/level/block/WeatheringCopperBlocks;"
                                    | "Lnet/minecraft/world/level/block/WeatheringCopperCollection;"
                            ) =>
                    {
                        if let Some(settings) = block_data.blocks.get(name_and_type.name.as_ref()) {
                            name = Some(name_and_type.name.to_string());
                            constant = Some(settings.ident.clone());
                            block = Some(true);
                        } else {
                            miette::bail!(
                                "Unknown block reference in `Items` <clinit>: {}",
                                name_and_type.name
                            );
                        }
                    }
                    Opcode::Getstatic(MemberRef { class_name, name_and_type })
                        if constant.is_none()
                            && class_name == "net/minecraft/world/level/block/Blocks"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/level/block/ColorCollection;" =>
                    {
                        if let Some(settings) =
                            block_data.blocks.get(&format!("WHITE_{}", name_and_type.name))
                        {
                            name = Some(name_and_type.name.to_string());
                            constant = Some(settings.ident.clone());
                            block = Some(true);
                        } else {
                            miette::bail!(
                                "Unknown colored block reference in {} `Items` <clinit>: {}",
                                version.as_str(),
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
                                None,
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
                            constant = Some(format!("minecraft:{extracted}"));
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
                            miette::bail!(
                                "Putstatic without preceding constant in Items <clinit>: {}",
                                name_and_type.name
                            );
                        };

                        items.insert(
                            name.take().unwrap_or_else(|| name_and_type.name.to_case(Case::Pascal)),
                            ItemSettings { ident: constant, display: String::new(), block },
                        );
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/item/Items"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/item/WeatheringCopperItems;" =>
                    {
                        let (Some(constant), Some(block)) = (constant.take(), block.take()) else {
                            miette::bail!(
                                "Putstatic without preceding constant in Items <clinit>: {}",
                                name_and_type.name
                            );
                        };

                        add_weathering_items(
                            ItemSettings { ident: constant, display: String::new(), block },
                            name.take().unwrap_or_else(|| name_and_type.name.to_case(Case::Pascal)),
                            &mut items,
                            version,
                        );
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/item/Items"
                            && matches!(
                                name_and_type.descriptor.as_ref(),
                                "Lnet/minecraft/world/item/WeatheringCopperItems;"
                                    | "Lnet/minecraft/world/level/block/WeatheringCopperCollection"
                            ) =>
                    {
                        let (Some(constant), Some(block)) = (constant.take(), block.take()) else {
                            miette::bail!(
                                "Putstatic without preceding constant in Items <clinit>: {}",
                                name_and_type.name
                            );
                        };

                        add_weathering_items(
                            ItemSettings { ident: constant, display: String::new(), block },
                            name.take().unwrap_or_else(|| name_and_type.name.to_case(Case::Pascal)),
                            &mut items,
                            version,
                        );
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == "net/minecraft/world/item/Items"
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/level/block/ColorCollection;" =>
                    {
                        let (constant, block) = if let (Some(constant), Some(block)) =
                            (constant.take(), block.take())
                        {
                            (constant, block)
                        } else if name_and_type.name == "DYE" {
                            (String::from("Dye"), false)
                        } else {
                            miette::bail!(
                                "Putstatic without preceding constant in Items <clinit>: {}",
                                name_and_type.name
                            );
                        };

                        add_colored_items(
                            ItemSettings { ident: constant, display: String::new(), block },
                            name.take().unwrap_or_else(|| name_and_type.name.to_case(Case::Pascal)),
                            &mut items,
                            version,
                        );
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type }) => {
                        if name_and_type.descriptor.ends_with(
                            "Lnet/minecraft/world/level/block/WeatheringCopperCollection;",
                        ) {
                            continue;
                        }

                        tracing::warn!(
                            "Unexpected static in Items <clinit>: {class_name}.{}:{}",
                            name_and_type.name,
                            name_and_type.descriptor
                        );
                    }

                    _ => {}
                }
            }

            Ok(())
        })
        .await?;

        for (item_name, item) in &mut items {
            // Update the display name
            item.display.clone_from(item_name);
            item.display = item.display.replace("_Dyed", "").replace("_DYED", "");
            if !item.ident.contains("block") {
                item.display = item.display.trim_end_matches("_BLOCK").to_string();
            }

            tracing::info!("{} -> {}", item_name, item.display);
        }

        tracing::debug!("Found {} items for \"{}\"", items.len(), version.as_str());

        Ok(ItemData { items })
    }
}

fn add_weathering_items(
    original: ItemSettings,
    key: String,
    items: &mut IndexMap<String, ItemSettings>,
    version: &Version,
) {
    let unwaxed: (String, String);
    let unwaxed_exposed: (String, String);
    let unwaxed_weathered: (String, String);
    let unwaxed_oxidized: (String, String);
    let waxed: (String, String);
    let waxed_exposed: (String, String);
    let waxed_weathered: (String, String);
    let waxed_oxidized: (String, String);

    match version.as_feature() {
        version if version.as_str() >= "v26_2" => {
            unwaxed = (key.clone(), original.ident.clone());
            waxed = (format!("Waxed{key}"), original.ident.replace(':', ":waxed_"));

            // Note:
            // v26_2 started removing the "_block" suffix from weathered copper block states
            let trimmed = original.ident.trim_end_matches("_block");

            unwaxed_exposed = (format!("Exposed{key}"), trimmed.replace(':', ":exposed_"));
            waxed_exposed = (format!("WaxedExposed{key}"), trimmed.replace(':', ":waxed_exposed_"));

            unwaxed_weathered = (format!("Weathered{key}"), trimmed.replace(':', ":weathered_"));
            waxed_weathered =
                (format!("WaxedWeathered{key}"), trimmed.replace(':', ":waxed_weathered_"));

            unwaxed_oxidized = (format!("Oxidized{key}"), trimmed.replace(':', ":oxidized_"));
            waxed_oxidized =
                (format!("WaxedOxidized{key}"), trimmed.replace(':', ":waxed_oxidized_"));
        }
        _ => {
            unwaxed = (key.clone(), original.ident.clone());
            waxed = (format!("Waxed{key}"), original.ident.replace(':', ":waxed_"));

            unwaxed_exposed = (format!("Exposed{key}"), original.ident.replace(':', ":exposed_"));
            waxed_exposed =
                (format!("WaxedExposed{key}"), original.ident.replace(':', ":waxed_exposed_"));

            unwaxed_weathered =
                (format!("Weathered{key}"), original.ident.replace(':', ":weathered_"));
            waxed_weathered =
                (format!("WaxedWeathered{key}"), original.ident.replace(':', ":waxed_weathered_"));

            unwaxed_oxidized =
                (format!("Oxidized{key}"), original.ident.replace(':', ":oxidized_"));
            waxed_oxidized =
                (format!("WaxedOxidized{key}"), original.ident.replace(':', ":waxed_oxidized_"));
        }
    }

    items.insert(unwaxed.0, ItemSettings { ident: unwaxed.1, ..original.clone() });
    items.insert(unwaxed_exposed.0, ItemSettings { ident: unwaxed_exposed.1, ..original.clone() });
    items.insert(
        unwaxed_weathered.0,
        ItemSettings { ident: unwaxed_weathered.1, ..original.clone() },
    );
    items
        .insert(unwaxed_oxidized.0, ItemSettings { ident: unwaxed_oxidized.1, ..original.clone() });
    items.insert(waxed.0, ItemSettings { ident: waxed.1, ..original.clone() });
    items.insert(waxed_exposed.0, ItemSettings { ident: waxed_exposed.1, ..original.clone() });
    items.insert(waxed_weathered.0, ItemSettings { ident: waxed_weathered.1, ..original.clone() });
    items.insert(waxed_oxidized.0, ItemSettings { ident: waxed_oxidized.1, ..original.clone() });
}

fn add_colored_items(
    original: ItemSettings,
    key: String,
    items: &mut IndexMap<String, ItemSettings>,
    _version: &Version,
) {
    let colors = [
        "WHITE",
        "ORANGE",
        "MAGENTA",
        "LIGHT_BLUE",
        "YELLOW",
        "LIME",
        "PINK",
        "GRAY",
        "LIGHT_GRAY",
        "CYAN",
        "PURPLE",
        "BLUE",
        "BROWN",
        "GREEN",
        "RED",
        "BLACK",
    ];

    let fixed = original.ident.replace("white_", "");
    for color in colors {
        let colored_name = format!("{color}_{key}");
        let colored_ident = fixed.replace(':', &format!(":{}_", color.to_lowercase()));

        items.insert(colored_name, ItemSettings { ident: colored_ident, ..original.clone() });
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate global item data.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<()>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async move {
            // Collect the `ItemData` for all versions.
            let global_items = VersionHelper::for_all_vec(config, async |version| {
                let pinned = DATA.pin_owned();
                let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);
                let mut storage = storage.write().await;
                ItemData::get_for(&version.real, &mut storage, async |data| Ok(data.clone())).await
            })
            .await?;

            // Deduplicate and sort the item types
            let mut items = IndexMap::new();
            for versioned in global_items {
                for settings in versioned.items.values() {
                    match items.entry(settings.display.to_case(Case::Pascal)) {
                        Entry::Vacant(entry) => {
                            entry.insert(settings.block);
                        }
                        Entry::Occupied(entry) => {
                            if entry.get() != &settings.block {
                                miette::bail!(
                                    "Inconsistent block status for item \"{}\"",
                                    settings.display
                                );
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
        Ok(()) => {
            let path = WORKSPACE_DIR.join("froglight-item/src/generated");
            let mut module = ModuleBuilder::new_after_marker(path).await?;

            for version in &config.versions {
                let guard = DATA.owned_guard();
                let storage =
                    DATA.get_or_insert_with(version.real.clone(), RwLock::default, &guard);

                module
                    .with_submodule(&version.base.as_feature(), async |module, settings| {
                        let mut storage = storage.write().await;
                        generate(version, module, &mut storage).await?;
                        Ok(settings.with_feature(version.base.as_feature()))
                    })
                    .await?;
            }

            module.build().await
        }
        Err(err) => miette::bail!("Failed to generate global biome data: {err}"),
    }
}

/// Generate `Version`-specific item data.
async fn generate(
    version: &VersionPair,
    module: &mut ModuleBuilder,
    storage: &mut VersionStorage,
) -> Result<()> {
    ItemData::get_for(&version.real, storage, async |data| {
        let mut content = String::new();

        let version_type = version.base.as_feature().to_ascii_uppercase();
        content.push_str("\nuse froglight_common::version::");
        content.push_str(&version_type);
        content.push_str(";\n\n#[allow(clippy::wildcard_imports, reason = \"Generated code\")]\nuse crate::{generated::item::*, item::ComponentData};\n\n");

        content.push_str("generate! {\n    @version ");
        content.push_str(&version_type);
        content.push_str(",\n");

        for (index, settings) in data.items.values().enumerate() {
            content.push_str("    ");
            content.push_str(&settings.display.to_case(Case::Pascal));
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

        Ok(())
    })
    .await
}
