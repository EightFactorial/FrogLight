use cafebabe::{
    bytecode::Opcode,
    constant_pool::{BootstrapArgument, InvokeDynamic, LiteralConstant, Loadable, MemberRef},
};
use indexmap::IndexMap;
use miette::Result;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    common::{DATA, Version, VersionStorage},
    config::{ConfigBundle, VersionPair},
    helper::{ClassFileExt, VersionHelper},
    source::JarData,
};

mod metadata;

#[derive(Debug, Clone, PartialEq)]
pub struct EntityData {
    pub entities: IndexMap<String, EntityInfo>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EntityInfo {
    pub id: String,
    pub class: Option<String>,
    pub metadata: Vec<EntityMetadataItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityMetadataItem {
    pub name: String,
    pub serializer: String,
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

        JarData::get_for(version, storage, async |data| {
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

        tracing::debug!("Found {} entities for \"{}\"", entities.len(), version.as_str());

        Ok(EntityData { entities })
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate global entity data.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<()>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async move {
            // Collect the `EntityData` for all versions.
            let _global_entities = VersionHelper::for_all_vec(config, async |version| {
                let pinned = DATA.pin_owned();
                let storage = pinned.get_or_insert_with(version.real.clone(), RwLock::default);
                let mut storage = storage.write().await;
                EntityData::get_for(&version.real, &mut storage, async |data| Ok(data.clone()))
                    .await
            })
            .await?;

            Ok(())
        })
        .await;

    match result {
        Ok(()) => Ok(()),
        Err(err) => miette::bail!("Failed to generate global entity data: {err}"),
    }
}

// -------------------------------------------------------------------------------------------------

/// Generate `Version`-specific entity data.
async fn generate(version: &VersionPair, storage: &mut VersionStorage) -> Result<()> {
    EntityData::get_for(&version.real, storage, async |_data| Ok(())).await
}
