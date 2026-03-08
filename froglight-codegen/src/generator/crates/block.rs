use core::cmp::Ordering;
use std::fmt::Write;

use cafebabe::{
    ClassFile,
    attributes::CodeData,
    bytecode::Opcode,
    constant_pool::{BootstrapArgument, InvokeDynamic, LiteralConstant, Loadable, MemberRef},
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
    helper::{ClassFileExt, ModuleBuilder, VersionHelper},
    source::{JarData, JarFile},
};

#[derive(Debug, Clone)]
pub struct BlockData {
    pub blocks: IndexMap<String, BlockSettings>,
    pub block_families: IndexMap<String, Vec<String>>,

    pub report: BlockReport,
}

#[derive(Debug, Clone, PartialEq)]
#[expect(clippy::struct_excessive_bools, reason = "False")]
pub struct BlockSettings {
    pub ident: String,
    pub classes: Vec<String>,

    pub is_air: bool,
    pub is_solid: bool,
    pub is_liquid: bool,
    pub has_collision: bool,
    pub has_occlusion: bool,

    pub shape: BlockShape,
    pub attributes: Vec<BlockAttribute>,

    /// Total states (min 1)
    pub states: usize,
    /// Zero-indexed
    pub default: usize,
}

impl Default for BlockSettings {
    fn default() -> Self {
        Self {
            ident: String::new(),
            classes: Vec::new(),
            is_air: false,
            is_solid: true,
            is_liquid: false,
            has_collision: true,
            has_occlusion: true,
            shape: BlockShape::default(),
            attributes: Vec::new(),
            states: 1,
            default: 0,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum BlockShape {
    Single([f64; 6]),
    Variable(Vec<[f64; 6]>),
    Custom(String),
    Unknown,
    #[default]
    Unset,
}

impl BlockShape {
    /// Returns true if the shape is [`BlockShape::Unknown`].
    #[must_use]
    pub const fn is_unknown(&self) -> bool { matches!(self, Self::Unknown) }

    /// Returns true if the shape is [`BlockShape::Unset`].
    #[must_use]
    pub const fn is_unset(&self) -> bool { matches!(self, Self::Unset) }
}

impl Eq for BlockShape {}
impl PartialEq for BlockShape {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(a), Self::Single(b)) => {
                a.iter().zip(b).all(|(a, b)| a.total_cmp(b) == Ordering::Equal)
            }
            (Self::Variable(a), Self::Variable(b)) => {
                a.len() == b.len()
                    && a.iter().zip(b).all(|(a, b)| {
                        a.iter().zip(b).all(|(a, b)| a.total_cmp(b) == Ordering::Equal)
                    })
            }
            (Self::Custom(a), Self::Custom(b)) => a == b,
            (Self::Unknown, Self::Unknown) | (Self::Unset, Self::Unset) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for BlockShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write_arr = |f: &mut std::fmt::Formatter<'_>, arr: &[f64; 6]| {
            write!(
                f,
                "{}f64, {}f64, {}f64, {}f64, {}f64, {}f64",
                arr[0], arr[1], arr[2], arr[3], arr[4], arr[5]
            )
        };

        match self {
            Self::Single(shape) => {
                write!(f, "BlockShape::new_xyz(")?;
                write_arr(f, shape)?;
                write!(f, ")")
            }
            Self::Variable(shapes) => {
                write!(f, "BlockShape::Collection(&[")?;
                for (i, shape) in shapes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "BlockAabb::new_xyz(")?;
                    write_arr(f, shape)?;
                    write!(f, ")")?;
                }
                write!(f, "])")
            }
            _ => write!(f, "BlockShape::FULL"),
        }
    }
}

impl std::hash::Hash for BlockShape {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Single(shape) => shape.iter().for_each(|f| f.to_bits().hash(state)),
            Self::Variable(shapes) => shapes.iter().for_each(|shape| {
                for f in shape {
                    f.to_bits().hash(state);
                }
            }),
            Self::Custom(name) => name.hash(state),
            _ => {}
        }
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
    #[facet(default)]
    pub properties: IndexMap<String, Vec<String>>,
    pub states: Vec<BlockReportState>,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct BlockReportState {
    #[facet(default)]
    pub default: bool,
    pub id: usize,
    #[facet(default)]
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
            let class = data.get_class("net/minecraft/world/level/block/Blocks").unwrap();
            let code = class.get_static_code().unwrap();

            let mut current = BlockSettings::default();
            let bytecode = code.bytecode.as_ref().unwrap();

            let mut constants = Vec::new();
            let mut retrieved: Option<MemberRef> = None;


            class.iterate_code(bytecode, data, 0, &mut |_, op| {
                match op {
                    Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(s)))
                    | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(s)))
                        if current.ident.is_empty() =>
                    {
                        current.ident = format!("minecraft:{s}");
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
                    Opcode::Getstatic(member @ MemberRef { class_name, name_and_type }) if name_and_type.descriptor == "Lnet/minecraft/world/level/block/Block;" => {
                        retrieved = Some(member.clone());
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
                        constants.clear();
                        retrieved = None;
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

                        constants.clear();
                        retrieved = None;
                    }
                    Opcode::Putstatic(..) => {
                        constants.clear();
                        retrieved = None;
                        current = BlockSettings::default();
                    }

                    Opcode::Invokeinterface(MemberRef { class_name, name_and_type }, _)
                    | Opcode::Invokespecial(MemberRef { class_name, name_and_type })
                    | Opcode::Invokestatic(MemberRef { class_name, name_and_type })
                    | Opcode::Invokevirtual(MemberRef { class_name, name_and_type })
                        if class_name.starts_with("net/minecraft/world/level/block/")
                             =>
                    {
                        // Add block classes based
                        if name_and_type.name == "<init>" {
                            current.classes.push(class_name.to_string());
                        } else if name_and_type.name == "register" && matches!(name_and_type.descriptor.as_ref(), "(Ljava/lang/String;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;" | "(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)Lnet/minecraft/world/level/block/Block;") {
                            current.classes.push(String::from("net/minecraft/world/level/block/Block"));
                        } else if name_and_type.name == "registerBed" {
                            current
                                .classes
                                .push(String::from("net/minecraft/world/level/block/BedBlock"));
                        } else if name_and_type.name == "registerStainedGlass" {
                            current
                                .classes
                                .push(String::from("net/minecraft/world/level/block/StainedGlassBlock"));
                        } else if matches!(
                            name_and_type.name.as_ref(),
                            "registerStair" | "registerLegacyStair"
                        ) {
                            current
                                .classes
                                .push(String::from("net/minecraft/world/level/block/StairBlock"));
                        }

                        // Add block properties
                        #[expect(clippy::match_same_arms, reason = "Easier to read")]
                        if name_and_type.descriptor.ends_with("Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;") {
                            match (name_and_type.name.as_ref(), name_and_type.descriptor.as_ref()) {
                                ("air", _) => current.is_air = true,
                                ("buttonProperties", _) => {}
                                ("candleProperties", _) => {}
                                ("dynamicShape", _) => {}
                                ("emissiveRendering", _) => {}
                                ("explosionResistance", _) => {}
                                ("flowerPotProperties", _) => {}
                                ("forceSolidOff", _) => {}
                                ("forceSolidOn", _) => {}
                                ("friction", _) => {}
                                ("hasPostProcess", _) => {}
                                ("ignitedByLava", _) => {}
                                ("instabreak", _) => {}
                                ("instrument", _) => {}
                                ("isRedstoneConductor", _) => {}
                                ("isSuffocating", _) => {}
                                ("isValidSpawn", _) => {}
                                ("isViewBlocking", _) => {}
                                ("jumpFactor", _) => {}
                                ("leavesProperties", _) => {}
                                ("lightLevel", _) => {}
                                ("liquid", _) => current.is_liquid = true,
                                ("litBlockEmission", _) => {}
                                ("logProperties", _) => {}
                                ("mapColor", _) => {}
                                ("netherStemProperties", _) => {}
                                ("noCollision", _) => {
                                    current.has_collision = false;
                                    current.has_occlusion = false;
                                },
                                ("noLootTable", _) => {}
                                ("noOcclusion", _) => current.has_occlusion = false,
                                ("noTerrainParticles", _) => {}
                                ("ofFullCopy", _) => {
                                    let retrieved = retrieved.as_ref().unwrap();
                                    let retrieved = blocks.get(retrieved.name_and_type.name.as_ref()).unwrap();
                                    // copyTo.jumpFactor = copyFrom.jumpFactor;
                                    // copyTo.isRedstoneConductor = copyFrom.isRedstoneConductor;
                                    // copyTo.isValidSpawn = copyFrom.isValidSpawn;
                                    // copyTo.hasPostProcess = copyFrom.hasPostProcess;
                                    // copyTo.isSuffocating = copyFrom.isSuffocating;
                                    // copyTo.isViewBlocking = copyFrom.isViewBlocking;
                                    // copyTo.drops = copyFrom.drops;
                                    // copyTo.destroyTime = copyFrom.destroyTime;
                                    // copyTo.explosionResistance = copyFrom.explosionResistance;
                                    current.has_collision = retrieved.has_collision;
                                    // copyTo.isRandomlyTicking = copyFrom.isRandomlyTicking;
                                    // copyTo.lightEmission = copyFrom.lightEmission;
                                    // copyTo.mapColor = copyFrom.mapColor;
                                    // copyTo.soundType = copyFrom.soundType;
                                    // copyTo.friction = copyFrom.friction;
                                    // copyTo.speedFactor = copyFrom.speedFactor;
                                    // copyTo.dynamicShape = copyFrom.dynamicShape;
                                    current.has_collision = retrieved.has_collision;
                                    current.is_air = retrieved.is_air;
                                    // copyTo.ignitedByLava = copyFrom.ignitedByLava;
                                    current.is_liquid = retrieved.is_liquid;
                                    // copyTo.forceSolidOff = copyFrom.forceSolidOff;
                                    // copyTo.forceSolidOn = copyFrom.forceSolidOn;
                                    // copyTo.pushReaction = copyFrom.pushReaction;
                                    // copyTo.requiresCorrectToolForDrops = copyFrom.requiresCorrectToolForDrops;
                                    // copyTo.offsetFunction = copyFrom.offsetFunction;
                                    // copyTo.spawnTerrainParticles = copyFrom.spawnTerrainParticles;
                                    // copyTo.requiredFeatures = copyFrom.requiredFeatures;
                                    // copyTo.emissiveRendering = copyFrom.emissiveRendering;
                                    // copyTo.instrument = copyFrom.instrument;
                                    // copyTo.replaceable = copyFrom.replaceable;
                                }
                                ("ofLegacyCopy", _) => {
                                    let retrieved = retrieved.as_ref().unwrap();
                                    let retrieved = blocks.get(retrieved.name_and_type.name.as_ref()).unwrap();
                                    // copyTo.destroyTime = copyFrom.destroyTime;
                                    // copyTo.explosionResistance = copyFrom.explosionResistance;
                                    current.has_collision = retrieved.has_collision;
                                    // copyTo.isRandomlyTicking = copyFrom.isRandomlyTicking;
                                    // copyTo.lightEmission = copyFrom.lightEmission;
                                    // copyTo.mapColor = copyFrom.mapColor;
                                    // copyTo.soundType = copyFrom.soundType;
                                    // copyTo.friction = copyFrom.friction;
                                    // copyTo.speedFactor = copyFrom.speedFactor;
                                    // copyTo.dynamicShape = copyFrom.dynamicShape;
                                    current.has_collision = retrieved.has_collision;
                                    current.is_air = retrieved.is_air;
                                    // copyTo.ignitedByLava = copyFrom.ignitedByLava;
                                    current.is_liquid = retrieved.is_liquid;
                                    // copyTo.forceSolidOff = copyFrom.forceSolidOff;
                                    // copyTo.forceSolidOn = copyFrom.forceSolidOn;
                                    // copyTo.pushReaction = copyFrom.pushReaction;
                                    // copyTo.requiresCorrectToolForDrops = copyFrom.requiresCorrectToolForDrops;
                                    // copyTo.offsetFunction = copyFrom.offsetFunction;
                                    // copyTo.spawnTerrainParticles = copyFrom.spawnTerrainParticles;
                                    // copyTo.requiredFeatures = copyFrom.requiredFeatures;
                                    // copyTo.emissiveRendering = copyFrom.emissiveRendering;
                                    // copyTo.instrument = copyFrom.instrument;
                                    // copyTo.replaceable = copyFrom.replaceable;
                                }
                                ("offsetType", _) => {}
                                ("pistonProperties", _) => {}
                                ("pushReaction", _) => {}
                                ("randomTicks", _) => {}
                                ("replaceable", _) => {}
                                ("requiresCorrectToolForDrops", _) => {}
                                ("shulkerBoxProperties", _) => {}
                                ("sound", _) => {}
                                ("speedFactor", _) => {}
                                ("strength", _) => {}
                                ("vanillaBlockId", _) => {}
                                ("wallVariant", _) => {}
                                ("waterloggedMapColor", _) => {}
                                // ("", _) => {}

                                // Ignore these
                                ("of", _) => {}

                                (unk, desc) => {
                                    miette::bail!("Unknown \"Blocks\" method: \"{unk}{desc}\"")
                                }
                            }
                        }
                    }
                    Opcode::Invokedynamic(InvokeDynamic { attr_index, .. }) => {
                        let entry = &class.get_bootstrap().unwrap()[usize::from(*attr_index)];
                        for arg in &entry.arguments {
                            if let BootstrapArgument::MethodHandle(handle) = arg
                                && handle.class_name.starts_with("net/minecraft/world/level/block/")
                                && handle.member_ref.name == "<init>"
                            {
                                current.classes.push(handle.class_name.to_string());
                            }
                        }
                    }

                    _ => {}
                }

                Ok(())
            })?;

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

        JarData::get_for(version, storage, async |data| {
            for (block_name, block) in &mut blocks {
                if block.classes.is_empty() {
                    miette::bail!(
                        "Failed to find any classes for block \"{}\" ({block_name})!",
                        block.ident
                    );
                }

                for class_name in &block.classes {
                    let mut class_name = class_name.as_str();
                    let mut class = data.get_class(class_name).unwrap();

                    while block.shape.is_unset() {
                        match class_name {
                            "net/minecraft/world/level/block/Block" => {
                                block.shape = BlockShape::Single([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
                            }
                            "net/minecraft/world/level/block/ShulkerBoxBlock" => {
                                block.shape = BlockShape::Custom(String::from("ShulkerBoxBlock"));
                            }
                            _ => {
                                if let Some(shape_init) = class.get_method_code("getShape") {
                                    block.shape = parse_shape_init_method(shape_init, class, data)?;
                                } else if let Some(parent) = class.super_class.as_ref() {
                                    class_name = parent;
                                    class = data.get_class(class_name).unwrap();
                                } else {
                                    miette::bail!(
                                        "Failed to find block shape for \"{}\" ({block_name})!",
                                        block.ident
                                    );
                                }
                            }
                        }
                    }
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

        let mut count = 0;
        for block in blocks.values() {
            if block.shape.is_unknown() {
                count += 1;
            }
        }
        if count > 0 {
            tracing::warn!(
                "Could not find shapes for {count} (out of {}) blocks for \"{}\".",
                blocks.len(),
                version.as_str()
            );
        }

        Ok(BlockData { blocks, block_families, report })
    }
}

fn parse_shape_init_method(
    init: &CodeData<'static>,
    class: &ClassFile<'static>,
    jar: &JarData,
) -> Result<BlockShape> {
    let bytecode = init.bytecode.as_ref().unwrap();
    let mut shape = BlockShape::Unknown;

    class.iterate_code(bytecode, jar, 0, &mut |_, op| {
        if !shape.is_unknown() {
            return Ok(());
        }

        match op {
            Opcode::Getstatic(MemberRef { class_name, name_and_type })
                if name_and_type.descriptor == "Lnet/minecraft/world/phys/shapes/VoxelShape;" =>
            {
                let class = jar.get_class(class_name).unwrap();
                let init = class.get_static_field_init(&name_and_type.name).unwrap();
                shape = parse_shape_init_static(&init, class, jar)?;
            }

            Opcode::Invokestatic(MemberRef { class_name, name_and_type })
                if class_name == "net/minecraft/world/phys/shapes/Shapes" =>
            {
                match name_and_type.name.as_ref() {
                    "block" => shape = BlockShape::Single([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]),
                    "empty" => shape = BlockShape::Single([0.0, 0.0, 0.0, 0.0, 0.0, 0.0]),
                    unk => miette::bail!("Unknown \"Shapes\" method: {unk}"),
                }
            }

            _ => {}
        }

        Ok(())
    })?;

    Ok(shape)
}

#[expect(clippy::too_many_lines, reason = "Large, multi-argument match statement")]
fn parse_shape_init_static(
    init: &[Opcode<'static>],
    _class: &ClassFile<'static>,
    _jar: &JarData,
) -> Result<BlockShape> {
    let mut constants = Vec::new();
    for op in init {
        match op {
            Opcode::Ldc(Loadable::LiteralConstant(c))
            | Opcode::LdcW(Loadable::LiteralConstant(c))
            | Opcode::Ldc2W(Loadable::LiteralConstant(c)) => {
                constants.push(c);
            }
            Opcode::Dconst0 => {
                constants.push(&LiteralConstant::Double(0.0));
            }
            Opcode::Dconst1 => {
                constants.push(&LiteralConstant::Double(1.0));
            }

            Opcode::Invokestatic(MemberRef { class_name, name_and_type })
                if name_and_type
                    .descriptor
                    .ends_with("Lnet/minecraft/world/phys/shapes/VoxelShape;") =>
            {
                match (
                    class_name.as_ref(),
                    name_and_type.name.as_ref(),
                    name_and_type.descriptor.as_ref(),
                ) {
                    (
                        "net/minecraft/world/level/block/Block",
                        "box",
                        "(DDDDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(min_x),
                            LiteralConstant::Double(min_y),
                            LiteralConstant::Double(min_z),
                            LiteralConstant::Double(max_x),
                            LiteralConstant::Double(max_y),
                            LiteralConstant::Double(max_z),
                        ] = constants.as_slice()
                        {
                            return Ok(BlockShape::Single([
                                min_x / 16.,
                                min_y / 16.,
                                min_z / 16.,
                                max_x / 16.,
                                max_y / 16.,
                                max_z / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"box\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"box\" shape!");
                    }
                    (
                        "net/minecraft/world/level/block/Block",
                        "boxZ",
                        "(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(size_xy),
                            LiteralConstant::Double(min_z),
                            LiteralConstant::Double(max_z),
                        ] = constants.as_slice()
                        {
                            let half_xy = size_xy / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half_xy) / 16.,
                                (8. - half_xy) / 16.,
                                min_z / 16.,
                                (8. + half_xy) / 16.,
                                (8. + half_xy) / 16.,
                                max_z / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"box\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"box\" shape!");
                    }
                    (
                        "net/minecraft/world/level/block/Block",
                        "boxZ",
                        "(DDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(size_x),
                            LiteralConstant::Double(size_y),
                            LiteralConstant::Double(min_z),
                            LiteralConstant::Double(max_z),
                        ] = constants.as_slice()
                        {
                            let half_x = size_x / 2.0;
                            let half_y = size_y / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half_x) / 16.,
                                (8. - half_y) / 16.,
                                min_z / 16.,
                                (8. + half_x) / 16.,
                                (8. + half_y) / 16.,
                                max_z / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"box\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"box\" shape!");
                    }
                    (
                        "net/minecraft/world/level/block/Block",
                        "boxZ",
                        "(DDDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(size_x),
                            LiteralConstant::Double(min_y),
                            LiteralConstant::Double(max_y),
                            LiteralConstant::Double(min_z),
                            LiteralConstant::Double(max_z),
                        ] = constants.as_slice()
                        {
                            let half_x = size_x / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half_x) / 16.,
                                min_y / 16.,
                                min_z / 16.,
                                (8. + half_x) / 16.,
                                max_y / 16.,
                                max_z / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"box\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"box\" shape!");
                    }

                    (
                        "net/minecraft/world/level/block/Block",
                        "column",
                        "(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(size_xz),
                            LiteralConstant::Double(min_y),
                            LiteralConstant::Double(max_y),
                        ] = constants.as_slice()
                        {
                            let half_xz = size_xz / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half_xz) / 16.,
                                min_y / 16.,
                                (8. - half_xz) / 16.,
                                (8. + half_xz) / 16.,
                                max_y / 16.,
                                (8. + half_xz) / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"column\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"column\" shape!");
                    }
                    (
                        "net/minecraft/world/level/block/Block",
                        "column",
                        "(DDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(size_x),
                            LiteralConstant::Double(size_z),
                            LiteralConstant::Double(min_y),
                            LiteralConstant::Double(max_y),
                        ] = constants.as_slice()
                        {
                            let half_x = size_x / 2.0;
                            let half_z = size_z / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half_x) / 16.,
                                min_y / 16.,
                                (8. - half_z) / 16.,
                                (8. + half_x) / 16.,
                                max_y / 16.,
                                (8. + half_z) / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"column\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"column\" shape!");
                    }

                    (
                        "net/minecraft/world/level/block/Block",
                        "cube",
                        "(D)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let Some(LiteralConstant::Double(size)) = constants.last() {
                            let half = size / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half) / 16.,
                                (8. - half) / 16.,
                                (8. - half) / 16.,
                                (8. + half) / 16.,
                                (8. + half) / 16.,
                                (8. + half) / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"cube\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"cube\" shape!");
                    }
                    (
                        "net/minecraft/world/level/block/Block",
                        "cube",
                        "(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;",
                    ) => {
                        if let [
                            ..,
                            LiteralConstant::Double(size_x),
                            LiteralConstant::Double(size_y),
                            LiteralConstant::Double(size_z),
                        ] = constants.as_slice()
                        {
                            let half_x = size_x / 2.0;
                            let half_y = size_y / 2.0;
                            let half_z = size_z / 2.0;
                            return Ok(BlockShape::Single([
                                (8. - half_x) / 16.,
                                (8. - half_y) / 16.,
                                (8. - half_z) / 16.,
                                (8. + half_x) / 16.,
                                (8. + half_y) / 16.,
                                (8. + half_z) / 16.,
                            ]));
                        }
                        tracing::error!("Constants for \"cube\" shape: {constants:#?}");
                        miette::bail!("Failed to find constants for \"cube\" shape!");
                    }

                    _ => miette::bail!(
                        "Unknown method in shape init: {}.{}{}",
                        class_name,
                        name_and_type.name,
                        name_and_type.descriptor
                    ),
                }
            }
            _ => {}
        }
    }

    Ok(BlockShape::Unknown)
}

// -------------------------------------------------------------------------------------------------

pub struct GlobalBlockData {
    pub global_shapes: IndexMap<Version, IndexMap<String, (BlockShape, usize)>>,
}

/// Generate global block data.
#[allow(clippy::too_many_lines, reason = "Necessary")]
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    static ONCE: OnceCell<Result<GlobalBlockData>> = OnceCell::const_new();
    let result = ONCE
        .get_or_init(|| async {
            // Collect the `BlockData` for all versions.
            let global_blocks = VersionHelper::for_all_map(config, async |version| {
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
                let mut attributes = IndexSet::<BlockAttribute>::new();
                for versioned in global_blocks.values() {
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
                let mut blocks = IndexSet::<String>::new();
                for versioned in global_blocks.values() {
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

            // Build the `shape` module with the collected block shapes
            let mut global_shapes =
                IndexMap::<_, IndexMap<_, _>>::with_capacity(global_blocks.len());
            {
                let path = WORKSPACE_DIR.join("froglight-block/src/generated");
                let mut module = ModuleBuilder::new("shape", path);

                // Deduplicate and sort the block shapes
                let mut shapes = IndexSet::<BlockShape>::new();
                for (version, data) in &global_blocks {
                    let global = global_shapes.entry(version.clone()).or_default();

                    for (name, settings) in &data.blocks {
                        shapes.insert(settings.shape.clone());
                        global.insert(
                            name.clone(),
                            (settings.shape.clone(), shapes.get_index_of(&settings.shape).unwrap()),
                        );
                    }
                }

                // Generate the content
                let mut content = String::from("\nuse crate::block::BlockShape;\n");

                content.push_str("\ngenerate! {\n    @shape\n");
                for (index, shape) in shapes.into_iter().enumerate() {
                    writeln!(content, "    SHAPE_{index} => {{ {shape} }}").unwrap();
                }
                content.push('}');

                // Finalize and build the module
                module
                    .with_docs(
                        "Block shapes for all [`Version`](froglight_common::version::Version)s.

@generated",
                    )
                    .with_content(&content);

                module.build().await?;
            }

            Ok(GlobalBlockData { global_shapes })
        })
        .await;

    match result {
        Ok(global) => {
            for version in &config.versions {
                let guard = DATA.owned_guard();
                let storage =
                    DATA.get_or_insert_with(version.real.clone(), RwLock::default, &guard);

                let mut storage = storage.write().await;
                generate(version, global, &mut storage).await?;
            }
            Ok(())
        }
        Err(err) => miette::bail!("Failed to generate global block data: {err}"),
    }
}

/// Generate `Version`-specific block data.
async fn generate(
    version: &VersionPair,
    global: &GlobalBlockData,
    storage: &mut VersionStorage,
) -> Result<()> {
    let global = global.global_shapes.get(&version.real).unwrap();
    BlockData::get_for(&version.real, storage, async |data| {
        let path = WORKSPACE_DIR.join("froglight-block/src/generated");
        let mut module = ModuleBuilder::new_after_marker(path).await?;

        module
            .with_submodule(&version.base.as_feature(), async |module, settings| {
                let mut content = String::new();

                let version_type = version.base.as_feature().to_ascii_uppercase();
                content.push_str("\nuse froglight_common::version::");
                content.push_str(&version_type);
                content.push_str(";\n\n#[allow(clippy::wildcard_imports, reason = \"Generated code\")]\nuse crate::generated::{attribute::*, block::*, shape::*};\n\n");

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
                    content.push_str(", air: ");
                    content.push_str(&settings.is_air.to_string());
                    content.push_str(", solid: ");
                    content.push_str(&settings.is_solid.to_string());
                    content.push_str(", liquid: ");
                    content.push_str(&settings.is_liquid.to_string());
                    content.push_str(", collision: ");
                    content.push_str(&settings.has_collision.to_string());
                    content.push_str(", occlusion: ");
                    content.push_str(&settings.has_occlusion.to_string());

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
                    content.push_str(" ], ");

                    let (_shape, shape_index) = global.get(block).unwrap();
                    content.push_str("shape: { SHAPE_");
                    content.push_str(&shape_index.to_string());
                    content.push_str(" }\n    }");

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
