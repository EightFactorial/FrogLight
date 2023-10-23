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
        let Some(insns) = Datasets::get_code(Self::METHOD, Self::CLASS, classmap) else {
            error!(
                "Could not get code for method {} in class {}",
                Self::METHOD,
                Self::CLASS
            );
            return;
        };

        let mut event_name = String::new();
        let mut hash = BTreeMap::new();

        for insn in insns.iter() {
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
