use std::fmt::Write;

use cafebabe::{
    bytecode::Opcode,
    constant_pool::{BootstrapArgument, InvokeDynamic, LiteralConstant, Loadable, MemberRef},
};
use convert_case::{Case, Casing};
use indexmap::{IndexMap, IndexSet};
use miette::Result;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    common::{DATA, Version, VersionStorage, WORKSPACE_DIR},
    config::{ConfigBundle, VersionPair},
    helper::{ClassFileExt, ModuleBuilder, VersionHelper},
    source::JarData,
};

mod metadata;

#[derive(Debug, Clone, PartialEq)]
pub struct EntityData {
    pub entities: IndexMap<String, EntityInfo>,
    pub metadata_classes: IndexMap<EntityMetadataItem, Vec<String>>,
    pub datatypes: IndexSet<(String, String)>,
    pub datatype_order: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct EntityInfo {
    pub id: String,
    pub class: Option<String>,
    pub metadata: Vec<EntityMetadataItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityMetadataItem {
    pub name: String,
    pub registered_class: String,

    pub serializer_name: String,
    pub serializer_class: String,
}

impl std::hash::Hash for EntityMetadataItem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.serializer_name.hash(state);
    }
}

impl EntityData {
    /// Get the [`EntityData`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `EntityData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`EntityData`] for the given [`Version`].
    #[allow(clippy::too_many_lines, reason = "Yes")]
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut entities = IndexMap::new();
        let mut datatype_order = Vec::new();

        JarData::get_for(version, storage, async |data| {
            datatype_order = metadata::parse_serializer_order(data);

            let class = data.get_class("net/minecraft/world/entity/EntityType").unwrap();
            let code = class.get_method_code("<clinit>").unwrap();
            let mut skip = true;

            let mut constants = Vec::<LiteralConstant>::new();

            let mut current = EntityInfo::default();
            let default_meta = metadata::parse_entity_constructor("net/minecraft/world/entity/Entity", data)?;

            let bytecode = code.bytecode.as_ref().unwrap();
            class.iterate_code(bytecode, data, 0, &mut |_, op| {
                match op {
                    Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(s)))
                        if current.id.is_empty() =>
                    {
                        current.id = s.to_string();
                    }
                    Opcode::Ldc(Loadable::LiteralConstant(c))
                    | Opcode::LdcW(Loadable::LiteralConstant(c))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(c)) => {
                        constants.push(c.clone());
                    }
                    Opcode::Iconst0 => {
                        constants.push(LiteralConstant::Integer(0));
                    }
                    Opcode::Iconst1 => {
                        constants.push(LiteralConstant::Integer(1));
                    }
                    Opcode::Iconst2 => {
                        constants.push(LiteralConstant::Integer(2));
                    }
                    Opcode::Iconst3 => {
                        constants.push(LiteralConstant::Integer(3));
                    }
                    Opcode::Iconst4 => {
                        constants.push(LiteralConstant::Integer(4));
                    }
                    Opcode::Iconst5 => {
                        constants.push(LiteralConstant::Integer(5));
                    }
                    Opcode::Lconst0 => {
                        constants.push(LiteralConstant::Long(0));
                    }
                    Opcode::Lconst1 => {
                        constants.push(LiteralConstant::Long(1));
                    }
                    Opcode::Fconst0 => {
                        constants.push(LiteralConstant::Float(0.0));
                    }
                    Opcode::Fconst1 => {
                        constants.push(LiteralConstant::Float(1.0));
                    }
                    Opcode::Fconst2 => {
                        constants.push(LiteralConstant::Float(2.0));
                    }
                    Opcode::Dconst0 => {
                        constants.push(LiteralConstant::Double(0.0));
                    }
                    Opcode::Dconst1 => {
                        constants.push(LiteralConstant::Double(1.0));
                    }

                    Opcode::Invokeinterface(MemberRef { class_name, name_and_type }, _)
                    | Opcode::Invokespecial(MemberRef { class_name, name_and_type })
                    | Opcode::Invokestatic(MemberRef { class_name, name_and_type })
                    | Opcode::Invokevirtual(MemberRef { class_name, name_and_type })
                        if class_name.starts_with("net/minecraft/world/entity")
                            && name_and_type.name != "<init>"
                            && !skip =>
                    {
                        #[expect(clippy::match_same_arms, reason = "Easier to read")]
                        match (name_and_type.name.as_ref(), class_name.as_ref()) {
                            ("attach", _) => {}
                            ("boatFactory", _) => {
                                current.class = Some(String::from(
                                    "net/minecraft/world/entity/vehicle/boat/Boat",
                                ));
                            }
                            ("canPickUpLoot", _) => {}
                            ("canSpawnFarFromPlayer", _) => {}
                            ("chestBoatFactory", _) => {
                                current.class = Some(String::from(
                                    "net/minecraft/world/entity/vehicle/boat/ChestBoat",
                                ));
                            }
                            ("chestRaftFactory", _) => {
                                current.class = Some(String::from(
                                    "net/minecraft/world/entity/vehicle/boat/ChestRaft",
                                ));
                            }
                            ("clientTrackingRange", _) => {}
                            ("createNothing", _) => {}
                            ("eyeHeight", _) => {}
                            ("fireImmune", _) => {}
                            ("immuneTo", _) => {}
                            ("nameTagOffset", _) => {}
                            ("noLootTable", _) => {}
                            ("noSave", _) => {}
                            ("noSummon", _) => {}
                            ("notInPeaceful", _) => {}
                            ("passengerAttachments", _) => {}
                            ("raftFactory", _) => {
                                current.class = Some(String::from(
                                    "net/minecraft/world/entity/vehicle/boat/Raft",
                                ));
                            }
                            ("ridingOffset", _) => {}
                            ("setCanPickUpLoot", _) => {}
                            ("sized", _) => {}
                            ("spawnDimensionsScale", _) => {}
                            ("updateInterval", _) => {}
                            ("vehicleAttachment", _) => {}
                            // ("", _) => {}

                            // Skip
                            ("getBrain", _) => {}
                            ("getMaxHealth", _) => {}
                            ("getNavigation", _) => {}
                            ("getRandom", _) => {}
                            ("of", "net/minecraft/world/entity/EntityType$Builder") => {}
                            ("of", "net/minecraft/world/entity/SlotAccess") => {}
                            ("pickNextScuteDropTime", _) => {}
                            ("reassessTameGoals", _) => {}
                            ("register", "net/minecraft/world/entity/EntityType") => {}
                            ("setCanFloat", _) => {}
                            ("setCanOpenDoors", _) => {}
                            ("setCanWalkOverFences", _) => {}
                            ("setHealth", _) => {}
                            ("setInterval", _) => {}
                            ("setMemory", _) => {}
                            ("setPathfindingMalus", _) => {}
                            ("setPersistenceRequired", _) => {}
                            ("setRequiredPathLength", _) => {}
                            ("setResting", _) => {}
                            ("setState", _) => {}
                            ("setYRot", _) => {}
                            ("setInvisible", _) => {}
                            ("create", "net/minecraft/world/entity/EntityType$EntityFactory") => {}
                            ("reassessTrustingGoals", _) => {}
                            ("refreshDimensions", _) => {}
                            ("setSpeedModifier", _) => {}
                            ("fixupDimensions", _) => {}
                            ("isBaby", _) => {}
                            ("createTicker", _) => {}
                            ("getId", _) => {}
                            ("getDisplayName", _) => {}
                            ("setTame", _) => {}

                            (method, class) => {
                                miette::bail!("Unknown \"Entity\" method: \"{class}.{method}\"")
                            }
                        }
                    }
                    Opcode::Invokedynamic(InvokeDynamic { attr_index, .. }) => {
                        let entry = &class.get_bootstrap().unwrap()[usize::from(*attr_index)];
                        for arg in &entry.arguments {
                            if let BootstrapArgument::MethodHandle(handle) = arg
                                && handle.member_ref.descriptor.starts_with("(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)")
                            {
                                current.class = Some(handle.class_name.to_string());
                                current.metadata.clone_from(&default_meta);
                                current.metadata.extend(metadata::parse_metadata_method(&handle.class_name, data)?);
                            }
                        }
                    }

                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == &*class.this_class
                            && name_and_type.descriptor
                                == "Lnet/minecraft/world/entity/EntityType;" =>
                    {
                        entities
                            .insert(name_and_type.name.to_string(), core::mem::take(&mut current));
                        constants.clear();
                    }
                    Opcode::Putstatic(MemberRef { class_name, name_and_type })
                        if class_name == &*class.this_class
                            && name_and_type.name == "STREAM_CODEC"
                            && skip =>
                    {
                        skip = false;
                        constants.clear();
                    }

                    _ => {}
                }

                Ok(())
            })?;

            Ok(())
        })
        .await?;

        let mut metadata_classes = IndexMap::<_, Vec<_>>::new();
        for entity in entities.values() {
            for meta in &entity.metadata {
                let entry = metadata_classes.entry(meta.clone()).or_default();
                if let Some(class) = &entity.class {
                    entry.push(class.clone());
                }
            }
        }

        let mut datatypes = IndexSet::new();
        for entity in entities.values() {
            for meta in &entity.metadata {
                datatypes.insert((meta.serializer_name.clone(), meta.serializer_class.clone()));
            }
        }
        for datatype in &datatype_order {
            if datatypes.iter().all(|(name, _)| name != datatype) {
                datatypes.insert((
                    datatype.clone(),
                    String::from("net/minecraft/network/syncher/EntityDataSerializers"),
                ));
            }
        }

        tracing::debug!("Found {} entities for \"{}\"", entities.len(), version.as_str());
        Ok(EntityData { entities, metadata_classes, datatypes, datatype_order })
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate global entity data.
#[allow(clippy::too_many_lines, reason = "Aaa")]
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<()>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async move {
            // Collect the `EntityData` for all versions.
            let global_entities = VersionHelper::for_all_vec(config, async |version| {
                let pinned = DATA.pin_owned();
                let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);

                let mut storage = storage.write().await;
                EntityData::get_for(&version.real, &mut storage, async |data| Ok(data.clone()))
                    .await
            })
            .await?;

            // Generate `datatypes`
            {
                let folder = WORKSPACE_DIR.join("froglight-entity/src/generated/");
                let mut module = ModuleBuilder::new("datatype", folder);

                let mut unique = IndexSet::new();
                for data in &global_entities {
                    unique.extend(data.datatypes.iter().cloned());
                }
                unique.sort_unstable();

                let mut content = String::from("\n\nuse alloc::borrow::Cow;\n\nuse crate::entity::{VarInt, VarLong};\n#[cfg(feature = \"bevy\")]\nuse bevy_ecs::reflect::ReflectComponent;\n#[cfg(feature = \"facet\")]\nuse facet_minecraft as mc;\n\ngenerate! {\n    @datatypes");
                for (index, (name, class)) in unique.iter().enumerate() {
                    content.push_str("\n    as_");
                    content.push_str(&name.to_ascii_lowercase());
                    content.push_str(" => ");
                    content.push_str(&name.to_case(Case::Pascal));
                    content.push('(');
                    content.push_str(datatype_type(name, class)?);
                    content.push(')');
                    if index != unique.len() - 1 {
                        content.push(',');
                    }
                }
                content.push_str("\n}\n");

                module.with_docs("Placeholder").with_content(&content);
                module.build().await?;
            }

            // Generate `components`
            {
                let folder = WORKSPACE_DIR.join("froglight-entity/src/generated/");
                let mut module = ModuleBuilder::new("component", folder);

                let mut unique = IndexMap::<_, Vec<_>>::new();
                for data in &global_entities {
                    unique.extend(data.metadata_classes.clone());
                }
                unique.sort_unstable_by_key(|a, _| a.name.clone());

                let mut content = String::from("\n\nuse alloc::borrow::Cow;\n\nuse crate::entity::{VarInt, VarLong};\n#[cfg(feature = \"bevy\")]\nuse bevy_ecs::reflect::ReflectComponent;\n\ngenerate! {\n    @components");
                for (index, (meta, classes)) in unique.iter().enumerate() {
                    content.push_str("\n    ");
                    content.push_str(&component_name(meta, classes));
                    content.push('(');
                    content.push_str(datatype_type(&meta.serializer_name, &meta.serializer_class)?);
                    content.push_str(") = ");
                    content.push_str(&meta.serializer_name.to_case(Case::Pascal));
                    if index != unique.len() - 1 {
                        content.push(',');
                    }
                }
                content.push_str("\n}\n");

                module.with_docs("Placeholder").with_content(&content);
                module.build().await?;
            }

            // Generate `entities`
            {
                let folder = WORKSPACE_DIR.join("froglight-entity/src/generated/");
                let mut module = ModuleBuilder::new("entity", folder);

                let mut unique = IndexSet::new();
                for data in &global_entities {
                    unique.extend(data.entities.values().map(|v| v.id.clone()));
                }
                unique.sort_unstable();

                let mut content = String::from("#![allow(clippy::large_stack_arrays, reason = \"Triggered by Facet\")]\n\n#[cfg(feature = \"bevy\")]\nuse bevy_ecs::reflect::ReflectComponent;\n\ngenerate! {\n    @entities");
                for (index, id) in unique.iter().enumerate() {
                    content.push_str("\n    ");
                    content.push_str(&entity_name(id));
                    if index != unique.len() - 1 {
                        content.push(',');
                    }
                }
                content.push_str("\n}\n");

                module.with_docs("Placeholder").with_content(&content);
                module.build().await?;
            }

            // Generate `versions`
            {
                let path = WORKSPACE_DIR.join("froglight-entity/src/generated/mod.rs");
                let mut module = ModuleBuilder::new_after_marker(path).await?;

                for version in &config.versions {
                    let pinned = DATA.pin_owned();
                    let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);

                    let mut storage = storage.write().await;
                    module.with_submodule(&version.base.as_feature(), async |module, settings| {
                        generate(version, module, &mut storage).await?;
                        Ok(settings.with_feature(version.base.as_feature()))
                    }).await?;
                }
            }

            Ok(())
        })
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(err) => miette::bail!("Failed to generate global entity data: {err}"),
    }
}

fn entity_name(id: &str) -> String {
    let name = id.split(':').next_back().unwrap();
    let mut name = name.to_case(Case::Pascal);

    if name == "Item" {
        name = String::from("ItemEntity");
    }

    name
}

fn component_name(meta: &EntityMetadataItem, classes: &[String]) -> String {
    let name = meta.name.trim_start_matches("DATA_");
    let mut name = name.to_case(Case::Pascal);

    {
        let mut prefix = meta.registered_class.split('/').next_back().unwrap();
        if let Some((first, _last)) = prefix.split_once('$') {
            prefix = first;
        }

        let prefix = prefix.to_case(Case::Pascal);
        if !name.starts_with(&prefix) {
            name = format!("{prefix}{name}");
        }
    }

    if classes.len() == 1 {
        let class = &classes[0];
        let mut prefix = class.split('/').next_back().unwrap();
        if let Some((first, _last)) = prefix.split_once('$') {
            prefix = first;
        }

        let prefix = prefix.to_case(Case::Pascal);
        if !name.starts_with(&prefix) {
            name = format!("{prefix}{name}");
        }
    }

    if name == "Item" {
        name = String::from("ItemType");
    }

    name
}

#[allow(clippy::unnecessary_wraps, reason = "For now")]
fn datatype_type(name: &str, class: &str) -> Result<&'static str> {
    match (name, class) {
        ("BOOLEAN", "net/minecraft/network/syncher/EntityDataSerializers") => Ok("bool"),
        ("BYTE", "net/minecraft/network/syncher/EntityDataSerializers") => Ok("u8"),
        ("INT", "net/minecraft/network/syncher/EntityDataSerializers") => Ok("VarInt"),
        ("LONG", "net/minecraft/network/syncher/EntityDataSerializers") => Ok("VarLong"),
        ("FLOAT", "net/minecraft/network/syncher/EntityDataSerializers") => Ok("f32"),
        ("DOUBLE", "net/minecraft/network/syncher/EntityDataSerializers") => Ok("f64"),
        ("STRING", "net/minecraft/network/syncher/EntityDataSerializers") => {
            Ok("Cow<'static, str>")
        }

        ("OPTIONAL_UNSIGNED_INT", "net/minecraft/network/syncher/EntityDataSerializers") => {
            Ok("Option<VarInt>")
        }

        // TODO
        (
            "ARMADILLO_STATE"
            | "BLOCK_POS"
            | "BLOCK_STATE"
            | "CAT_SOUND_VARIANT"
            | "CAT_VARIANT"
            | "CHICKEN_SOUND_VARIANT"
            | "CHICKEN_VARIANT"
            | "COMPONENT"
            | "COPPER_GOLEM_STATE"
            | "COW_SOUND_VARIANT"
            | "COW_VARIANT"
            | "DIRECTION"
            | "FROG_VARIANT"
            | "HUMANOID_ARM"
            | "ITEM_STACK"
            | "PAINTING_VARIANT"
            | "PARTICLE"
            | "PARTICLES"
            | "PIG_SOUND_VARIANT"
            | "PIG_VARIANT"
            | "POSE"
            | "QUATERNION"
            | "RESOLVABLE_PROFILE"
            | "ROTATIONS"
            | "SNIFFER_STATE"
            | "VECTOR3"
            | "VILLAGER_DATA"
            | "WEATHERING_COPPER_STATE"
            | "WOLF_SOUND_VARIANT"
            | "WOLF_VARIANT"
            | "ZOMBIE_NAUTILUS_VARIANT",
            "net/minecraft/network/syncher/EntityDataSerializers",
        ) => Ok("()"),
        (
            "OPTIONAL_COMPONENT"
            | "OPTIONAL_BLOCK_POS"
            | "OPTIONAL_BLOCK_STATE"
            | "OPTIONAL_LIVING_ENTITY_REFERENCE"
            | "OPTIONAL_GLOBAL_POS",
            "net/minecraft/network/syncher/EntityDataSerializers",
        ) => Ok("Option<()>"),

        _ => miette::bail!("Unknown entity datatype: \"{class}.{name}\")"),
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate `Version`-specific entity data.
async fn generate(
    version: &VersionPair,
    module: &mut ModuleBuilder,
    storage: &mut VersionStorage,
) -> Result<()> {
    EntityData::get_for(&version.real, storage, async |data| {
        let version_ident = version.base.as_feature().to_ascii_uppercase();
        let mut content = format!(
            "use froglight_common::version::{version_ident};
use crate::entity::{{VarInt, VarLong}};
#[expect(clippy::wildcard_imports, reason = \"Generated code\")]
use crate::generated::{{component::*, entity::*}};

generate! {{
    @version {version_ident},
    datatypes: {{
"
        );

        for (index, datatype) in data.datatype_order.iter().enumerate() {
            let (_, class) = data.datatypes.iter().find(|(name, _)| name == datatype).unwrap();

            write!(
                content,
                "        {}({}) = {index}",
                datatype.to_case(Case::Pascal),
                datatype_type(datatype, class)?,
            )
            .unwrap();
            if index != data.datatypes.len() - 1 {
                content.push(',');
            }
            content.push('\n');
        }

        content.push_str("    },\n");
        for (index, entity) in data.entities.values().enumerate() {
            let ident = entity_name(&entity.id);

            write!(
                content,
                "    {ident} => {{ ident: \"minecraft:{}\", global: {index},\n        components: [",
                entity.id,
            )
            .unwrap();

            for (index, component) in entity.metadata.iter().enumerate() {
                let classes = data.metadata_classes.get(component).unwrap();
                write!(content, " {} = {}", component_name(component, classes), index).unwrap();
                if index != entity.metadata.len() - 1 {
                    content.push(',');
                }
            }

            content.push_str(" ]\n    }");
            if index != data.entities.len() - 1 {
                content.push(',');
            }
            content.push('\n');
        }
        content.push('}');

        module.with_content(&content);
        Ok(())
    })
    .await
}
