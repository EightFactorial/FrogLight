use std::{fs::File, path::PathBuf};

use classfile::{classfile::ClassFile, error::ParserError};
use derive_more::{Deref, DerefMut};
use hashbrown::HashMap;
use thiserror::Error;
use tracing::error;
use zip::ZipArchive;

use crate::util::{mapped_minecraft_jar, minecraft_jar};

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
    pub fn new(version: &Version, manifest: &Manifest) -> Result<ClassMap, MappingsError> {
        let path = minecraft_jar(version, manifest).ok_or(MappingsError::JarNotFound)?;
        Self::open_jar(path)
    }

    /// Create a new class map with mappings applied
    pub fn new_with_mappings(
        version: &Version,
        manifest: &Manifest,
    ) -> Result<ClassMap, MappingsError> {
        let path = mapped_minecraft_jar(version, manifest)?;
        Self::open_jar(path)
    }

    /// Create a new class map for the given version
    fn open_jar(path: PathBuf) -> Result<ClassMap, MappingsError> {
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
                    if !matches!(err, ParserError::IO(_)) {
                        error!("Failed to parse class file: {}", err);
                    }

                    continue;
                }
            };

            map.insert(class.this_class.to_string(), class);
        }

        #[cfg(debug_assertions)]
        {
            tracing::debug!("Found {} classes", map.len());
        }

        Ok(map)
    }
}

/// An error that can occur while getting the mappings
#[derive(Debug, Error)]
pub enum MappingsError {
    #[error("Path not found")]
    PathNotFound,
    #[error("Jar not found")]
    JarNotFound,
    #[error("Mapping failed")]
    MappingFailed,
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
