use classfile::{
    ast::{GetFieldInsn, Insn, InvokeInsn, LdcInsn, LdcType, NewObjectInsn, PutFieldInsn},
    classfile::ClassFile,
};
use hashbrown::HashMap;
use itertools::Itertools;
use json::JsonValue;
use log::{error, warn};

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

use super::{states::PropertyType, Blocks, States};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockStates;

impl BlockStates {
    pub const CLASS: &'static str = "net/minecraft/class_2246";
    pub const METHOD: &'static str = "<clinit>";
}

impl Dataset for BlockStates {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[Datasets::Blocks(Blocks), Datasets::States(States)] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(insns) = Datasets::get_code(Self::METHOD, Self::CLASS, classmap) else {
            error!(
                "Could not get code for method {} in class {}",
                Self::METHOD,
                Self::CLASS
            );
            return;
        };

        // Get the NewObjectInsn class for each block class
        let mut block_classes = HashMap::with_capacity(1024);
        for (index, insn) in insns.iter().enumerate() {
            match insn {
                Insn::NewObject(NewObjectInsn { kind }) => {
                    // Skip sapling generator classes
                    if matches!(
                        kind.as_str(),
                        "net/minecraft/class_2662"
                            | "net/minecraft/class_2659"
                            | "net/minecraft/class_2652"
                            | "net/minecraft/class_2655"
                            | "net/minecraft/class_2654"
                            | "net/minecraft/class_8175"
                            | "net/minecraft/class_2657"
                    ) {
                        continue;
                    }

                    let Some(Insn::Ldc(LdcInsn {
                        constant: LdcType::String(name),
                    })) = insns.get(index - 1)
                    else {
                        error!("Could not get name for blockstate with class {}", kind);
                        continue;
                    };

                    if data["blocks"]["blocks"]["blocks"][name].is_object() {
                        block_classes.insert(name, kind.clone());
                    } else {
                        error!("Could not find block {}", name);
                    }
                }
                Insn::Invoke(InvokeInsn {
                    class: class_name,
                    name,
                    ..
                }) => {
                    if *class_name != Self::CLASS
                        || matches!(
                            name.as_str(),
                            // register()
                            "method_9492"
                            // createLightLevelFromLitBlockState()
                            | "method_26107"
                        )
                    {
                        continue;
                    }

                    // The number of instructions to go back to get the block name
                    let insn_index = match name.as_str() {
                        // createLogBlock()
                        "method_26117" => 3,
                        // createLeavesBlock()
                        "method_26106" => 2,
                        // createBedBlock()
                        "method_26109" => 2,
                        // createShulkerBoxBlock()
                        "method_26110" => 5,
                        // createBambooBlock()
                        "method_47375" => 4,
                        // createNetherStemBlock()
                        "method_26115" => 2,
                        // createPistonBlock()
                        "method_26119" => 2,
                        // createStainedBlassBlock()
                        "method_26120" => 2,
                        // createWoodenButtonBlock()
                        "method_45451" => 4,
                        // createStoneButtonBlock()
                        "method_45453" => 1,
                        // createFlowerPotBlock()
                        "method_50000" => 4,
                        // createCandleBlock()
                        "method_50001" => 2,
                        _ => {
                            warn!("Unhandled invoke for method {}", name);
                            2
                        }
                    };

                    let Some(Insn::Ldc(LdcInsn {
                        constant: LdcType::String(block_name),
                    })) = insns.get(index - insn_index)
                    else {
                        error!("Could not get name for block with field {name}");
                        continue;
                    };

                    let Some(class_name) = find_new_object(class_name, name, classmap) else {
                        error!("Could not find class for block {}", block_name);
                        continue;
                    };

                    if data["blocks"]["blocks"]["blocks"][block_name].is_object() {
                        block_classes.insert(block_name, class_name);
                    } else {
                        error!("Could not find block {}", block_name);
                    }
                }
                _ => {}
            }
        }

        // Get the states for each block class
        let classes = block_classes.values().unique().collect_vec();
        let mut class_info = HashMap::with_capacity(block_classes.len());
        for class in classes {
            class_info.insert(class, get_class_info(class, classmap, data));
        }

        // Add the states to the block data
        for (&block, class) in block_classes.iter() {
            let info = class_info.get_mut(&class).unwrap();

            // TODO: Get these automatically
            match block.as_str() {
                "chiseled_bookshelf" => info.states.extend(
                    [
                        "HORIZONTAL_FACING",
                        "SLOT_0_OCCUPIED",
                        "SLOT_1_OCCUPIED",
                        "SLOT_2_OCCUPIED",
                        "SLOT_3_OCCUPIED",
                        "SLOT_4_OCCUPIED",
                        "SLOT_5_OCCUPIED",
                    ]
                    .map(|s| s.to_owned()),
                ),
                "glow_lichen" => info.states.extend(
                    [
                        "DOWN",
                        "UP",
                        "NORTH",
                        "SOUTH",
                        "WEST",
                        "EAST",
                        "WATERLOGGED",
                    ]
                    .map(|s| s.to_owned()),
                ),
                "brewing_stand" => info
                    .states
                    .extend(["HAS_BOTTLE_0", "HAS_BOTTLE_1", "HAS_BOTTLE_2"].map(|s| s.to_owned())),
                "torchflower_crop" => info.states.retain(|s| s != "AGE_7"),
                "pitcher_crop" => info.states.clear(),
                "beetroots" => info.states.retain(|s| s != "AGE_7"),
                _ => {}
            }

            if !info.states.is_empty() {
                data["blocks"]["blocks"]["blocks"][block]["states"] = info
                    .states
                    .iter()
                    .unique()
                    .cloned()
                    .collect::<Vec<String>>()
                    .into();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ClassInfo {
    states: Vec<String>,
}

// Get the class info for a block class
fn get_class_info(class: &str, classmap: &ClassMap, data: &JsonValue) -> ClassInfo {
    if let Some(class) = classmap.get(class) {
        let mut states = Vec::with_capacity(class.fields.len());

        for field in class.fields.iter() {
            if !PropertyType::TYPES.contains(&field.descriptor.as_str()) {
                // Skip fields that aren't state properties
                continue;
            } else if field.name == "field_12196" {
                // TODO: Figure out why this field isn't being resolved
                states.push("FACING".to_owned());
            } else if let Some(class) = get_field_class(&field.name, class, classmap, data) {
                states.push(class);
            } else {
                error!("Could not find class for field {}", field.name);
            }
        }

        // Get the states for the super class
        if let Some(class) = &class.super_class {
            if !class.contains("java/lang/Object") {
                let info = get_class_info(class, classmap, data);
                states.extend(info.states);
            }
        }

        ClassInfo { states }
    } else {
        error!("Could not find class {}", class);
        ClassInfo { states: Vec::new() }
    }
}

/// Returns the state property name for the given field
fn get_field_class(
    field: &str,
    class: &ClassFile,
    classmap: &ClassMap,
    data: &JsonValue,
) -> Option<String> {
    let Some(method) = class.methods.iter().find(|&m| m.name == "<clinit>") else {
        return None;
    };
    let mut method = method.clone();

    let Some(code) = method.code() else {
        error!("Could not get code for <clinit>");
        return None;
    };
    let insns = &code.insns.insns;

    // Get the index of the PutFieldInsn for the desired field
    if let Some((index, _)) = insns.iter().enumerate().find(|(_, insn)| {
        if let Insn::PutField(PutFieldInsn { name, .. }) = insn {
            name == field
        } else {
            false
        }
    }) {
        // Get the GetFieldInsn for the field
        if let Some(Insn::GetField(GetFieldInsn { name, class, .. })) = insns.get(index - 1) {
            // If we know the the field, get the state name found during the States dataset
            if data["blocks"]["states"]["fields"].has_key(name.as_str()) {
                return data["blocks"]["states"]["fields"][name]
                    .as_str()
                    .map(|s| s.to_owned());
            } else if let Some(class) = classmap.get(class) {
                // If we don't know of the field, it's likely pointing to an abstract block class.
                // Recurse through the abstract block field and get where that field points to.
                get_field_class(name, class, classmap, data)
            } else {
                error!("Could not find state property {name} in class {class}");
                None
            }
        } else {
            error!("Could not get field for {field}");
            None
        }
    } else {
        error!("Could not find a PutFieldInsn for {field}");
        None
    }
}

/// Returns the class name of the first NewObjectInsn for the given method
fn find_new_object(class: &str, method: &str, classmap: &ClassMap) -> Option<String> {
    let insns = Datasets::get_code(method, class, classmap)?;

    // Get the first NewObjectInsn and return the class name
    if let Some(insn) = insns.iter().find(|insn| matches!(insn, Insn::NewObject(_))) {
        if let Insn::NewObject(NewObjectInsn { kind }) = insn {
            Some(kind.clone())
        } else {
            unreachable!("NewObjectInsn was not a NewObjectInsn?")
        }
    } else {
        error!("Could not find a NewObjectInsn for {}", method);
        None
    }
}
