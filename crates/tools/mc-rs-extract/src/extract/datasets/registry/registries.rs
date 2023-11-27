use std::mem;

use classfile::ast::{Insn, LdcInsn, LdcType, PutFieldInsn};
use itertools::Itertools;
use json::JsonValue;
use tracing::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Registry;

impl Registry {
    pub const CLASS: &'static str = "net/minecraft/class_7924";
    pub const METHOD: &'static str = "<clinit>";
}

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
        let Some(insns) = Datasets::get_code(Self::METHOD, Self::CLASS, classmap) else {
            error!(
                "Could not get code for method {} in class {}",
                Self::METHOD,
                Self::CLASS
            );
            return;
        };

        let mut vec = Vec::new();
        let mut constant = String::new();

        for insn in insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn {
                    constant: LdcType::String(s),
                }) => {
                    constant = s.clone();
                }
                Insn::PutField(PutFieldInsn { class, name, .. }) => {
                    if !constant.is_empty() && class == Self::CLASS {
                        vec.push((mem::take(&mut constant), name.clone()));
                    }
                }
                _ => {}
            }
        }

        // Add registry map
        {
            for (constant, name) in vec.clone().into_iter().sorted_by(|(_, a), (_, b)| a.cmp(b)) {
                data["registry"]["fields"][name] = constant.into();
            }
        }

        // Add registry list
        {
            data["registry"]["list"] = vec
                .into_iter()
                .map(|(constant, _)| constant)
                .sorted()
                .collect_vec()
                .into();
        }
    }
}
