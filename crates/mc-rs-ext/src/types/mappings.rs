use std::{fs::File, hash::Hash, io::read_to_string, mem};

use classfile::{ast::Insn, classfile::ClassFile, error::ParserError};
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;
use log::error;
use thiserror::Error;
use zip::ZipArchive;

use crate::util::{get_mappings, minecraft_jar};

use super::Version;

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct ClassMap(HashMap<String, ClassFile>);

impl ClassMap {
    pub fn with_capacity(size: usize) -> Self { Self(HashMap::with_capacity(size)) }
}

impl IntoIterator for ClassMap {
    type Item = (String, ClassFile);
    type IntoIter = hashbrown::hash_map::IntoIter<String, ClassFile>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl ClassMap {
    pub fn new_mapped(version: &Version) -> Result<ClassMap, MappingsError> {
        ClassMap::new(version)?.apply_mappings(version)
    }

    pub fn new(version: &Version) -> Result<ClassMap, MappingsError> {
        let path = minecraft_jar(version).ok_or(MappingsError::McDirNotFound)?;
        let jar = File::open(path)?;

        let mut zip = ZipArchive::new(jar)?;
        let len = zip.len();

        let mut map = ClassMap::with_capacity(len);
        for i in 0..len {
            let mut file = match zip.by_index(i) {
                Ok(f) => f,
                Err(err) => {
                    error!("Failed to get file from minecraft jar: {}", err);
                    continue;
                }
            };

            if file.is_dir() {
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

    pub fn apply_mappings(mut self, ver: &Version) -> Result<ClassMap, MappingsError> {
        let mut new_map = HashMap::with_capacity(self.len());
        let mappings = Mappings::new(ver)?;

        for (key, value) in mappings.iter() {
            if let Some((_, mut class)) = self.remove_entry(key) {
                class.this_class = value.name.clone();

                for method in class.methods.iter_mut() {
                    for (signature, obf, name) in value.methods.iter() {
                        if method.name == *obf {
                            method.name = name.clone();
                            method.descriptor = signature.clone();
                            break;
                        }
                    }

                    if let Some(code) = method.code() {
                        for insn in code.insns.insns.iter_mut() {
                            match insn {
                                Insn::CheckCast(insn) => {
                                    if let Some(class) = mappings.get(&insn.kind) {
                                        insn.kind = class.name.clone();
                                    }
                                }
                                Insn::GetField(insn) => {
                                    if let Some(class) = mappings.get(&insn.class) {
                                        insn.class = class.name.clone();

                                        for (kind, obf, name) in class.fields.iter() {
                                            if insn.name == *obf {
                                                insn.name = name.clone();
                                                insn.descriptor = kind.clone();
                                                break;
                                            }
                                        }
                                    }
                                }
                                Insn::PutField(insn) => {
                                    if let Some(class) = mappings.get(&insn.class) {
                                        insn.class = class.name.clone();

                                        for (kind, obf, name) in class.fields.iter() {
                                            if insn.name == *obf {
                                                insn.name = name.clone();
                                                insn.descriptor = kind.clone();
                                                break;
                                            }
                                        }
                                    }
                                }
                                Insn::InstanceOf(insn) => {
                                    if let Some(class) = mappings.get(&insn.class) {
                                        insn.class = class.name.clone();
                                    }
                                }
                                Insn::Invoke(insn) => {
                                    if let Some(class) = mappings.get(&insn.class) {
                                        insn.class = class.name.clone();

                                        for (signature, obf, name) in class.methods.iter() {
                                            if insn.name == *obf {
                                                insn.name = name.clone();
                                                insn.descriptor = signature.clone();
                                                break;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                for field in class.fields.iter_mut() {
                    for (kind, obf, name) in value.fields.iter() {
                        if field.name == *obf {
                            field.name = name.clone();
                            field.descriptor = kind.clone();
                            break;
                        }
                    }
                }

                new_map.insert(value.name.clone(), class);
            }
        }

        #[cfg(debug_assertions)]
        {
            log::debug!("Unmapped classes: {:#?}", self.keys());
        }

        // Carry over any classes that weren't in the mappings
        for (key, value) in self.into_iter() {
            new_map.insert(key, value);
        }

        Ok(ClassMap(new_map))
    }
}

#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Mappings(HashMap<String, ClassMappings>);

#[derive(Debug, Default, Clone, Hash)]
pub struct ClassMappings {
    pub name: String,
    pub methods: Vec<(String, String, String)>,
    pub fields: Vec<(String, String, String)>,
}

impl Mappings {
    fn new(ver: &Version) -> Result<Mappings, MappingsError> {
        let path = get_mappings(ver)?;
        let jar = File::open(path)?;
        let mut zip = ZipArchive::new(jar)?;

        let tiny = zip.by_name("mappings/mappings.tiny")?;
        let tiny_string = read_to_string(tiny)?;

        let mut mappings = Mappings::default();
        let mut obf_name = String::new();
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
                    if !obf_name.is_empty() {
                        mappings.insert(mem::take(&mut obf_name), mem::take(&mut class_mappings));
                    }

                    obf_name = parts.next().unwrap().to_string();
                    class_mappings.name = parts.next().unwrap().to_string();
                }
                Ok(MappingType::Method) => {
                    let signature = parts.next().unwrap().to_string();
                    let obf_name = parts.next().unwrap().to_string();
                    let name = parts.next().unwrap().to_string();

                    class_mappings.methods.push((signature, obf_name, name));
                }
                Ok(MappingType::Field) => {
                    let kind = parts.next().unwrap().to_string();
                    let obf_name = parts.next().unwrap().to_string();
                    let name = parts.next().unwrap().to_string();

                    class_mappings.fields.push((kind, obf_name, name));
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
