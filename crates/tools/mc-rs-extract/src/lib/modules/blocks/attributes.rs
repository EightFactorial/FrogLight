use classfile::{
    access::FieldAccessFlags,
    ast::{Insn, LdcInsn, LdcType, PutFieldInsn},
    attributes::Attribute as ClassFileAttribute,
    classfile::ClassFile,
};
use hashbrown::HashMap;
use itertools::Itertools;
use json::JsonValue;
use strum::Display;
use tracing::{error, info, warn};

use crate::data::ModuleData;

use crate::modules::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockAttributesModule;

impl BlockAttributesModule {
    pub const CLASS_PATH: &'static str = "net/minecraft/class_2741";
    pub const CLASS_METHOD: &'static str = "<clinit>";

    pub const BOOLEAN_CLASS: &'static str = "net/minecraft/class_2746";
    pub const DIRECTION_CLASS: &'static str = "net/minecraft/class_2753";
    pub const ENUM_CLASS: &'static str = "net/minecraft/class_2754";
    pub const INT_CLASS: &'static str = "net/minecraft/class_2758";
}

impl ModuleExt for BlockAttributesModule {
    fn run(&self, data: &mut ModuleData) {
        let classmap = data.classmap.clone();
        let Some(class) = data.classmap.get_mut(Self::CLASS_PATH) else {
            error!("Could not find class {}", Self::CLASS_PATH);
            return;
        };

        // Get a map of field names to their types
        let field_classmap = Self::field_classmap(class);
        // Get a map of enum attribute types to their values
        let enum_attributes = Self::parse_attributes(&field_classmap, classmap);

        info!("Found {} enum attributes!", enum_attributes.len());
        info!("Parsing attributes...");

        let Some(method) = class
            .methods
            .iter_mut()
            .find(|m| m.name == Self::CLASS_METHOD)
        else {
            error!(
                "Could not find method {} in class {}",
                Self::CLASS_METHOD,
                Self::CLASS_PATH
            );
            return;
        };

        let Some(code) = method.code() else {
            error!(
                "Could not find code attribute in method {} in class {}",
                Self::CLASS_METHOD,
                Self::CLASS_PATH
            );
            return;
        };

        let mut attributes: HashMap<String, AttributeType> = HashMap::new();
        let mut attr = AttributeType::default();

        let mut ints = Vec::new();
        for insn in code.insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn {
                    constant: LdcType::Int(i),
                }) => {
                    // Save ints in case they are needed
                    ints.push(*i);
                }
                Insn::PutField(PutFieldInsn { name, .. }) => {
                    if let Some(attr_type) = field_classmap.get(name) {
                        match attr_type.as_str() {
                            Self::BOOLEAN_CLASS => {
                                attr = AttributeType::Boolean;
                            }
                            Self::DIRECTION_CLASS => {
                                attr = AttributeType::Direction {
                                    values: Direction::field_override(name),
                                };
                            }
                            Self::INT_CLASS => {
                                // Manual override for the `rotation` attribute
                                if name.as_str() == "field_12532" {
                                    ints.push(15);
                                }

                                attr = AttributeType::Int {
                                    min: ints[0],
                                    max: ints[1],
                                };
                                ints.clear();
                            }
                            _ => {
                                if let Some(values) = enum_attributes.get(attr_type) {
                                    attr = AttributeType::Enum {
                                        values: values.clone(),
                                    };
                                } else {
                                    warn!("Could not find attribute data for enum {attr_type}");
                                }
                            }
                        }
                        attributes.insert(name.clone(), std::mem::take(&mut attr));
                    } else {
                        warn!("Could not find attribute type for field {name}");
                    }
                }
                _ => {}
            }
        }

        info!("Found {} total attributes!", attributes.len());

        for key in attributes.keys() {
            let name = field_to_attribute_name(key);
            data.output["blocks"]["attributes"]["field_map"][key] = name.into();
        }

        for (key, value) in attributes.into_iter() {
            let mut object = JsonValue::new_object();
            match value {
                AttributeType::Boolean => {
                    object["type"] = "boolean".into();
                }
                AttributeType::Int { min, max } => {
                    object["type"] = "integer".into();
                    object["min"] = min.into();
                    object["max"] = max.into();
                }
                AttributeType::Direction { values } => {
                    object["type"] = "direction".into();
                    object["values"] = values
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .into();
                }
                AttributeType::Enum { values } => {
                    object["type"] = "enum".into();
                    object["values"] = values.into_iter().collect::<Vec<_>>().into();
                }
                AttributeType::Unknown => {}
            }

            let name = field_to_attribute_name(&key);
            data.output["blocks"]["attributes"]["values"][name.to_string()] = object;
        }
    }
}

impl BlockAttributesModule {
    /// Get a map of field names to their types.
    fn field_classmap(class: &ClassFile) -> HashMap<String, String> {
        info!("Parsing field types...");
        let mut fields = HashMap::with_capacity(class.fields.len());

        // Iterate over all class fields with a Minecraft class type
        for field in class
            .fields
            .iter()
            .filter(|field| field.descriptor.starts_with("Lnet/minecraft/class_"))
        {
            // If the field is a class_2754, then it is an enum
            if field.descriptor.contains(Self::ENUM_CLASS) {
                // Get the signature attribute
                if let Some(signature) = field
                    .attributes
                    .iter()
                    .find(|a| matches!(a, ClassFileAttribute::Signature(_)))
                {
                    if let ClassFileAttribute::Signature(sig) = signature {
                        // Get the enum class name
                        let sig = Self::parse_signature(&sig.signature);
                        fields.insert(field.name.clone(), sig.to_string());
                    } else {
                        warn!(
                            "Found non-signature attribute on field {} in class {}",
                            field.name,
                            Self::CLASS_PATH
                        );
                    }
                }
            } else {
                let desc = field
                    .descriptor
                    .trim_start_matches('L')
                    .trim_end_matches(';');

                fields.insert(field.name.clone(), desc.to_string());
            }
        }

        fields
    }

    /// Get the inner enum type from a signature.
    fn parse_signature(signature: &str) -> &str {
        if let Some(sig) = signature.strip_prefix("Lnet/minecraft/class_2754<L") {
            if let Some(sig) = sig.strip_suffix(";>;") {
                return sig;
            }
        }
        signature
    }

    /// Parse the attributes from the class.
    fn parse_attributes(
        field_classmap: &HashMap<String, String>,
        mut classmap: HashMap<String, ClassFile>,
    ) -> HashMap<String, Vec<String>> {
        info!("Parsing enum attributes...");

        // Get the unique types
        let mut unique_types = field_classmap
            .values()
            .cloned()
            .unique()
            .collect::<Vec<_>>();

        // Remove the boolean, direction, and int classes
        unique_types.retain(|t| {
            !matches!(
                t.as_str(),
                Self::BOOLEAN_CLASS | Self::DIRECTION_CLASS | Self::INT_CLASS
            )
        });

        // Create a map of attribute types to their values
        let mut attribute_map = HashMap::with_capacity(unique_types.len());
        for attribute_type in unique_types {
            // Get the class file for the attribute
            let Some(class) = classmap.get_mut(&attribute_type) else {
                error!("Could not find class {attribute_type}");
                continue;
            };

            // Get the enum fields
            let enum_fields = class
                .fields
                .iter()
                .filter_map(|f| {
                    if f.access_flags.contains(FieldAccessFlags::ENUM) {
                        Some(f.name.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // Get the <clinit> method
            let Some(method) = class
                .methods
                .iter_mut()
                .find(|m| m.name == Self::CLASS_METHOD)
            else {
                error!(
                    "Could not find method {} in class {attribute_type}",
                    Self::CLASS_METHOD
                );
                continue;
            };

            // Get the code attribute
            let Some(code) = method.code() else {
                error!(
                    "Could not find code attribute in method {} in class {attribute_type}",
                    Self::CLASS_METHOD
                );
                continue;
            };

            // Get the enum values from the code
            let mut values = Vec::with_capacity(enum_fields.len());
            let mut constant = String::new();
            for insn in code.insns.iter() {
                if let Insn::Ldc(LdcInsn {
                    constant: LdcType::String(s),
                }) = insn
                {
                    if constant.is_empty() {
                        constant = s.clone();
                    }
                } else if let Insn::PutField(PutFieldInsn { name, .. }) = insn {
                    if enum_fields.contains(name) && !constant.is_empty() {
                        values.push(std::mem::take(&mut constant));
                    }
                }
            }

            // Insert the attribute type and values into the map
            attribute_map.insert(attribute_type, values);
        }

        attribute_map
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
enum AttributeType {
    #[default]
    Unknown,
    Boolean,
    Int {
        min: i32,
        max: i32,
    },
    Direction {
        values: Vec<Direction>,
    },
    Enum {
        values: Vec<String>,
    },
}

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[strum(serialize_all = "UPPERCASE")]
enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Direction {
    /// Manual overrides for direction fields.
    fn field_override(field: &str) -> Vec<Direction> {
        match field {
            // Facing
            "field_12525" => vec![
                Self::North,
                Self::East,
                Self::South,
                Self::West,
                Self::Up,
                Self::Down,
            ],
            // Hopper Facing
            "field_12545" => vec![Self::North, Self::East, Self::South, Self::West, Self::Down],
            // Horizontal Facing
            "field_12481" => vec![Self::North, Self::East, Self::South, Self::West],
            // Vertical Direction
            "field_28062" => vec![Self::Up, Self::Down],
            _ => Vec::new(),
        }
    }
}

/// This data is manually copied from the latest mappings.
///
/// This match statement can be generated with the following command:
/// cat Properties.mapping | awk '/field_/ {print "\"" $2 "\" => \"" $3 "\","}'
pub(crate) fn field_to_attribute_name(field: &str) -> &str {
    match field {
        "field_12480" => "FALLING",
        "field_12481" => "HORIZONTAL_FACING",
        "field_12482" => "AGE_5",
        "field_12483" => "BED_PART",
        "field_12484" => "POWERED",
        "field_12485" => "SLAB_TYPE",
        "field_12486" => "CONDITIONAL",
        "field_12487" => "EAST",
        "field_12488" => "EYE",
        "field_12489" => "NORTH",
        "field_12490" => "LEVEL_1_8",
        "field_12491" => "IN_WALL",
        "field_12492" => "PISTON_TYPE",
        "field_12493" => "ATTACHED",
        "field_12494" => "DELAY",
        "field_12495" => "NORTH_WIRE_CONNECTION",
        "field_12496" => "AXIS",
        "field_12497" => "AGE_3",
        "field_12498" => "AGE_15",
        "field_12499" => "INSTRUMENT",
        "field_12500" => "HAS_BOTTLE_1",
        "field_12501" => "INVERTED",
        "field_12502" => "LOCKED",
        "field_12503" => "STAIR_SHAPE",
        "field_12504" => "WEST_WIRE_CONNECTION",
        "field_12505" => "BITES",
        "field_12506" => "CHEST_TYPE",
        "field_12507" => "RAIL_SHAPE",
        "field_12508" => "WATERLOGGED",
        "field_12509" => "EGGS",
        "field_12510" => "MOISTURE",
        "field_12511" => "POWER",
        "field_12512" => "SNOWY",
        "field_12513" => "LEVEL_3",
        "field_12514" => "PERSISTENT",
        "field_12515" => "ENABLED",
        "field_12516" => "BAMBOO_LEAVES",
        "field_12517" => "AGE_25",
        "field_12518" => "BLOCK_HALF",
        "field_12519" => "UP",
        "field_12520" => "DOOR_HINGE",
        "field_12521" => "AGE_1",
        "field_12522" => "TRIGGERED",
        "field_12523" => "EAST_WIRE_CONNECTION",
        "field_12524" => "NOTE",
        "field_12525" => "FACING",
        "field_12526" => "DRAG",
        "field_12527" => "WEST",
        "field_12528" => "OCCUPIED",
        "field_12529" => "HORIZONTAL_AXIS",
        "field_12530" => "HATCH",
        "field_12531" => "HAS_BOTTLE_2",
        "field_12532" => "ROTATION",
        "field_12533" => "DOUBLE_BLOCK_HALF",
        "field_12534" => "COMPARATOR_MODE",
        "field_12535" => "SHORT",
        "field_12536" => "LAYERS",
        "field_12537" => "OPEN",
        "field_12538" => "LEVEL_15",
        "field_12539" => "UNSTABLE",
        "field_12540" => "SOUTH",
        "field_12541" => "DISTANCE_1_7",
        "field_12542" => "STRAIGHT_RAIL_SHAPE",
        "field_12543" => "PICKLES",
        "field_12544" => "HAS_RECORD",
        "field_12545" => "HOPPER_FACING",
        "field_12546" => "DOWN",
        "field_12547" => "STRUCTURE_BLOCK_MODE",
        "field_12548" => "LIT",
        "field_12549" => "STAGE",
        "field_12550" => "AGE_7",
        "field_12551" => "SOUTH_WIRE_CONNECTION",
        "field_12552" => "EXTENDED",
        "field_12553" => "DISARMED",
        "field_12554" => "HAS_BOTTLE_0",
        "field_12555" => "BLOCK_FACE",
        "field_12556" => "AGE_2",
        "field_16503" => "DISTANCE_0_7",
        "field_16561" => "HANGING",
        "field_16562" => "BOTTOM",
        "field_17104" => "ATTACHMENT",
        "field_17393" => "HAS_BOOK",
        "field_17394" => "SIGNAL_FIRE",
        "field_17586" => "LEVEL_8",
        "field_20432" => "HONEY_LEVEL",
        "field_22174" => "EAST_WALL_SHAPE",
        "field_22175" => "NORTH_WALL_SHAPE",
        "field_22176" => "SOUTH_WALL_SHAPE",
        "field_22177" => "WEST_WALL_SHAPE",
        "field_23187" => "CHARGES",
        "field_23333" => "ORIENTATION",
        "field_27220" => "CANDLES",
        "field_28062" => "VERTICAL_DIRECTION",
        "field_28063" => "THICKNESS",
        "field_28120" => "SCULK_SENSOR_PHASE",
        "field_28716" => "BERRIES",
        "field_28717" => "TILT",
        "field_31387" => "LEVEL_3_MIN",
        "field_31388" => "LEVEL_1_8_MIN",
        "field_31389" => "LEVEL_3_MAX",
        "field_31390" => "LEVEL_1_8_MAX",
        "field_31391" => "DISTANCE_0_7_MAX",
        "field_31392" => "CHARGES_MIN",
        "field_31393" => "CHARGES_MAX",
        "field_31395" => "AGE_1_MAX",
        "field_31396" => "AGE_2_MAX",
        "field_31397" => "AGE_3_MAX",
        "field_31398" => "AGE_5_MAX",
        "field_31399" => "AGE_7_MAX",
        "field_31400" => "AGE_15_MAX",
        "field_31401" => "AGE_25_MAX",
        "field_31402" => "DISTANCE_1_7_MAX",
        "field_33723" => "LEVEL_15_MAX",
        "field_37651" => "BLOOM",
        "field_37652" => "SHRIEKING",
        "field_37653" => "AGE_4_MAX",
        "field_37654" => "AGE_4",
        "field_38423" => "CAN_SUMMON",
        "field_41317" => "SLOT_0_OCCUPIED",
        "field_41318" => "SLOT_1_OCCUPIED",
        "field_41319" => "SLOT_2_OCCUPIED",
        "field_41320" => "SLOT_3_OCCUPIED",
        "field_41321" => "SLOT_4_OCCUPIED",
        "field_41322" => "SLOT_5_OCCUPIED",
        "field_42835" => "FLOWER_AMOUNT",
        "field_42836" => "DUSTED",
        "field_43307" => "CRACKED",
        "field_46822" => "CRAFTING",
        "field_47408" => "TRIAL_SPAWNER_STATE",
        _ => {
            warn!("Could not find a name for field {field}");
            field
        }
    }
}
