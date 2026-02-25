use cafebabe::{
    ClassFile,
    attributes::CodeData,
    bytecode::Opcode,
    constant_pool::{LiteralConstant, Loadable, MemberRef},
};
use indexmap::IndexMap;
use miette::Result;

use super::{PacketData, PacketInfo, PacketStateData};
use crate::{helper::ClassFileExt, source::JarData};

impl PacketData {
    pub(super) async fn identify_packet_classes(
        class: &ClassFile<'static>,
        jar: &JarData,
    ) -> Result<PacketStateData> {
        let mut data =
            PacketStateData { clientbound: IndexMap::new(), serverbound: IndexMap::new() };

        let mut clientbound = None;
        let mut serverbound = None;

        for field in &class.fields {
            // Look for `SimpleUnboundProtocol` or `UnboundProtocol`
            if !field.descriptor.field_type.to_string().contains("UnboundProtocol") {
                continue;
            }
            if field.name.contains("CLIENTBOUND") {
                clientbound = Some(field);
            } else if field.name.contains("SERVERBOUND") {
                serverbound = Some(field);
            }
        }

        let code = class.get_static_code().unwrap();

        if let Some(clientbound) = clientbound {
            Self::identify_protocol_packets(
                class,
                &clientbound.name,
                code,
                &mut data.clientbound,
                jar,
            )
            .await?;
        }
        if let Some(serverbound) = serverbound {
            Self::identify_protocol_packets(
                class,
                &serverbound.name,
                code,
                &mut data.serverbound,
                jar,
            )
            .await?;
        }

        Ok(data)
    }

    async fn identify_protocol_packets(
        class: &ClassFile<'static>,
        template: &str,
        code: &CodeData<'static>,
        storage: &mut IndexMap<String, PacketInfo>,
        jar: &JarData,
    ) -> Result<()> {
        let references = identify_protocol_types_and_codecs(class, template, code, jar)?;

        for (packet_type, packet_codec) in references {
            let identifier = get_packet_type_identifier(&packet_type, jar)?;

            let mut info = PacketInfo {
                packet_ident: identifier.clone(),
                packet_type,
                packet_codec,
                read_ops: Vec::new(),
                read_hash: 0,
                write_ops: Vec::new(),
                write_hash: 0,
            };

            Self::analyze_packet_protocol(&mut info, jar).await?;
            storage.insert(identifier, info);
        }

        Ok(())
    }
}

/// Collect all of the `PacketType` and `StreamCodec` references used while
/// initializing the given protocol template.
fn identify_protocol_types_and_codecs(
    class: &ClassFile<'static>,
    template: &str,
    code: &CodeData<'static>,
    jar: &JarData,
) -> Result<Vec<(MemberRef<'static>, Option<MemberRef<'static>>)>> {
    let mut references = Vec::new();
    let mut references_complete = false;

    let mut packet_type = None;
    let bytecode = code.bytecode.as_ref().unwrap();
    class.iterate_code(bytecode, jar, 0, &mut |_, op| match op {
        Opcode::Getstatic(reference) if !references_complete && reference.name_and_type.descriptor == "Lnet/minecraft/network/protocol/PacketType;" => {
            // Handle the `Bundle` packet types, which has no codec
            if reference.name_and_type.name == "CLIENTBOUND_BUNDLE" {
                references.push((reference.clone(), None));
                return Ok(());
            }

            if packet_type.is_none() {
                packet_type = Some(reference.clone());
                Ok(())
            } else {
                Err(miette::miette!("Found multiple PacketTypes before a template!\n{packet_type:#?}\n->\n{reference:#?}"))
            }
        }
        Opcode::Getstatic(reference) if !references_complete && reference.name_and_type.descriptor == "Lnet/minecraft/network/codec/StreamCodec;" => {
            references.push((packet_type.take().unwrap(), Some(reference.clone())));
            Ok(())
        }

        Opcode::Putstatic(MemberRef { class_name, name_and_type })
            if class_name == &*class.this_class =>
        {
            if name_and_type.name == template {
                references_complete = true;
            } else if !references_complete {
                references.clear();
            }

            Ok(())
        }
        _ => Ok(()),
    })?;

    Ok(references)
}

/// Get the "minecraft:..." identifier for the given packet type.
fn get_packet_type_identifier(packet_type: &MemberRef<'static>, jar: &JarData) -> Result<String> {
    let Some(code) = jar.get_class_method_code(&packet_type.class_name, "<clinit>", None) else {
        miette::bail!("Failed to find <clinit> method for class \"{}\"", packet_type.class_name);
    };

    let mut found = false;
    let mut identifier = None;

    for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
        match op {
            Opcode::Ldc(Loadable::LiteralConstant(LiteralConstant::String(constant)))
            | Opcode::LdcW(Loadable::LiteralConstant(LiteralConstant::String(constant)))
            | Opcode::Ldc2W(Loadable::LiteralConstant(LiteralConstant::String(constant))) => {
                identifier = Some(constant.clone());
            }
            Opcode::Putstatic(MemberRef { name_and_type, .. })
                if name_and_type.name == packet_type.name_and_type.name =>
            {
                found = true;
                break;
            }
            _ => {}
        }
    }

    if found {
        Ok(format!("minecraft:{}", identifier.unwrap()))
    } else {
        Err(miette::miette!(
            "Failed to find reference to packet type \"{}.{}\"",
            packet_type.class_name,
            packet_type.name_and_type.name
        ))
    }
}
