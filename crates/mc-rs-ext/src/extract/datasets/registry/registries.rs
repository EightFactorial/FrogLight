use std::mem;

use classfile::ast::{Insn, LdcInsn, LdcType, PutFieldInsn};
use itertools::Itertools;
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Registry;

impl Dataset for Registry {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(class) = classmap.get("net/minecraft/class_7924") else {
            error!("Failed to find Registry class");
            return;
        };

        let Some(method) = class.methods.iter().find(|m| m.name == "<clinit>") else {
            error!("Failed to find Registry.<clinit>");
            return;
        };

        let mut method = method.clone();
        let Some(code) = method.code() else {
            error!("Failed to find Registry.<clinit> code");
            return;
        };

        let mut vec = Vec::new();
        let mut constant = String::new();

        for insn in code.insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn {
                    constant: LdcType::String(s),
                }) => {
                    constant = s.clone();
                }
                Insn::PutField(PutFieldInsn { class, name, .. }) => {
                    if !constant.is_empty() && class == "net/minecraft/class_7924" {
                        vec.push((mem::take(&mut constant), name.clone()));
                    }
                }
                _ => {}
            }
        }
        vec.sort();

        // Add registry map
        {
            for (constant, name) in vec.clone() {
                data["registry"]["map"][name] = constant.into();
            }
        }

        // Add registry list
        {
            data["registry"]["list"] = vec
                .into_iter()
                .map(|(constant, _)| constant)
                .collect_vec()
                .into();
        }
    }
}
