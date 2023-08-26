use classfile::{
    ast::{GetFieldInsn, Insn, InvokeInsn, LdcInsn, LdcType, NewObjectInsn, PutFieldInsn},
    classfile::ClassFile,
};
use hashbrown::HashMap;
use itertools::Itertools;
use json::JsonValue;
use log::error;

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
                        error!("Could not get name for blockstate with kind {}", kind);
                        continue;
                    };

                    if data["blocks"]["blocks"]["blocks"][name].is_object() {
                        block_classes.insert(name, kind.clone());
                    } else {
                        error!("Could not find block {}", name);
                    }
                }
                Insn::Invoke(InvokeInsn { name: _name, .. }) => {
                    // TODO: Map method names to block classes
                    // Needed for blocks like candles, flowerpots, buttons, etc.
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
            let info = &class_info[&class];

            if !info.states.is_empty() {
                data["blocks"]["blocks"]["blocks"][block]["states"] = info.states.clone().into();
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
            if field.name == "field_12196" {
                // TODO: Figure out why this field isn't being resolved
                states.push("FACING".to_owned());
            } else if !PropertyType::TYPES.contains(&field.descriptor.as_str()) {
                // Skip fields that aren't state properties
                continue;
            } else if let Some(class) = get_field_class(&field.name, class, classmap, data) {
                states.push(class);
            } else {
                error!("Could not find class for field {}", field.name);
            }
        }

        ClassInfo { states }
    } else {
        error!("Could not find class {}", class);
        ClassInfo { states: Vec::new() }
    }
}

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
                // Recurse through abstract block classes and get where that field points to.
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
