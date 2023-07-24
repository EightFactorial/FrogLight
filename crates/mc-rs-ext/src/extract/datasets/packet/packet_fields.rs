use std::mem;

use classfile::{
    access::ClassAccessFlags,
    ast::{Insn, InvokeInsn},
    classfile::ClassFile,
    method::Method,
};
use itertools::Itertools;
use json::JsonValue;
use log::{error, warn};

use crate::types::{ClassMap, Manifest, Version};

use crate::extract::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PacketFields;

impl PacketFields {
    pub const CLASS: &'static str = "net/minecraft/class_2596";
    pub const METHOD: &'static str = "<init>";

    pub const BUFFER_CLASS: &'static str = "net/minecraft/class_2540";
}

impl Dataset for PacketFields {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let mut packet_vec = Vec::new();
        for class in classmap.values() {
            if class.interfaces.contains(&Self::CLASS.to_string()) {
                packet_vec.push(class);
            }
        }
        packet_vec.sort_by(|a, b| a.this_class.cmp(&b.this_class));

        for packet in packet_vec.into_iter() {
            match packet.access_flags.contains(ClassAccessFlags::ABSTRACT) {
                true => Self::process_abstract(packet, classmap, data),
                false => Self::process_packet(packet, data),
            }
        }
    }
}

impl PacketFields {
    /// Process an abstract packet
    fn process_abstract(packet: &ClassFile, classmap: &ClassMap, data: &mut JsonValue) {
        for class in classmap.values().filter(|&c| {
            c.super_class
                .as_ref()
                .is_some_and(|n| n == &packet.this_class)
        }) {
            let Some(method) = Self::find_initializer(class) else {
                continue;
            };

            let desc = &method.descriptor[1..method.descriptor.len() - 2];
            let Some(fields) = Self::decode_descriptor(desc) else {
                error!("Failed to decode descriptor {}", method.descriptor);
                continue;
            };

            data["packets"]["fields"][class.this_class.clone()] = fields.into();
        }
    }

    /// Find a method that creates the packet
    fn find_initializer(packet: &ClassFile) -> Option<&Method> {
        let found_methods = packet
            .methods
            .iter()
            .filter(|m| m.name == Self::METHOD)
            .collect_vec();

        match found_methods.len() {
            0 => None,
            1 => Some(found_methods.into_iter().next().unwrap()),
            _ => {
                error!("Found multiple methods in class {}", packet.this_class);
                None
            }
        }
    }

    /// Process a packet
    fn process_packet(packet: &ClassFile, data: &mut JsonValue) {
        let Some(method) = Self::find_buffer_reader(packet) else {
            return;
        };

        let fields = Self::find_buffer_read_calls(method);

        data["packets"]["fields"][packet.this_class.clone()] = fields.into();
    }

    /// Find the method that takes a buffer and returns the packet
    fn find_buffer_reader(packet: &ClassFile) -> Option<&Method> {
        let found_methods = packet
            .methods
            .iter()
            .filter(|m| {
                m.name == Self::METHOD && m.descriptor == format!("(L{};)V", Self::BUFFER_CLASS)
            })
            .collect_vec();

        match found_methods.len() {
            0 => None,
            1 => Some(found_methods.into_iter().next().unwrap()),
            n => {
                error!("Found {n} buffer methods in class {}", packet.this_class);
                None
            }
        }
    }

    /// Find calls to methods that read the packet from the buffer
    fn find_buffer_read_calls(method: &Method) -> Vec<String> {
        let mut fields = Vec::new();

        let mut method = method.clone();
        let Some(code) = method.code() else {
            error!("No code found for method {}", method.name);
            return fields;
        };

        for insn in code.insns.iter() {
            if let Insn::Invoke(InvokeInsn {
                class,
                descriptor,
                name,
                ..
            }) = insn
            {
                if class == Self::BUFFER_CLASS && name != Self::METHOD {
                    if let Some(field) = Self::decode_descriptor(descriptor) {
                        fields.extend(field);
                    } else {
                        error!("Unknown buffer read method: {}, {}", name, descriptor);
                    }
                }
            }
        }

        fields
    }

    /// Decode the method descriptor into a list of fields
    fn decode_descriptor(mut desc: &str) -> Option<Vec<String>> {
        let mut fields = Vec::new();

        if desc.starts_with('(') {
            let pos = desc.find(')').unwrap();
            desc = &desc[pos + 1..];
        }

        let mut reading_array = false;

        let mut reading_class = false;
        let mut class = String::new();

        for c in desc.chars() {
            match (reading_class, reading_array, c) {
                (false, _, 'L') => {
                    reading_class = true;
                }
                (true, _, ';') => {
                    reading_class = false;
                    fields.push(mem::take(&mut class));
                }
                (false, false, '[') => {
                    reading_array = true;
                }
                (false, true, _) => {
                    reading_array = false;
                    fields.push(format!("Vec<{c}>"));
                }
                (true, _, c) => {
                    class.push(c);
                }
                (false, _, c) => {
                    fields.push(c.to_string());
                }
            }
        }

        if fields.is_empty() {
            return None;
        }

        Self::rename_class_fields(&mut fields);
        Self::rename_basic_fields(&mut fields);

        Some(fields)
    }

    /// Rename basic fields to their actual types
    fn rename_basic_fields(fields: &mut [String]) {
        for field in fields.iter_mut() {
            if field.len() != 1 {
                continue;
            }

            match field.chars().next().unwrap() {
                'Z' => *field = "bool".to_string(),
                'B' => *field = "u8".to_string(),
                'S' => *field = "u16".to_string(),
                'I' => *field = "u32".to_string(),
                'J' => *field = "u64".to_string(),
                'F' => *field = "f32".to_string(),
                'D' => *field = "f64".to_string(),
                'C' => *field = "char".to_string(),
                _ => {
                    warn!("Unknown field type {}", field)
                }
            }
        }
    }

    /// Rename class fields to their actual types
    fn rename_class_fields(fields: &mut Vec<String>) {
        let mut remove = Vec::new();

        for (index, field) in fields.iter_mut().enumerate() {
            if field.len() == 1 {
                continue;
            }

            // Resolve arrays
            if field.starts_with("Vec<") {
                let pos = field.find('>').unwrap();
                let mut class = [field[4..pos].to_string()];

                Self::rename_basic_fields(&mut class);
                *field = format!("Vec<{}>", class[0]);
                continue;
            }

            // Strip java prefixes
            if field.starts_with("java/lang/")
                || field.starts_with("java/util/")
                || field.starts_with("java/time/")
            {
                *field = field[10..].to_string();
            }

            match field.as_str() {
                "net/minecraft/class_2338" => {
                    *field = "BlockPos".to_string();
                }
                "net/minecraft/class_3965" => {
                    *field = "BlockHitResult".to_string();
                }
                "net/minecraft/class_2487" => {
                    *field = "NbtCompound".to_string();
                }
                "net/minecraft/class_1799" => {
                    *field = "ItemStack".to_string();
                }
                "net/minecraft/class_2960" => {
                    *field = "ResourceLocation".to_string();
                }
                "net/minecraft/class_2561" => {
                    *field = "FormattedText".to_string();
                }
                "net/minecraft/class_5321" => {
                    *field = "RegistryKey".to_string();
                }
                "net/minecraft/class_6880" => {
                    *field = "RegistryEntry".to_string();
                }

                "UUID" => {
                    *field = "Uuid".to_string();
                }
                "com/mojang/authlib/GameProfile" => {
                    *field = "GameProfile".to_string();
                }
                "io/netty/buffer/ByteBuf" => {
                    *field = "UnsizedByteBuffer".to_string();
                }

                "it/unimi/dsi/fastutil/ints/IntList" => {
                    *field = "Vec<u32>".to_string();
                }
                "Instant" => {
                    *field = "u64".to_string();
                }
                "function/IntFunction" | "Iterable" => {
                    remove.push(index);
                }

                "Optional" => {
                    *field = "Option".to_string();
                }
                "Map" => {
                    *field = "HashMap".to_string();
                }
                "List" | "Collection" => {
                    *field = "Vec".to_string();
                }

                _ => {
                    if field.starts_with("net/minecraft") {
                        warn!("Unknown class type {}", field);
                    }
                }
            }
        }

        for index in remove.iter().rev() {
            fields.remove(*index);
        }
    }
}
