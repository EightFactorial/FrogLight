use std::{collections::BTreeMap, mem};

use classfile::ast::{GetFieldInsn, Insn, LdcInsn, LdcType, PutFieldInsn};
use itertools::Itertools;
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packets;

impl Packets {
    pub const CLASS: &'static str = "net/minecraft/class_2539";
    pub const METHOD: &'static str = "<clinit>";
}

impl Dataset for Packets {
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

        let mut packet_id = 0;
        let mut direction = false;

        let mut state = State::default();
        let mut states = Vec::new();
        let mut prev_insn: Option<Insn> = None;

        for insn in insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn { constant }) => {
                    if let LdcType::Int(int) = constant {
                        if matches!(prev_insn, Some(Insn::Ldc(LdcInsn { .. }))) {
                            state.id = *int;
                        }
                    } else if let LdcType::String(string) = constant {
                        state.name = string.clone();
                    } else if let LdcType::Class(class) = constant {
                        match direction {
                            true => state.clientbound.insert(packet_id, class.clone()),
                            false => state.serverbound.insert(packet_id, class.clone()),
                        };
                        packet_id += 1;
                    }
                }
                Insn::GetField(GetFieldInsn { name, .. }) => match name.as_str() {
                    "field_11941" => {
                        direction = false;
                        packet_id = 0;
                    }
                    "field_11942" => {
                        direction = true;
                        packet_id = 0;
                    }
                    _ => {}
                },
                Insn::PutField(PutFieldInsn {
                    class, descriptor, ..
                }) => {
                    if class.as_str() == Self::CLASS
                        && descriptor.as_str().contains(Self::CLASS)
                        && !state.name.is_empty()
                    {
                        states.push(mem::take(&mut state));
                    }
                }
                _ => {}
            }
            prev_insn = Some(insn.clone());
        }

        // Replace `HANDSHAKING` with `HANDSHAKE`
        {
            if let Some(state) = states.iter_mut().find(|s| s.name == "HANDSHAKING") {
                state.name = "HANDSHAKE".to_string();
            }
        }

        // Add packets
        {
            let mut classes = Vec::new();
            for state in states.iter() {
                for class in state.clientbound.values() {
                    classes.push(class.clone());
                }
                for class in state.serverbound.values() {
                    classes.push(class.clone());
                }
            }

            data["packets"]["classes"] = classes.into();
        }

        // Add packet state names
        {
            data["packets"]["states"]["names"] = states
                .iter()
                .map(|state| state.name.clone())
                .collect_vec()
                .into();
        }

        // Add packet state data
        {
            for s in states {
                let mut client_list = JsonValue::new_object();
                for (id, class) in s.clientbound {
                    client_list[class] = id.into();
                }

                let mut server_list = JsonValue::new_object();
                for (id, class) in s.serverbound {
                    server_list[class] = id.into();
                }

                data["packets"]["states"]["data"][s.name] = json::object! {
                    id: s.id,
                    clientbound: client_list,
                    serverbound: server_list,
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    pub name: String,
    pub id: i32,
    pub serverbound: BTreeMap<u32, String>,
    pub clientbound: BTreeMap<u32, String>,
}
