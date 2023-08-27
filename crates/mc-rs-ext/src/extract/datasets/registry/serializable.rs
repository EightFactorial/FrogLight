use classfile::ast::{GetFieldInsn, Insn};
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

use super::Registry;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SerializableRegistry;

impl SerializableRegistry {
    pub const CLASS: &'static str = "net/minecraft/class_7782";
    pub const METHOD: &'static str = "method_45958";
}

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
        let Some(insns) = Datasets::get_code(Self::METHOD, Self::CLASS, classmap) else {
            error!(
                "Could not get code for method {} in class {}",
                Self::METHOD,
                Self::CLASS
            );
            return;
        };

        data["registry"]["serializable"] = JsonValue::new_array();

        for insn in insns.iter() {
            if let Insn::GetField(GetFieldInsn { class, name, .. }) = insn {
                if class == Registry::CLASS && data["registry"]["map"].has_key(name) {
                    // Get the registry name
                    let registry_name = data["registry"]["map"][name].as_str().unwrap().to_owned();

                    // Add it to the list
                    data["registry"]["serializable"]
                        .push(registry_name)
                        .unwrap();
                }
            }
        }
    }
}
