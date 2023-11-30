use std::{path::Path, process::Stdio};

use classfile::classfile::ClassFile;
use hashbrown::HashMap;
use json::JsonValue;

use thiserror::Error;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};
use tracing::debug;
use zip::ZipArchive;

use crate::{
    manifest::{ManifestError, ParsedManifestVersion, ParsedVersionData, VersionManifest},
    path::minecraft_dir,
    Version,
};

/// A struct containing all the data needed to generate the output.
///
/// Modules should use this struct to get the data they need to generate their output,
/// and should insert the resulting data into the `output` field.
pub struct ModuleData {
    /// The version of Minecraft that data is being extracted from.
    pub version: Version,
    /// The [`VersionManifest`](crate::manifest::VersionManifest),
    /// used to get information about versions.
    pub manifest: VersionManifest,

    /// The Minecraft jar, opened as a zip file.
    pub zip: ZipArchive<std::fs::File>,
    /// A map of class names to their corresponding deobfuscated
    /// [`ClassFile`](classfile::classfile::ClassFile).
    pub classmap: HashMap<String, ClassFile>,

    /// The output of all previously run modules.
    pub output: JsonValue,
}

#[derive(Debug, Error)]
pub enum ModuleDataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("Classfile error: {0}")]
    Classfile(#[from] classfile::error::ParserError),

    #[error("Manifest error: {0}")]
    Manifest(#[from] ManifestError),
    #[error("Could not find .minecraft directory")]
    NoMinecraftDir,
    #[error("Invalid Version, not in VersionManifest")]
    InvalidVersion,
}

impl ModuleData {
    pub async fn new(
        version: Option<Version>,
        refresh: bool,
    ) -> Result<ModuleData, ModuleDataError> {
        // Get the VersionManifest
        let manifest = VersionManifest::new(refresh).await?;

        // Use the latest release if no Version was specified
        let version = version.unwrap_or_else(|| manifest.latest.release.clone());

        // Get the ParsedManifestVersion, making sure the Version is valid
        let Some(manifest_version) = manifest.get(&version) else {
            return Err(ModuleDataError::InvalidVersion);
        };
        debug!("Found {version:?} in VersionManifest, continuing...");

        let jar = {
            let folder_path = minecraft_dir()
                .ok_or(ModuleDataError::NoMinecraftDir)?
                .join("versions")
                .join(version.to_string());

            // Create the folder if it doesn't exist
            if !folder_path.exists() {
                tokio::fs::create_dir_all(&folder_path).await?;
            }

            // Get the mapped jar
            let mapped_jar_path = folder_path.join(format!("{}-mapped.jar", version));
            if !mapped_jar_path.exists() {
                // Get the jar
                let jar_path = folder_path.join(format!("{}.jar", version));
                if !jar_path.exists() {
                    get_mc_jar(&jar_path, manifest_version).await?;
                }

                // Get the mappings
                let mappings_path = folder_path.join("mappings.tiny");
                if !mappings_path.exists() {
                    // Get the mappings jar
                    let mappings_jar_path = folder_path.join("intermediary-mappings.jar");
                    if !mappings_jar_path.exists() {
                        get_mappings_jar(&mappings_jar_path, &version).await?;
                    }

                    // Extract the mappings from the jar
                    get_mappings_file_from_mappings_jar(&mappings_path, &mappings_jar_path).await?;
                }

                // Get the remapper
                let remapper_path = folder_path.parent().unwrap().join("tiny-remapper.jar");
                if !remapper_path.exists() {
                    get_remapper(&remapper_path).await?;
                }

                // Map the jar
                let mut command = Command::new("java")
                    .arg("-jar")
                    .arg(remapper_path)
                    .arg(jar_path)
                    .arg(&mapped_jar_path)
                    .arg(mappings_path)
                    .arg("official")
                    .arg("intermediary")
                    .current_dir(folder_path)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .spawn()?;

                command.wait().await?;
            }

            // Open the mapped jar
            debug!("Opening mapped jar at {}", mapped_jar_path.display());
            std::fs::File::open(mapped_jar_path)?
        };
        let mut zip = ZipArchive::new(jar)?;

        // Create the ClassMap
        let mut classmap = HashMap::with_capacity(
            zip.file_names()
                .filter(|name| name.ends_with(".class"))
                .count(),
        );
        for index in 0..zip.len() {
            let Ok(mut file) = zip.by_index(index) else {
                continue;
            };

            // Skip non-class files
            if !file.name().ends_with(".class") {
                continue;
            }

            // Parse the classfile
            match ClassFile::parse(&mut file) {
                Ok(class) => classmap.insert(class.this_class.to_string(), class),
                Err(err) => {
                    debug!("Error while parsing classfile {}: {err}", file.name());
                    continue;
                }
            };
        }

        // Create the ModuleData
        Ok(ModuleData {
            version,
            manifest,
            classmap,
            zip,
            output: JsonValue::new_object(),
        })
    }
}

/// Downloads the Minecraft jar from the given `manifest_version` and saves it to `jar_path`.
async fn get_mc_jar(
    jar_path: &Path,
    manifest_version: &ParsedManifestVersion,
) -> Result<(), ModuleDataError> {
    debug!("Downloading version data from {}", manifest_version.url);
    let data_response: ParsedVersionData =
        reqwest::get(&manifest_version.url).await?.json().await?;

    debug!(
        "Downloading jar from {}",
        data_response.downloads.client.url
    );
    let response = reqwest::get(&data_response.downloads.client.url)
        .await?
        .bytes()
        .await?;

    let mut jar = File::create(&jar_path).await?;
    jar.write_all(&response).await?;
    jar.flush().await?;

    Ok(())
}

/// Downloads the mappings jar for the given `version` and saves it to `mappings_jar_path`.
async fn get_mappings_jar(
    mappings_jar_path: &Path,
    version: &Version,
) -> Result<(), ModuleDataError> {
    // Slice off the version's `.0` patch if it's a release, release candidate, or pre-release
    let ver_string = match version {
        Version::Release {
            major,
            minor,
            patch,
        } => {
            if *patch == 0 {
                format!("{}.{}", major, minor)
            } else {
                format!("{}.{}.{}", major, minor, patch)
            }
        }
        Version::ReleaseCandidate {
            major,
            minor,
            patch,
            rc,
        } => {
            if *patch == 0 {
                format!("{}.{}-rc{}", major, minor, rc)
            } else {
                format!("{}.{}.{}-rc{}", major, minor, patch, rc)
            }
        }
        Version::PreRelease {
            major,
            minor,
            patch,
            pre,
        } => {
            if *patch == 0 {
                format!("{}.{}-pre{}", major, minor, pre)
            } else {
                format!("{}.{}.{}-pre{}", major, minor, patch, pre)
            }
        }
        _ => version.to_string(),
    };

    // Download the mappings jar
    let url = format!(
        "https://maven.fabricmc.net/net/fabricmc/intermediary/{VER}/intermediary-{VER}-v2.jar",
        VER = ver_string
    );
    debug!("Downloading mappings from {url}");

    let response = reqwest::get(url).await?.bytes().await?;

    let mut mappings_jar = File::create(&mappings_jar_path).await?;
    mappings_jar.write_all(&response).await?;
    mappings_jar.flush().await?;

    Ok(())
}

/// Extracts the mappings file from the mappings jar
/// at `mappings_jar_path` and saves it to `mappings_path`.
async fn get_mappings_file_from_mappings_jar(
    mappings_path: &Path,
    mappings_jar_path: &Path,
) -> Result<(), ModuleDataError> {
    let mut file = File::open(&mappings_jar_path)
        .await?
        .try_into_std()
        .unwrap();
    let mut zip = ZipArchive::new(&mut file)?;
    let mut zipped_mappings = zip.by_name("mappings/mappings.tiny")?;

    let mut mappings = std::fs::File::create(mappings_path)?;
    std::io::copy(&mut zipped_mappings, &mut mappings)?;
    File::from_std(mappings).flush().await?;

    Ok(())
}

/// Downloads the tiny-remapper jar and saves it to `remapper_path`.
async fn get_remapper(remapper_path: &Path) -> Result<(), ModuleDataError> {
    let url = format!(
        "https://maven.fabricmc.net/net/fabricmc/tiny-remapper/{VER}/tiny-remapper-{VER}-fat.jar",
        VER = "0.8.9"
    );
    debug!("Downloading tiny-remapper from {url}");

    let response = reqwest::get(url).await?.bytes().await?;

    let mut remapper = File::create(&remapper_path).await?;
    remapper.write_all(&response).await?;
    remapper.flush().await?;

    Ok(())
}

impl std::fmt::Debug for ModuleData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleData")
            .field("version", &self.version)
            .field("classmap length", &self.classmap.len())
            .field("data length", &self.output.len())
            .finish()
    }
}
