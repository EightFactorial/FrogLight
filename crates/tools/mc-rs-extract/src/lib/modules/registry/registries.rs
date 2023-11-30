use std::mem;

use classfile::ast::{Insn, LdcInsn, LdcType, PutFieldInsn};
use json::JsonValue;
use tracing::{error, info};

use crate::{data::ModuleData, modules::ModuleExt};

/// A module that generates a list of registries.
///
/// These are registries Minecraft uses internally for various things.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RegistriesModule;

impl RegistriesModule {
    pub const CLASS_PATH: &'static str = "net/minecraft/class_7924";
    pub const CLASS_METHOD: &'static str = "<clinit>";
}

impl ModuleExt for RegistriesModule {
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

        info!("Parsing registry list...");

        let mut reg_list = Vec::new();
        let mut reg = Registry::default();

        for insn in code.insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn {
                    constant: LdcType::String(constant),
                }) => {
                    reg.constant = constant.clone();
                }
                Insn::PutField(PutFieldInsn { class, name, .. }) => {
                    if class == Self::CLASS_PATH {
                        reg.name = name.clone();
                        if reg.constant.is_empty() {
                            error!("Found registry with empty constant: {reg:?}");
                        }

                        // Add registry to list and replace it with the default
                        reg_list.push(mem::take(&mut reg));
                    }
                }
                _ => {}
            }
        }

        info!("Found {} registries!", reg_list.len());

        // Add registry map of `field: constant`
        {
            let mut object = JsonValue::new_object();
            for reg in &reg_list {
                object[reg.name.clone()] = reg.constant.clone().into();
            }
            data.output["registry"]["field_map"] = object;
        }

        // Add sorted list of registry constants
        {
            let mut list = Vec::new();
            for reg in reg_list {
                list.push(reg.constant);
            }
            list.dedup();
            list.sort();
            data.output["registry"]["list"] = list.into();
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Registry {
    name: String,
    constant: String,
}
