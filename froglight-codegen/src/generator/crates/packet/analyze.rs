use cafebabe::{bytecode::Opcode, constant_pool::MemberRef};
use miette::Result;

use crate::{
    generator::crates::packet::{PacketData, PacketInfo},
    helper::ClassFileExt,
    source::JarData,
};

impl PacketData {
    pub(super) async fn analyze_packet_protocol(
        packet: &mut PacketInfo,
        jar: &JarData,
    ) -> Result<()> {
        let Some(codec) = packet.packet_codec.as_ref() else {
            return Ok(());
        };

        let class = jar.get_class(&codec.class_name).unwrap();
        let codec_init = class.get_static_field_init(&codec.name_and_type.name).unwrap();

        match codec_init.last().unwrap() {
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
                if name_and_type.name == "codec" => {}
            Opcode::Invokestatic(MemberRef { name_and_type, .. })
                if name_and_type.name == "composite" => {}
            Opcode::Invokestatic(MemberRef { name_and_type, .. })
                if name_and_type.name == "unit" => {}
            Opcode::Invokeinterface(MemberRef { name_and_type, .. }, id)
                if name_and_type.name == "map" => {}

            other => {
                tracing::warn!(
                    "Unexpected opcode in packet codec initializer: {other:?}\n({}.{})",
                    codec.class_name,
                    codec.name_and_type.name
                );
            }
        }

        Ok(())
    }
}
