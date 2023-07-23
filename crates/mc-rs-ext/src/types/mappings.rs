use std::{fs::File, hash::Hash, io::read_to_string, mem};

use classfile::{
    ast::{Insn, LdcType},
    classfile::ClassFile,
    error::ParserError,
    types::Type,
};
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;
use log::error;
use thiserror::Error;
use zip::ZipArchive;

use crate::util::{get_mappings, minecraft_jar};

use super::{Manifest, Version};

/// A map of class names to class files
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct ClassMap(HashMap<String, ClassFile>);

impl ClassMap {
    /// Create a new class map with the given capacity
    pub fn with_capacity(size: usize) -> Self { Self(HashMap::with_capacity(size)) }
}

impl IntoIterator for ClassMap {
    type Item = (String, ClassFile);
    type IntoIter = hashbrown::hash_map::IntoIter<String, ClassFile>;
    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl ClassMap {
    /// Create a new class map with mappings applied
    pub fn new_mapped(version: &Version, manifest: &Manifest) -> Result<ClassMap, MappingsError> {
        let mut map = ClassMap::new(version, manifest)?;
        map.apply_mappings(version)?;
        Ok(map)
    }

    /// Create a new class map for the given version
    pub fn new(version: &Version, manifest: &Manifest) -> Result<ClassMap, MappingsError> {
        let path = minecraft_jar(version, manifest).ok_or(MappingsError::McDirNotFound)?;
        let jar = File::open(path)?;

        let mut zip = ZipArchive::new(jar)?;
        let mut map = ClassMap::with_capacity(8192);

        for i in 0..zip.len() {
            let mut file = match zip.by_index(i) {
                Ok(f) => f,
                Err(err) => {
                    error!("Failed to get file from minecraft jar: {}", err);
                    continue;
                }
            };

            if file.is_dir() || !file.name().ends_with(".class") {
                continue;
            }

            let class = match ClassFile::parse(&mut file) {
                Ok(c) => c,
                Err(err) => {
                    if !matches!(
                        err,
                        ParserError::Unrecognized(_, _)
                            | ParserError::UnknownInstruction { .. }
                            | ParserError::IO(_)
                    ) {
                        error!("Failed to parse class file: {}", err);
                    }

                    continue;
                }
            };

            map.insert(class.this_class.to_string(), class);
        }

        #[cfg(debug_assertions)]
        {
            log::debug!("Found {} classes", map.len());
        }

        Ok(map)
    }

    /// Apply mappings to the class map
    pub fn apply_mappings(&mut self, ver: &Version) -> Result<(), MappingsError> {
        let mappings = Mappings::get(ver)?;

        for (key, mapping) in mappings.iter() {
            if let Some((key, mut class)) = self.remove_entry(key) {
                class.this_class = mapping.name.clone();

                for method in class.methods.iter_mut() {
                    if let Some((kind, _, name)) = mapping
                        .methods
                        .iter()
                        .find(|(_, obf, _)| method.name == *obf)
                    {
                        method.name = name.clone();
                        method.descriptor = kind.clone();
                    }

                    // TODO: Map attributes
                    // for attribute in method.attributes.iter_mut() {
                    //     match attribute {
                    //         Attribute::ConstantValue(_) => todo!(),
                    //         Attribute::Signature(_) => todo!(),
                    //         Attribute::Code(_) => todo!(),
                    //         Attribute::Exceptions(_) => todo!(),
                    //         Attribute::SourceFile(_) => todo!(),
                    //         Attribute::LocalVariableTable(_) => todo!(),
                    //         Attribute::Unknown(_) => todo!(),
                    //     }
                    // }

                    if let Some(code) = method.code() {
                        for insn in code.insns.insns.iter_mut() {
                            match insn {
                                Insn::CheckCast(insn) => {
                                    insn.kind = mapping.name.clone();
                                }
                                Insn::GetField(insn) => {
                                    insn.class = mapping.name.clone();
                                    Self::set_name_and_descriptor(
                                        &mut insn.name,
                                        &mut insn.descriptor,
                                        &mapping.fields,
                                        &key,
                                        mapping,
                                    );
                                }
                                Insn::PutField(insn) => {
                                    insn.class = mapping.name.clone();
                                    Self::set_name_and_descriptor(
                                        &mut insn.name,
                                        &mut insn.descriptor,
                                        &mapping.fields,
                                        &key,
                                        mapping,
                                    );
                                }
                                Insn::NewObject(insn) => {
                                    if insn.kind == key {
                                        insn.kind = mapping.name.clone();
                                    } else if let Some((_, _, name)) =
                                        mapping.fields.iter().find(|(_, field_obf_name, _)| {
                                            insn.kind == **field_obf_name
                                        })
                                    {
                                        insn.kind = name.clone();
                                    }
                                }
                                Insn::NewArray(insn) => {
                                    if let Type::Reference(Some(kind)) = &mut insn.kind {
                                        if kind == &key {
                                            *kind = mapping.name.clone();
                                        } else if let Some((_, _, name)) = mapping
                                            .fields
                                            .iter()
                                            .find(|(_, field_obf_name, _)| kind == field_obf_name)
                                        {
                                            *kind = name.clone();
                                        }
                                    }
                                }
                                Insn::Invoke(insn) => {
                                    insn.class = mapping.name.clone();
                                    Self::set_name_and_descriptor(
                                        &mut insn.name,
                                        &mut insn.descriptor,
                                        &mapping.methods,
                                        &key,
                                        mapping,
                                    );
                                }
                                Insn::InstanceOf(insn) => {
                                    Self::resolve_name(
                                        &mut insn.class,
                                        &mapping.fields,
                                        &key,
                                        mapping,
                                        &mappings,
                                    );
                                }
                                Insn::Ldc(insn) => {
                                    if let LdcType::Class(class_kind) = &mut insn.constant {
                                        Self::resolve_name(
                                            class_kind,
                                            &mapping.fields,
                                            &key,
                                            mapping,
                                            &mappings,
                                        );
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                for field in class.fields.iter_mut() {
                    if let Some((kind, _, name)) = mapping
                        .fields
                        .iter()
                        .find(|(_, obf, _)| field.name == **obf)
                    {
                        field.name = name.clone();
                        field.descriptor = kind.clone();
                    }
                }

                self.insert(mapping.name.clone(), class);
            }
        }

        Ok(())
    }

    /// Set the name and descriptor of an instruction
    fn set_name_and_descriptor(
        insn_name: &mut String,
        insn_desc: &mut String,
        array: &[(String, String, String)],
        key: &str,
        mapping: &ClassMappings,
    ) {
        if let Some((kind, _, name)) = array.iter().find(|(_, obf, _)| insn_name == obf) {
            *insn_name = name.clone();

            let mut desc = kind.clone().replace(&format!("L{};", key), &mapping.name);
            for (_, field_obf_name, field_name) in mapping.fields.iter() {
                desc = desc.replace(&format!("L{};", field_obf_name), field_name);
            }
            *insn_desc = desc;
        }
    }

    /// Resolve the name of an instruction,
    /// potentially replacing it with a field from another class
    fn resolve_name(
        insn_name: &mut String,
        array: &[(String, String, String)],
        key: &str,
        mapping: &ClassMappings,
        mappings: &Mappings,
    ) {
        if insn_name == key {
            *insn_name = mapping.name.clone();
        } else if let Some((_, _, name)) = mapping
            .fields
            .iter()
            .find(|(_, field_obf_name, _)| insn_name == field_obf_name)
        {
            *insn_name = name.clone();
        } else if insn_name.contains('$') {
            let (class, kind) = insn_name.split_once('$').unwrap();
            let mut class = class.to_string();
            let mut kind = kind.to_string();

            if let Some(class_mapping) = mappings.get(&class) {
                class = class_mapping.name.clone();

                if let Some((_, _, name)) = array
                    .iter()
                    .find(|(_, field_obf_name, _)| &kind == field_obf_name)
                {
                    kind = name.clone();
                }
            }

            *insn_name = format!("{}${}", class, kind);
        }
    }
}

/// The mappings for a given jar
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Mappings(HashMap<String, ClassMappings>);

/// The mappings for a single class
#[derive(Debug, Default, Clone, Hash)]
pub struct ClassMappings {
    pub name: String,
    pub methods: Vec<(String, String, String)>,
    pub fields: Vec<(String, String, String)>,
}

impl Mappings {
    /// Get the mappings for a given version
    fn get(ver: &Version) -> Result<Mappings, MappingsError> {
        let path = get_mappings(ver)?;
        let jar = File::open(path)?;
        let mut zip = ZipArchive::new(jar)?;

        let tiny = zip.by_name("mappings/mappings.tiny")?;
        let tiny_string = read_to_string(tiny)?;

        let mut mappings = Mappings::default();
        let mut class_obf_name = String::new();
        let mut class_mappings = ClassMappings::default();

        let mut lines = tiny_string.lines();
        lines.next(); // skip first line

        for line in lines {
            let mut parts = line.split('\t');

            // Skip the first part if it's empty
            if matches!(parts.clone().peekable().peek(), Some(&"")) {
                parts.next();
            }

            match MappingType::try_from(parts.next().unwrap()) {
                Ok(MappingType::Class) => {
                    if !class_obf_name.is_empty() {
                        mappings.insert(
                            mem::take(&mut class_obf_name),
                            mem::take(&mut class_mappings),
                        );
                    }

                    class_obf_name = parts.next().unwrap().to_string();
                    class_mappings.name = parts.next().unwrap().to_string();
                }
                Ok(m) => {
                    let kind = parts.next().unwrap().to_string();
                    let obf_name = parts.next().unwrap().to_string();
                    let name = parts.next().unwrap().to_string();

                    // Put the new class name in the kind
                    let kind = kind.replace(&class_obf_name, &class_mappings.name);

                    match m {
                        MappingType::Field => {
                            class_mappings.fields.push((kind, obf_name, name));
                        }
                        MappingType::Method => {
                            class_mappings.methods.push((kind, obf_name, name));
                        }
                        _ => unreachable!(),
                    }
                }

                Err(err) => {
                    error!("Failed to parse mapping type: {}", err);
                    continue;
                }
            };
        }

        Ok(mappings)
    }
}

/// An error that can occur while getting the mappings
#[derive(Debug, Error)]
pub enum MappingsError {
    #[error("Minecraft directory not found")]
    McDirNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
}

/// The type of mapping
#[derive(Debug, Clone, Copy)]
pub enum MappingType {
    Field,
    Method,
    Class,
}

impl TryFrom<&str> for MappingType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "f" => Ok(MappingType::Field),
            "m" => Ok(MappingType::Method),
            "c" => Ok(MappingType::Class),
            e => Err(e.to_string()),
        }
    }
}
