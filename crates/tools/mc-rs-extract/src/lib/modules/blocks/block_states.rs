use classfile::{
    ast::{GetFieldInsn, Insn, InvokeInsn, LdcInsn, LdcType, NewObjectInsn, PutFieldInsn},
    classfile::ClassFile,
};
use hashbrown::HashMap;
use json::JsonValue;
use tracing::{error, info, warn};

use crate::{data::ModuleData, modules::BlockAttributesModule};

use crate::modules::{ExtractModule, ModuleExt};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockStatesModule;

impl BlockStatesModule {
    pub const CLASS_PATH: &'static str = "net/minecraft/class_2246";
    pub const CLASS_METHOD: &'static str = "<clinit>";

    pub const BLOCK_CLASS: &'static str = "net/minecraft/class_2248";
    const SETTINGS_CLASS: &'static str = "net/minecraft/class_4970$class_2251";
    const UNIFORM_INT_CLASS: &'static str = "net/minecraft/class_6019";
    const BLOCKSTATE_CLASS: &'static str = "net/minecraft/class_2680";
    const MAPCOLOR_CLASS: &'static str = "net/minecraft/class_3620";
}

impl ModuleExt for BlockStatesModule {
    fn deps(&self) -> &'static [ExtractModule] {
        &[ExtractModule::BlockList, ExtractModule::BlockAttributes]
    }

    fn run(&self, data: &mut ModuleData) {
        let Some(class) = data.classmap.get_mut(Self::CLASS_PATH) else {
            error!("Could not find class {}", Self::CLASS_PATH);
            return;
        };

        let Some(method) = class
            .methods
            .iter_mut()
            .find(|method| method.name == Self::CLASS_METHOD)
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
                "Could not get code for method {} in class {}",
                Self::CLASS_METHOD,
                Self::CLASS_PATH
            );
            return;
        };

        let mut block_list = Vec::default();
        let mut insn_list = Vec::new();

        // Group insns for easier parsing
        for insn in code.insns.iter() {
            insn_list.push(insn.clone());

            if let Insn::PutField(field) = insn {
                if field.class == Self::CLASS_PATH
                    && field.descriptor == format!("L{};", Self::BLOCK_CLASS)
                {
                    let block = BlockInsns(std::mem::take(&mut insn_list));
                    block_list.push(block);
                }
            }
        }

        info!("Parsing block classes...");
        for block in block_list {
            let mut block = block.parse(&mut data.classmap, &data.output);
            block.attributes.sort();
            block.attributes.dedup();

            if block.name.is_empty() {
                warn!("Block name is empty, skipping block");
                continue;
            }

            data.output["blocks"]["data"][block.name]["attributes"] = block.attributes.into();
        }

        info!("Calculating block state ids...");
        let attribute_data = data.output["blocks"]["attributes"].clone();

        let mut state_id: u32 = 0;
        for (_, block_data) in data.output["blocks"]["data"].entries_mut() {
            let mut state_vals = Vec::new();

            for field in block_data["attributes"].members() {
                let field = field.as_str().unwrap();

                let values: u32 = match attribute_data["values"][field]["type"].as_str().unwrap() {
                    "boolean" => 2,
                    "integer" => {
                        let min = attribute_data["values"][field]["min"].as_i32().unwrap();
                        let max = attribute_data["values"][field]["max"].as_i32().unwrap();
                        i32::abs_diff(min, max) + 1
                    }
                    "direction" | "enum" => {
                        attribute_data["values"][field]["values"].members().count() as u32
                    }
                    unk => {
                        error!("Unknown attribute type `{unk}`");
                        1
                    }
                };

                state_vals.push(values);
            }

            let min = state_id;
            let max = min + state_vals.into_iter().product::<u32>().saturating_sub(1);

            block_data["state_ids"]["min"] = min.into();
            block_data["state_ids"]["max"] = max.into();
            state_id = max + 1;
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct BlockInsns(Vec<Insn>);

#[derive(Debug, Default, Clone, PartialEq)]
struct BlockAttrs {
    name: String,
    attributes: Vec<String>,
}

impl BlockInsns {
    fn parse(self, classmap: &mut HashMap<String, ClassFile>, output: &JsonValue) -> BlockAttrs {
        let mut block_attrs = BlockAttrs::default();

        for (index, insn) in self.0.into_iter().enumerate() {
            if index == 0 {
                if let Insn::Ldc(LdcInsn {
                    constant: LdcType::String(constant),
                }) = insn
                {
                    block_attrs.name = constant;
                } else {
                    error!("Could not get name for block");
                }

                continue;
            }

            if let Insn::NewObject(NewObjectInsn { kind }) = insn {
                // Directly get the attributes from the block class
                if let Some(attrs) = Self::parse_class(&kind, classmap, output) {
                    if !attrs.is_empty() {
                        block_attrs.attributes = attrs;
                        return block_attrs;
                    }
                }
            } else if let Insn::Invoke(InvokeInsn { descriptor, .. }) = insn {
                // Look at the return type of the method and get the block attributes from there
                let descriptor = descriptor.split(')').last().unwrap();
                let descriptor = descriptor.trim_start_matches('L').trim_end_matches(';');

                if descriptor.starts_with("net/minecraft")
                    && !matches!(
                        descriptor,
                        BlockStatesModule::BLOCK_CLASS
                            | BlockStatesModule::BLOCKSTATE_CLASS
                            | BlockStatesModule::SETTINGS_CLASS
                            | BlockStatesModule::UNIFORM_INT_CLASS
                            | BlockStatesModule::MAPCOLOR_CLASS
                    )
                {
                    // If the class has already been parsed, use the cached attributes
                    if let Some(attrs) = Self::parse_class(descriptor, classmap, output) {
                        if !attrs.is_empty() {
                            block_attrs.attributes = attrs;
                            return block_attrs;
                        }
                    }
                }
            }
        }

        block_attrs
    }

    /// Parse a class for block attributes
    fn parse_class(
        class: &str,
        classmap: &mut HashMap<String, ClassFile>,
        output: &JsonValue,
    ) -> Option<Vec<String>> {
        let Some(mut class) = classmap.get(class).cloned() else {
            error!("Could not find class `{class}`");
            return None;
        };

        // Get the attributes from the superclass
        let mut attrs = Vec::new();
        if let Some(superclass) = class.super_class {
            if superclass.as_str() != "java/lang/Object" {
                if let Some(super_attrs) = Self::parse_class(&superclass, classmap, output) {
                    attrs.extend(super_attrs);
                }
            }
        }

        // Get the attributes from the interfaces
        for extended in class.interfaces {
            if let Some(super_attrs) = Self::parse_class(&extended, classmap, output) {
                attrs.extend(super_attrs);
            }
        }

        // Get a list of all fields that are an attribute class
        let mut fields = Vec::new();
        for field in class.fields.iter() {
            if matches!(
                field
                    .descriptor
                    .as_str()
                    .trim_start_matches('L')
                    .trim_end_matches(';'),
                BlockAttributesModule::BOOLEAN_CLASS
                    | BlockAttributesModule::INT_CLASS
                    | BlockAttributesModule::DIRECTION_CLASS
                    | BlockAttributesModule::ENUM_CLASS
            ) {
                fields.push(field.name.clone());
            }
        }

        let Some(method) = class
            .methods
            .iter_mut()
            .find(|method| method.name == BlockStatesModule::CLASS_METHOD)
        else {
            return Some(attrs);
        };

        let Some(code) = method.code() else {
            error!(
                "Could not get code for method {} in class `{}`",
                BlockStatesModule::CLASS_METHOD,
                class.this_class
            );
            return Some(attrs);
        };

        // Get the attributes from the class
        for insns in code.insns.insns.windows(2) {
            if let [Insn::GetField(GetFieldInsn {
                class: get_class,
                name: get_name,
                ..
            }), Insn::PutField(PutFieldInsn { name: put_name, .. })] = insns
            {
                if fields.contains(put_name) {
                    // If the attribute is already in the output, use that
                    if output["blocks"]["attributes"]["field_map"].has_key(get_name) {
                        let attr = output["blocks"]["attributes"]["field_map"][get_name]
                            .as_str()
                            .unwrap();

                        attrs.push(attr.to_string());
                    } else if let Some(attr) =
                        Self::get_field(get_name, get_class, classmap, output)
                    {
                        // Otherwise, it's likely pulled from another class,
                        // so get the attribute from there instead.
                        attrs.push(attr);
                    } else {
                        warn!(
                            "Could not find attribute for field `{get_name}` in class `{get_class}`",
                        );
                    }
                }
            }
        }

        attrs.retain(|attr| !attr.starts_with("net/minecraft/class_"));

        Some(attrs)
    }

    /// Get the attribute for a field, recursively if needed.
    fn get_field(
        field: &str,
        class: &str,
        classmap: &mut HashMap<String, ClassFile>,
        output: &JsonValue,
    ) -> Option<String> {
        // Override `field_10927` in `net/minecraft/class_2671`
        if class == "net/minecraft/class_2671" && field == "field_10927" {
            return Some("net/minecraft/class_2680".to_owned());
        }

        let Some(class) = classmap.get_mut(class) else {
            error!("Could not find class `{class}`");
            return None;
        };

        let Some(method) = class
            .methods
            .iter_mut()
            .find(|method| method.name == BlockStatesModule::CLASS_METHOD)
        else {
            // Silently return, some classes don't have a <clinit> method
            return None;
        };

        let Some(code) = method.code().cloned() else {
            error!(
                "Could not get code for method {} in class `{}`",
                BlockStatesModule::CLASS_METHOD,
                class.this_class
            );
            return None;
        };

        // Look for the GetField and PutField insns that correspond to the field we're looking for
        for insns in code.insns.insns.windows(2) {
            if let [Insn::GetField(GetFieldInsn {
                class: get_class,
                name: get_name,
                ..
            }), Insn::PutField(PutFieldInsn { name: put_name, .. })] = insns
            {
                if put_name != field {
                    continue;
                }

                // If the attribute is already in the output, use that
                if output["blocks"]["attributes"]["field_map"].has_key(get_name) {
                    let attr = output["blocks"]["attributes"]["field_map"][get_name]
                        .as_str()
                        .unwrap();

                    return Some(attr.to_string());
                } else if let Some(attr) = Self::get_field(get_name, get_class, classmap, output) {
                    // Otherwise, it's likely pulled from another class,
                    // so get the attribute from there instead.
                    return Some(attr);
                } else {
                    warn!("Could not recursively find attribute for field `{get_name}` in class `{get_class}`",);
                    return None;
                }
            }
        }

        None
    }
}
