use std::{collections::BTreeMap, mem};

use classfile::ast::{Insn, LdcInsn, LdcType, PutFieldInsn};
use itertools::Itertools;
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SoundEvents;

impl SoundEvents {
    pub const CLASS: &'static str = "net/minecraft/class_3417";
    pub const METHOD: &'static str = "<clinit>";
}

impl Dataset for SoundEvents {
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
            error!("Failed to find SoundEvents class");
            return;
        };

        let Some(method) = class.methods.iter().find(|m| m.name == Self::METHOD) else {
            error!("Failed to find SoundEvents.{}", Self::METHOD);
            return;
        };

        let mut method = method.clone();
        let Some(code) = method.code() else {
            error!("Failed to find SoundEvents.{} code", Self::METHOD);
            return;
        };

        let mut event_name = String::new();
        let mut hash = BTreeMap::new();

        for insn in code.insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn {
                    constant: LdcType::String(s),
                }) => {
                    event_name = s.clone();
                }
                Insn::PutField(PutFieldInsn {
                    class,
                    name,
                    descriptor,
                    ..
                }) => {
                    if class == Self::CLASS && descriptor == "Lnet/minecraft/class_3414;" {
                        hash.insert(name.clone(), mem::take(&mut event_name));
                    }
                }
                _ => {}
            }
        }

        // Add event map
        {
            for (field, event) in hash.clone().into_iter() {
                data["sound"]["events"]["map"][field] = event.into();
            }
        }

        // Add event list
        {
            let mut list = hash.into_values().collect_vec();
            list.sort();
            data["sound"]["events"]["list"] = list.into();
        }
    }
}
