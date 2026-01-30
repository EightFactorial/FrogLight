use std::fmt::Write;

use facet::Facet;
use miette::Result;
use tokio::sync::RwLock;

use crate::{
    common::{DATA, Version, WORKSPACE_DIR},
    config::ConfigBundle,
    helper::ModuleBuilder,
    source::JarFile,
};

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
struct VersionInfo {
    id: Version,
    name: Version,
    world_version: u32,
    protocol_version: u32,
    pack_version: VersionPackInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Facet)]
struct VersionPackInfo {
    resource_major: u32,
}

/// Generates structs that implement `Version`.
pub async fn generate(config: &ConfigBundle) -> Result<()> {
    let mut content = String::new();

    for version in &config.versions {
        let pinned = DATA.pin_owned();
        let storage_lock = pinned.get_or_insert_with(version.real.clone(), RwLock::default);

        // Get the JAR file for the version
        let mut storage = storage_lock.write().await;
        JarFile::get_for(&version.real, &mut storage, async |file| {
            let mut reader = file.reader.clone();

            // Get the "version.json" file index
            let Some(entry) = reader
                .file()
                .entries()
                .iter()
                .position(|entry| entry.filename().as_str().is_ok_and(|n| n == "version.json"))
            else {
                miette::bail!(
                    "Failed to find \"version.json\" in JAR for \"{}\"",
                    version.real.as_str()
                );
            };

            // Read and parse the "version.json" file
            let mut buffer = Vec::new();
            let mut reader = reader.reader_with_entry(entry).await.unwrap();
            reader.read_to_end_checked(&mut buffer).await.unwrap();
            let Ok(info) = facet_json::from_slice::<VersionInfo>(buffer.as_slice()) else {
                miette::bail!(
                    "Failed to parse \"version.json\" in JAR for \"{}\"",
                    version.real.as_str(),
                );
            };

            // Generate the version struct and trait implementation
            let version_raw = version.base.as_str();
            let version_feature = version.base.as_feature();
            let version_name = version_feature.to_ascii_uppercase();

            write!(
                content,
                r#"
/// Minecraft {version_raw}
///
/// See the [Minecraft Wiki](https://minecraft.wiki/w/Java_Edition_{version_raw}) for more details.
#[cfg(feature = "{version_feature}")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(bevy_reflect::Reflect))]
#[cfg_attr(feature = "bevy", reflect(Debug, Clone, PartialEq, Hash))]
pub struct {version_name};

#[cfg(feature = "{version_feature}")]
impl super::Version for {version_name} {{
const DATA_VERSION: u32 = {};
const PROTOCOL_ID: u32 = {};
const RESOURCE_VERSION: u32 = {};
}}
"#,
                info.world_version, info.protocol_version, info.pack_version.resource_major,
            )
            .unwrap();

            Ok(())
        })
        .await?;
    }

    // Create and build the module
    let path = WORKSPACE_DIR.join("froglight-common/src/version");
    let mut module = ModuleBuilder::new("generated", path);
    module.with_docs(
        "Version types\n\nThis file is automatically @generated, do not edit it manually.",
    );
    module.with_content(&content);
    module.build().await
}
