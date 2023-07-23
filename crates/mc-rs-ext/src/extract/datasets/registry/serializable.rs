use classfile::ast::{GetFieldInsn, Insn};
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

use super::Registry;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerializableRegistry;

impl Dataset for SerializableRegistry {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[Datasets::Registry(Registry)] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(class) = classmap.get("net/minecraft/class_7782") else {
            error!("Failed to find SerializableRegistry class");
            return;
        };

        let Some(method) = class.methods.iter().find(|m| m.name == "method_45958") else {
            error!("Failed to find SerializableRegistry.method_45958");
            return;
        };

        let mut method = method.clone();
        let Some(code) = method.code() else {
            error!("Failed to find SerializableRegistry.method_45958 code");
            return;
        };

        // Add serializable registry list
        {
            data["registry"]["serializable"] = JsonValue::new_array();
            for insn in code.insns.iter() {
                if let Insn::GetField(GetFieldInsn { class, name, .. }) = insn {
                    if class == "net/minecraft/class_7924" && data["registry"]["map"].has_key(name)
                    {
                        let registry_name =
                            data["registry"]["map"][name].as_str().unwrap().to_owned();
                        data["registry"]["serializable"]
                            .push(registry_name)
                            .unwrap();
                    }
                }
            }
        }
    }
}
