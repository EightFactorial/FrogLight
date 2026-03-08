use cafebabe::{
    bytecode::Opcode,
    constant_pool::{MemberRef, NameAndType},
};
use miette::Result;

use crate::{generator::crates::entity::EntityMetadataItem, helper::ClassFileExt, source::JarData};

/// Parse the metadata read using Entity's constructor.
pub(super) fn parse_entity_constructor(
    class: &str,
    jar: &JarData,
) -> Result<Vec<EntityMetadataItem>> {
    let class = jar.get_class(class).unwrap();
    let code = class.get_method_code("<init>").unwrap();

    let mut metadata = Vec::new();
    for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
        if let Opcode::Getstatic(MemberRef { class_name, name_and_type }) = op
            && name_and_type.descriptor == "Lnet/minecraft/network/syncher/EntityDataAccessor;"
        {
            if let Some(serializer) = parse_data_accessor(class_name, name_and_type, jar) {
                metadata.push(serializer);
            } else {
                miette::bail!(
                    "Failed to parse entity constructor: Unknown data accessor \"{class_name}.{}\"",
                    name_and_type.name
                )
            }
        }
    }
    metadata.sort_by_cached_key(|v| class.fields.iter().position(|f| f.name == v.name));

    Ok(metadata)
}

/// Parse the metadata read using the class's "defineSynchedData" method.
pub(super) fn parse_metadata_method(class: &str, jar: &JarData) -> Result<Vec<EntityMetadataItem>> {
    let mut class = jar.get_class(class).unwrap();
    let code;

    loop {
        if let Some(method_data) = class.get_method_code("defineSynchedData") {
            code = method_data;
            break;
        } else if let Some(parent) = class.super_class.as_ref() {
            class = jar.get_class(parent).unwrap();
        } else {
            return Ok(Vec::new());
        }
    }

    let mut metadata = Vec::new();
    let mut this_class = Vec::new();

    for (_, op) in &code.bytecode.as_ref().unwrap().opcodes {
        match op {
            Opcode::Invokespecial(MemberRef { class_name, name_and_type })
                if name_and_type.name == "defineSynchedData" =>
            {
                metadata.extend(parse_metadata_method(class_name, jar)?);
            }
            Opcode::Getstatic(MemberRef { class_name, name_and_type })
                if name_and_type.descriptor
                    == "Lnet/minecraft/network/syncher/EntityDataAccessor;" =>
            {
                if let Some(serializer) = parse_data_accessor(class_name, name_and_type, jar) {
                    this_class.push(serializer);
                } else {
                    miette::bail!(
                        "Failed to parse metadata method: Unknown data accessor \"{class_name}.{}\"",
                        name_and_type.name
                    )
                }
            }
            _ => {}
        }
    }

    this_class.sort_by_cached_key(|v| class.fields.iter().position(|f| f.name == v.name));
    metadata.extend(this_class);

    Ok(metadata)
}

/// Parse an `EntityDataAccessor` into a `EntityMetadataItem`.
pub(super) fn parse_data_accessor(
    class: &str,
    accessor: &NameAndType,
    jar: &JarData,
) -> Option<EntityMetadataItem> {
    let class = jar.get_class(class).unwrap();
    let init = class.get_static_field_init(&accessor.name).unwrap();

    for op in init {
        if let Opcode::Getstatic(MemberRef { class_name: _, name_and_type }) = op
            && name_and_type.descriptor == "Lnet/minecraft/network/syncher/EntityDataSerializer;"
        {
            return Some(EntityMetadataItem {
                name: accessor.name.to_string(),
                serializer: name_and_type.name.to_string(),
            });
        }
    }

    None
}
