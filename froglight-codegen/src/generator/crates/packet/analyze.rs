#![allow(clippy::unnecessary_wraps, reason = "May return an error in the future")]

use core::hash::Hasher;
use std::hash::DefaultHasher;

use cafebabe::{
    ClassFile,
    attributes::CodeData,
    bytecode::{ByteCode, Opcode},
    constant_pool::{BootstrapArgument, InvokeDynamic, MemberRef, NameAndType},
};
use miette::Result;

use crate::{
    generator::crates::packet::{PacketData, PacketInfo},
    helper::ClassFileExt,
    source::JarData,
};

impl PacketData {
    #[expect(clippy::unused_async, reason = "Yes")]
    pub(super) async fn analyze_packet_protocol(
        packet: &mut PacketInfo,
        jar: &JarData,
    ) -> Result<()> {
        let Some(codec) = packet.packet_codec.clone() else {
            return Ok(());
        };

        let class = jar.get_class(&codec.class_name).unwrap();
        let init = class.get_static_field_init(&codec.name_and_type.name).unwrap();
        Self::analyze_codec_init(
            packet,
            class,
            &codec,
            init.as_slice(),
            init.last().unwrap(),
            jar,
        )?;

        Ok(())
    }

    fn analyze_codec_init(
        packet: &mut PacketInfo,
        class: &ClassFile<'static>,
        codec: &MemberRef<'static>,
        init: &[Opcode<'static>],
        invoke: &Opcode<'static>,
        jar: &JarData,
    ) -> Result<()> {
        match invoke {
            other @ (Opcode::Invokeinterface(MemberRef { class_name, .. }, _)
            | Opcode::Invokestatic(MemberRef { class_name, .. }))
                if !matches!(
                    class_name.as_ref(),
                    "net/minecraft/network/codec/StreamCodec"
                        | "net/minecraft/network/protocol/Packet"
                ) =>
            {
                tracing::warn!(
                    "Unexpected packet codec initializer: {other:?}\n({}.{})",
                    codec.class_name,
                    codec.name_and_type.name
                );
            }

            Opcode::Invokestatic(MemberRef { name_and_type, .. })
                if name_and_type.name == "codec" =>
            {
                let [encode, decode, _] = init else {
                    miette::bail!(
                        "Unexpected number of opcodes in packet codec initializer: {}\n({}.{})",
                        init.len(),
                        codec.class_name,
                        codec.name_and_type.name
                    );
                };

                if let (Opcode::Invokedynamic(encode), Opcode::Invokedynamic(decode)) =
                    (encode, decode)
                {
                    Self::analyze_codec_dynamic_encode(packet, class, encode, jar)?;
                    Self::analyze_codec_dynamic_decode(packet, class, decode, jar)?;
                } else {
                    miette::bail!(
                        "Unexpected opcodes as \"codec\" codec arguments: {encode:?}, {decode:?}\n({}.{})",
                        codec.class_name,
                        codec.name_and_type.name
                    );
                }
            }
            Opcode::Invokestatic(MemberRef { name_and_type, .. })
                if name_and_type.name == "composite" => {}
            Opcode::Invokestatic(MemberRef { name_and_type, .. })
                if name_and_type.name == "unit" => {}
            Opcode::Invokeinterface(MemberRef { name_and_type, .. }, id)
                if name_and_type.name == "map" => {}

            other => {
                tracing::warn!(
                    "Unexpected opcode as codec initializer: {other:?}\n({}.{})",
                    codec.class_name,
                    codec.name_and_type.name
                );
            }
        }

        Ok(())
    }

    fn analyze_codec_dynamic_encode(
        packet: &mut PacketInfo,
        class: &ClassFile<'static>,
        encode: &InvokeDynamic<'static>,
        jar: &JarData,
    ) -> Result<()> {
        let bootstrap = class.get_bootstrap().unwrap();
        let entry = &bootstrap[encode.attr_index as usize];

        for arg in &entry.arguments {
            if let BootstrapArgument::MethodHandle(method) = arg {
                tracing::trace!("\n\nEncode \"{}.{}\"", method.class_name, method.member_ref.name);

                if let Some(code) = jar.get_class_method_code(
                    &method.class_name,
                    &method.member_ref.name,
                    Some(&method.member_ref.descriptor),
                ) {
                    Self::analyze_encode_method(
                        packet,
                        jar.get_class(&method.class_name).unwrap(),
                        code.bytecode.as_ref().unwrap(),
                        jar,
                    )?;
                } else {
                    miette::bail!(
                        "Could not find method for codec encode: {}.{}{}",
                        method.class_name,
                        method.member_ref.name,
                        method.member_ref.descriptor
                    );
                }
            }
        }
        Ok(())
    }

    fn analyze_codec_dynamic_decode(
        packet: &mut PacketInfo,
        class: &ClassFile<'static>,
        decode: &InvokeDynamic<'static>,
        jar: &JarData,
    ) -> Result<()> {
        let bootstrap = class.get_bootstrap().unwrap();
        let entry = &bootstrap[decode.attr_index as usize];

        for arg in &entry.arguments {
            if let BootstrapArgument::MethodHandle(method) = arg {
                tracing::trace!("\n\nDecode \"{}.{}\"", method.class_name, method.member_ref.name);

                if let Some(code) = jar.get_class_method_code(
                    &method.class_name,
                    &method.member_ref.name,
                    Some(&method.member_ref.descriptor),
                ) {
                    Self::analyze_decode_method(
                        packet,
                        jar.get_class(&method.class_name).unwrap(),
                        code.bytecode.as_ref().unwrap(),
                        jar,
                    )?;
                } else {
                    miette::bail!(
                        "Could not find method for codec decode: {}.{}{}",
                        method.class_name,
                        method.member_ref.name,
                        method.member_ref.descriptor
                    );
                }
            }
        }

        Ok(())
    }

    fn analyze_encode_method(
        packet: &mut PacketInfo,
        class: &ClassFile<'static>,
        bytecode: &ByteCode<'static>,
        jar: &JarData,
    ) -> Result<()> {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(packet.write_hash);

        if let Err(err) = class.iterate_code(bytecode, jar, 0, &mut |_, op| {
            match op {
                Opcode::Invokeinterface(invoke, _)
                | Opcode::Invokespecial(invoke)
                | Opcode::Invokestatic(invoke)
                | Opcode::Invokevirtual(invoke) => {
                    hasher.write(invoke.name_and_type.name.as_bytes());
                    if invoke.class_name.ends_with("ByteBuf") {
                        tracing::trace!(
                            "ByteBuf => Encode: {}.{}{}",
                            invoke.class_name,
                            invoke.name_and_type.name,
                            invoke.name_and_type.descriptor
                        );
                    }
                }
                _ => {}
            }
            Ok(())
        }) {
            tracing::error!("Error analyzing packet \"{}\": {err}", packet.packet_ident);
        }

        hasher.write_u64(packet.write_hash);
        packet.write_hash = hasher.finish();
        Ok(())
    }

    fn analyze_decode_method(
        packet: &mut PacketInfo,
        class: &ClassFile<'static>,
        bytecode: &ByteCode<'static>,
        jar: &JarData,
    ) -> Result<()> {
        let mut hasher = DefaultHasher::new();
        hasher.write_u64(packet.read_hash);

        if let Err(err) = class.iterate_code(bytecode, jar, 0, &mut |_, op| {
            match op {
                Opcode::Invokeinterface(invoke, _)
                | Opcode::Invokespecial(invoke)
                | Opcode::Invokestatic(invoke)
                | Opcode::Invokevirtual(invoke) => {
                    hasher.write(invoke.name_and_type.name.as_bytes());
                    if invoke.class_name.contains("ByteBuf") {
                        tracing::trace!(
                            "ByteBuf => Decode: {}.{}{}",
                            invoke.class_name,
                            invoke.name_and_type.name,
                            invoke.name_and_type.descriptor
                        );
                    }
                }
                _ => {}
            }
            Ok(())
        }) {
            tracing::error!("Error analyzing packet \"{}\": {err}", packet.packet_ident);
        }

        hasher.write_u64(packet.read_hash);
        packet.read_hash = hasher.finish();
        Ok(())
    }
}

/// Find a method and it's code in a class,
/// recursively searching any superclasses.
fn find_method_or_super<'a>(
    class: &'a ClassFile<'static>,
    method: &NameAndType<'static>,
    jar: &'a JarData,
) -> Option<(&'a ClassFile<'static>, &'a CodeData<'static>)> {
    if let Some(code) = class.get_method_code(&method.name) {
        return Some((class, code));
    }

    if let Some(super_class) = class.super_class.as_ref()
        && super_class.starts_with("net/minecraft")
        && let Some(super_class) = jar.get_class(super_class)
    {
        return find_method_or_super(super_class, method, jar);
    }

    None
}
