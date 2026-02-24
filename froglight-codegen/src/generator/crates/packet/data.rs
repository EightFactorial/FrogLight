use std::hash::{DefaultHasher, Hasher};

use cafebabe::{
    ClassFile,
    attributes::CodeData,
    bytecode::Opcode,
    constant_pool::{BootstrapArgument, LiteralConstant, Loadable, MemberRef, NameAndType},
};
use convert_case::{Case, Casing};
use indexmap::IndexMap;
use miette::Result;

use crate::{
    common::{Version, VersionStorage},
    helper::ClassFileExt,
    source::JarData,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PacketData {
    pub states: IndexMap<String, PacketStateData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PacketStateData {
    pub clientbound: IndexMap<String, PacketInfo>,
    pub serverbound: IndexMap<String, PacketInfo>,
}

#[derive(Debug, Clone)]
pub struct PacketInfo {
    pub packet_type: MemberRef<'static>,
    pub packet_codec: Option<MemberRef<'static>>,

    pub read_ops: Vec<TypeOperation>,
    pub read_hash: u64,

    pub write_ops: Vec<TypeOperation>,
    pub write_hash: u64,
}

impl PartialEq for PacketInfo {
    fn eq(&self, other: &Self) -> bool {
        self.read_hash == other.read_hash && self.write_hash == other.write_hash
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeOperation {
    Bool,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    String,
    VarShort,
    VarInt,
    VarLong,

    Profile,
    Tag,
    CompoundTag,
    ContainerId,
    RotationByte,
    OptionalVarInt,
    ByteArray,
}

// -------------------------------------------------------------------------------------------------

impl PacketData {
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `PacketData` for \"{}\"", version.as_str());
            let data = Self::fetch(version, &mut *storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`JarData`] for the given [`Version`].
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        JarData::get_for(version, storage, async |jar| {
            let mut states = IndexMap::new();

            for (name, class) in [
                ("handshake", "net/minecraft/network/protocol/handshake/HandshakeProtocols"),
                ("status", "net/minecraft/network/protocol/status/StatusProtocols"),
                ("login", "net/minecraft/network/protocol/login/LoginProtocols"),
                ("config", "net/minecraft/network/protocol/configuration/ConfigurationProtocols"),
                ("play", "net/minecraft/network/protocol/game/GameProtocols"),
            ] {
                if let Some(class) = jar.get_class(class) {
                    let data = Self::analyze_protocol_class(class, jar).await?;
                    states.insert(name.to_case(Case::Pascal), data);
                } else {
                    miette::bail!(
                        "Failed to find class \"{class}\" in version \"{}\"",
                        version.as_str()
                    );
                }
            }

            Ok(Self { states })
        })
        .await
    }

    async fn analyze_protocol_class(
        class: &ClassFile<'static>,
        _jar: &JarData,
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
            Self::analyze_protocol_template(
                class,
                &clientbound.name,
                code,
                &mut data.clientbound,
                _jar,
            )
            .await?;
        }
        if let Some(serverbound) = serverbound {
            Self::analyze_protocol_template(
                class,
                &serverbound.name,
                code,
                &mut data.serverbound,
                _jar,
            )
            .await?;
        }

        Ok(data)
    }

    async fn analyze_protocol_template(
        class: &ClassFile<'static>,
        template: &str,
        code: &CodeData<'static>,
        storage: &mut IndexMap<String, PacketInfo>,
        jar: &JarData,
    ) -> Result<()> {
        let references = collect_types_and_codecs(class, template, code, jar)?;

        for (packet_type, packet_codec) in references {
            let identifier = get_packet_identifier(&packet_type, jar)?;

            let (read_ops, read_hash, write_ops, write_hash) = if let Some(codec) = &packet_codec {
                Self::analyze_protocol_codec(codec, jar).await?
            } else {
                (Vec::new(), 0, Vec::new(), 0)
            };

            storage.insert(
                identifier,
                PacketInfo {
                    packet_type,
                    packet_codec,
                    read_ops,
                    read_hash,
                    write_ops,
                    write_hash,
                },
            );
        }

        Ok(())
    }

    async fn analyze_protocol_codec(
        codec: &MemberRef<'static>,
        jar: &JarData,
    ) -> Result<(Vec<TypeOperation>, u64, Vec<TypeOperation>, u64)> {
        let (mut read_ops, mut write_ops) = (Vec::new(), Vec::new());
        let (mut read_hasher, mut write_hasher) = (DefaultHasher::new(), DefaultHasher::new());

        let Some(class) = jar.get_class(&codec.class_name) else {
            miette::bail!(
                "Failed to find class for codec \"{}.{}\"",
                codec.class_name,
                codec.name_and_type.name
            );
        };
        let Some(init_ops) = class.get_static_field_init(&codec.name_and_type.name) else {
            miette::bail!(
                "Failed to find initialization of codec \"{}.{}\"",
                codec.class_name,
                codec.name_and_type.name
            );
        };

        parse_type_operations(
            class,
            init_ops.into_iter(),
            &mut read_ops,
            &mut read_hasher,
            &mut write_ops,
            &mut write_hasher,
            jar,
        )
        .await?;

        Ok((read_ops, read_hasher.finish(), write_ops, write_hasher.finish()))
    }
}

/// Collect all of the `PacketType` and `StreamCodec` references used while
/// initializing the given protocol template.
fn collect_types_and_codecs(
    class: &ClassFile<'static>,
    template: &str,
    code: &CodeData<'static>,
    jar: &JarData,
) -> Result<Vec<(MemberRef<'static>, Option<MemberRef<'static>>)>> {
    let mut references = Vec::new();
    let mut references_complete = false;

    let mut packet_type = None;
    let bytecode = code.bytecode.as_ref().unwrap();
    class.iterate_code(bytecode, jar, &mut |_, op| match op {
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
fn get_packet_identifier(packet_type: &MemberRef<'static>, jar: &JarData) -> Result<String> {
    let Some(code) = jar.get_class_method_code(&packet_type.class_name, "<clinit>") else {
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

// -------------------------------------------------------------------------------------------------

/// Parse the type operations used to initialize a protocol codec,
/// hashing pieces data for easy comparison.
async fn parse_type_operations<T: Iterator<Item = Opcode<'static>>>(
    class: &ClassFile<'static>,
    opcodes: T,
    read_ops: &mut Vec<TypeOperation>,
    read_hasher: &mut DefaultHasher,
    write_ops: &mut Vec<TypeOperation>,
    write_hasher: &mut DefaultHasher,
    jar: &JarData,
) -> Result<()> {
    for op in opcodes {
        match op {
            Opcode::Getstatic(member)
                if member.class_name == "net/minecraft/network/codec/ByteBufCodecs"
                    && member.name_and_type.descriptor
                        == "Lnet/minecraft/network/codec/StreamCodec;" =>
            {
                match member.name_and_type.name.as_ref() {
                    "BOOL" => {
                        read_ops.push(TypeOperation::Bool);
                        write_ops.push(TypeOperation::Bool);
                    }
                    "BYTE" => {
                        read_ops.push(TypeOperation::Byte);
                        write_ops.push(TypeOperation::Byte);
                    }
                    "SHORT" => {
                        read_ops.push(TypeOperation::Short);
                        write_ops.push(TypeOperation::Short);
                    }
                    "INT" => {
                        read_ops.push(TypeOperation::Int);
                        write_ops.push(TypeOperation::Int);
                    }
                    "LONG" => {
                        read_ops.push(TypeOperation::Long);
                        write_ops.push(TypeOperation::Long);
                    }
                    "FLOAT" => {
                        read_ops.push(TypeOperation::Float);
                        write_ops.push(TypeOperation::Float);
                    }
                    "DOUBLE" => {
                        read_ops.push(TypeOperation::Double);
                        write_ops.push(TypeOperation::Double);
                    }
                    "STRING" | "STRING_UTF8" => {
                        read_ops.push(TypeOperation::String);
                        write_ops.push(TypeOperation::String);
                    }
                    "VAR_SHORT" => {
                        read_ops.push(TypeOperation::VarShort);
                        write_ops.push(TypeOperation::VarShort);
                    }
                    "VAR_INT" => {
                        read_ops.push(TypeOperation::VarInt);
                        write_ops.push(TypeOperation::VarInt);
                    }
                    "VAR_LONG" => {
                        read_ops.push(TypeOperation::VarLong);
                        write_ops.push(TypeOperation::VarLong);
                    }
                    "GAME_PROFILE" => {
                        read_ops.push(TypeOperation::Profile);
                        write_ops.push(TypeOperation::Profile);
                    }
                    "TAG" => {
                        read_ops.push(TypeOperation::Tag);
                        write_ops.push(TypeOperation::Tag);
                    }
                    "TRUSTED_COMPOUND_TAG" => {
                        read_ops.push(TypeOperation::CompoundTag);
                        write_ops.push(TypeOperation::CompoundTag);
                    }
                    "CONTAINER_ID" => {
                        read_ops.push(TypeOperation::ContainerId);
                        write_ops.push(TypeOperation::ContainerId);
                    }
                    "ROTATION_BYTE" => {
                        read_ops.push(TypeOperation::RotationByte);
                        write_ops.push(TypeOperation::RotationByte);
                    }
                    "OPTIONAL_VAR_INT" => {
                        read_ops.push(TypeOperation::OptionalVarInt);
                        write_ops.push(TypeOperation::OptionalVarInt);
                    }
                    "BYTE_ARRAY" => {
                        read_ops.push(TypeOperation::ByteArray);
                        write_ops.push(TypeOperation::ByteArray);
                    }
                    _ => tracing::error!("UNKNOWN BYTE_BUF CODEC: {member:?}"),
                }
            }
            Opcode::Getstatic(member)
                if member.name_and_type.descriptor
                    == "Lnet/minecraft/network/codec/StreamCodec;" =>
            {
                let result = Box::pin(PacketData::analyze_protocol_codec(&member, jar)).await?;
                read_ops.extend(result.0);
                read_hasher.write_u64(result.1);
                write_ops.extend(result.2);
                write_hasher.write_u64(result.3);
            }

            Opcode::Getstatic(member) => {
                tracing::debug!("UNKNOWN STATIC: {member:?}");
            }

            Opcode::Invokespecial(method)
            | Opcode::Invokestatic(method)
            | Opcode::Invokevirtual(method)
                if !should_skip_class(&method.class_name) =>
            {
                parse_method_type_operations(
                    jar.get_class(&method.class_name).unwrap(),
                    &method.name_and_type,
                    read_ops,
                    read_hasher,
                    write_ops,
                    write_hasher,
                    jar,
                )
                .await?
            }
            Opcode::Invokedynamic(invoke) => {
                let dynamic = class.get_bootstrap().unwrap();
                let entry = &dynamic[invoke.attr_index as usize];

                if !should_skip_class(&entry.method.class_name) {
                    parse_method_type_operations(
                        jar.get_class(&entry.method.class_name).unwrap(),
                        &entry.method.member_ref,
                        read_ops,
                        read_hasher,
                        write_ops,
                        write_hasher,
                        jar,
                    )
                    .await?;
                }

                for arg in &entry.arguments {
                    if let BootstrapArgument::MethodHandle(method) = arg {
                        if should_skip_class(&method.class_name) {
                            continue; // Skip Java standard library methods
                        }

                        parse_method_type_operations(
                            jar.get_class(&method.class_name).unwrap(),
                            &method.member_ref,
                            read_ops,
                            read_hasher,
                            write_ops,
                            write_hasher,
                            jar,
                        )
                        .await?;
                    }
                }
            }

            _ => {}
        }
    }

    Ok(())
}

async fn parse_method_type_operations(
    class: &ClassFile<'static>,
    method: &NameAndType<'static>,
    read_ops: &mut Vec<TypeOperation>,
    read_hasher: &mut DefaultHasher,
    write_ops: &mut Vec<TypeOperation>,
    write_hasher: &mut DefaultHasher,
    jar: &JarData,
) -> Result<()> {
    tracing::trace!("Method: \"{}.{}\"", class.this_class, method.name);
    let class_name = &*class.this_class;
    let method_name = method.name.as_ref();

    // Match certain methods and add fields
    match (class_name, method_name) {
        ("net/minecraft/network/FriendlyByteBuf", "readByte") => read_ops.push(TypeOperation::Byte),
        ("net/minecraft/network/FriendlyByteBuf", "readVarInt") => {
            read_ops.push(TypeOperation::VarInt)
        }

        ("net/minecraft/network/FriendlyByteBuf", "writeByte") => {
            write_ops.push(TypeOperation::Byte)
        }
        ("net/minecraft/network/FriendlyByteBuf", "writeVarInt") => {
            write_ops.push(TypeOperation::VarInt)
        }

        _ => {}
    }

    // Skip certain methods that loop infinitely
    if matches!(
        (class_name, method_name),
        (
            "net/minecraft/network/codec/ByteBufCodecs",
            "collection" | "fromCodec" | "fromCodecWithRegistries" | "map",
        ) | (
            "net/minecraft/network/FriendlyByteBuf",
            "readByteArray" | "readUtf" | "writeByteArray" | "writeUtf",
        )
    ) {
        return Ok(());
    };

    let Some(code) = class.get_method_code(&method.name) else {
        tracing::warn!("Failed to find method \"{}.{}\"", class.this_class, method.name);
        return Ok(());
    };

    Box::pin(parse_type_operations(
        class,
        code.bytecode.as_ref().unwrap().opcodes.iter().map(|(_, op)| op.clone()),
        read_ops,
        read_hasher,
        write_ops,
        write_hasher,
        jar,
    ))
    .await
}

fn should_skip_class(class: &str) -> bool {
    !class.starts_with("net/minecraft")
        || class.ends_with("ChatType$Bound")
        || class.ends_with("ClientboundRecipeBookAddPacket$Entry")
        || class.ends_with("NbtAccounter")
        || class.ends_with("Packet")
        || class.ends_with("Payload")
        || class.ends_with("RecipeBookSettings")
        || class.ends_with("RecipeBookSettings$TypeSettings")
        || class.ends_with("ResourceKey")
}
