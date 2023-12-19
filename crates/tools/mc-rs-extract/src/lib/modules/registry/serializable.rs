use classfile::ast::{GetFieldInsn, Insn};
use tracing::{error, info};

use crate::{
    data::ModuleData,
    modules::{registry::registries::RegistriesModule, ExtractModule, ModuleExt},
};

/// A module that generates a list of serializable registries.
///
/// These are registries that are sent to the client from the server.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SerializableRegistriesModule;

impl SerializableRegistriesModule {
    pub const CLASS_PATH: &'static str = "net/minecraft/class_7782";
    pub const CLASS_METHOD: &'static str = "method_45958";
}

impl ModuleExt for SerializableRegistriesModule {
    fn deps(&self) -> &'static [ExtractModule] { &[ExtractModule::Registries] }

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

        // Get the registry field map
        let field_map = if data.output["registry"].has_key("field_map") {
            &data.output["registry"]["field_map"]
        } else {
            error!("Could not get registry field map");
            return;
        };

        info!("Matching field names to registry names...");

        let mut ser_list = Vec::new();
        for insn in code.insns.iter() {
            if let Insn::GetField(GetFieldInsn { class, name, .. }) = insn {
                // Check if the field is a registry
                if class == RegistriesModule::CLASS_PATH {
                    // Get the registry name from the field name
                    if field_map.has_key(name) {
                        ser_list.push(field_map[name].as_str().unwrap().to_string());
                    } else {
                        error!("Could not find field `{name}` in registry field map");
                        continue;
                    }
                }
            }
        }

        info!("Found {} serializable registries!", ser_list.len());
        data.output["registry"]["serializable"] = ser_list.into();
    }
}
