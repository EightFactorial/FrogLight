use std::collections::BTreeMap;

use classfile::ast::{Insn, LdcInsn, LdcType, PutFieldInsn};
use itertools::Itertools;
use json::JsonValue;
use log::{error, warn};
use strum::Display;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct States;

impl States {
    pub const CLASS: &'static str = "net/minecraft/class_2741";
    pub const METHOD: &'static str = "<clinit>";
}

impl Dataset for States {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(class) = classmap.get(Self::CLASS) else {
            error!("Could not find class {}", Self::CLASS);
            return;
        };

        let Some(method) = class.methods.iter().find(|&m| m.name == Self::METHOD) else {
            error!("Could not find method {}", Self::METHOD);
            return;
        };
        let mut method = method.clone();

        let Some(code) = method.code() else {
            error!("Could not get code for method {}", Self::METHOD);
            return;
        };
        let insns = &code.insns.insns;

        let mut properties = BTreeMap::new();
        let mut constant = String::new();
        let mut class = Option::<String>::default();

        for insn in insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn {
                    constant: LdcType::String(string),
                }) => {
                    constant = string.clone();
                }
                Insn::Ldc(LdcInsn {
                    constant: LdcType::Class(kind),
                }) => {
                    class = Some(kind.clone());
                }
                Insn::PutField(PutFieldInsn {
                    name, descriptor, ..
                }) => {
                    let key = Property::field_name(name).to_string();
                    let kind = PropertyType::from_descriptor(descriptor);

                    properties.insert(
                        key,
                        Property {
                            kind,
                            constant: std::mem::take(&mut constant),
                            class: std::mem::take(&mut class),
                        },
                    );
                }
                _ => {}
            }
        }

        data["blocks"]["states"]["list"] = properties.keys().cloned().collect_vec().into();

        for (key, prop) in properties {
            let obj = match prop.kind {
                PropertyType::Enum => {
                    json::object! {
                        "type": prop.kind.to_string(),
                        "constant": prop.constant,
                        "class": prop.class.unwrap(),
                    }
                }
                _ => {
                    json::object! {
                        "type": prop.kind.to_string(),
                        "constant": prop.constant,
                    }
                }
            };

            data["blocks"]["states"]["states"][key] = obj;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
#[strum(serialize_all = "lowercase")]
enum PropertyType {
    Boolean,
    Direction,
    Enum,
    Integer,
    Unknown(String),
}

impl PropertyType {
    fn from_descriptor(descriptor: &str) -> Self {
        match descriptor
            .trim_start_matches("Lnet/minecraft/")
            .trim_end_matches(';')
        {
            "class_2746" => Self::Boolean,
            "class_2753" => Self::Direction,
            "class_2754" => Self::Enum,
            "class_2758" => Self::Integer,
            unk => {
                warn!("Unknown property type: {}", unk);
                Self::Unknown(unk.to_owned())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Property {
    kind: PropertyType,
    constant: String,
    class: Option<String>,
}

impl Property {
    //  grep FIELD Properties.mapping | awk '{print "\"" $2 "\" => \"" $3 "\","}'
    fn field_name(field: &str) -> &str {
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
            "field_12555" => "WALL_MOUNT_LOCATION",
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
            _ => {
                warn!("Unknown block state: {}", field);
                field
            }
        }
    }
}

impl Default for PropertyType {
    fn default() -> Self { Self::Unknown(String::new()) }
}
