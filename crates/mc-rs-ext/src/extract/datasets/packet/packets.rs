use std::{collections::BTreeMap, mem};

use classfile::ast::{GetFieldInsn, Insn, LdcInsn, LdcType, PutFieldInsn};
use itertools::Itertools;
use json::JsonValue;
use log::error;

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Packets;

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
        let Some(class) = classmap.get("net/minecraft/class_2539") else {
            error!("Failed to find Packets class");
            return;
        };

        let Some(method) = class.methods.iter().find(|m| m.name == "<clinit>") else {
            error!("Failed to find Packets.<clinit>");
            return;
        };

        let mut method = method.clone();
        let Some(code) = method.code() else {
            error!("Failed to find Packets.<clinit> code");
            return;
        };

        let mut packet_id = 0;
        let mut direction = false;

        let mut state = State::default();
        let mut states = Vec::new();
        let mut prev_insn: Option<Insn> = None;

        for insn in code.insns.iter() {
            match insn {
                Insn::Ldc(LdcInsn { constant }) => {
                    if let Some(Insn::Ldc(LdcInsn { .. })) = prev_insn {
                        if let LdcType::Int(int) = constant {
                            state.id = *int;
                        }
                    } else if let LdcType::Class(class) = constant {
                        match direction {
                            true => state.clientbound.insert(packet_id, class.clone()),
                            false => state.serverbound.insert(packet_id, class.clone()),
                        };
                        packet_id += 1;
                    } else if let LdcType::String(string) = constant {
                        state.name = string.clone();
                    }
                }
                Insn::GetField(GetFieldInsn { name, .. }) => match name.as_str() {
                    "field_11942" => {
                        direction = true;
                        packet_id = 0;
                    }
                    "field_11941" => {
                        direction = false;
                        packet_id = 0;
                    }
                    _ => {}
                },
                Insn::PutField(PutFieldInsn {
                    class, descriptor, ..
                }) => {
                    if class.as_str() == "net/minecraft/class_2539"
                        && descriptor.as_str() == "Lnet/minecraft/class_2539;"
                    {
                        states.push(mem::take(&mut state));
                    }
                }
                _ => {}
            }
            prev_insn = Some(insn.clone());
        }

        // Add packets
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

        // Add packet states
        data["packets"]["states"]["names"] = states
            .iter()
            .map(|state| state.name.clone())
            .collect_vec()
            .into();

        let mut state_data = JsonValue::new_object();
        for s in states {
            let mut client_list = JsonValue::new_object();
            for (id, class) in s.clientbound {
                client_list[class] = id.into();
            }

            let mut server_list = JsonValue::new_object();
            for (id, class) in s.serverbound {
                server_list[class] = id.into();
            }

            state_data[s.name] = json::object! {
                id: s.id,
                clientbound: client_list,
                serverbound: server_list,
            }
        }

        data["packets"]["states"]["data"] = state_data;
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    pub name: String,
    pub id: i32,
    pub serverbound: BTreeMap<u32, String>,
    pub clientbound: BTreeMap<u32, String>,
}
