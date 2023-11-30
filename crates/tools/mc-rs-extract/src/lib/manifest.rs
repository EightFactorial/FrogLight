use chrono::{DateTime, Utc};
use serde::Deserialize;
use thiserror::Error;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::debug;

use crate::{path::minecraft_dir, Version};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct VersionManifest {
    pub latest: ParsedManifestLatest,
    pub versions: Vec<ParsedManifestVersion>,
}

impl VersionManifest {
    pub(crate) const MANIFEST_URL: &'static str =
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

    pub async fn new(refresh: bool) -> Result<VersionManifest, ManifestError> {
        let path = minecraft_dir()
            .ok_or(ManifestError::NoMinecraftDir)?
            .join("versions/version_manifest_v2.json");

        let mut contents: String;
        if refresh || !path.exists() {
            debug!("Downloading manifest from {}", Self::MANIFEST_URL);

            contents = reqwest::get(Self::MANIFEST_URL).await?.text().await?;

            let mut file = File::create(&path).await?;
            file.write_all(contents.as_bytes()).await?;
        } else {
            contents = String::new();

            let mut file = File::open(&path).await?;
            file.read_to_string(&mut contents).await?;
        }

        Ok(serde_json::from_str(&contents)?)
    }

    pub fn get(&self, version: &Version) -> Option<&ParsedManifestVersion> {
        self.versions.iter().find(|v| &v.id == version)
    }
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Could not find .minecraft directory")]
    NoMinecraftDir,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ParsedManifestLatest {
    pub release: Version,
    pub snapshot: Version,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ParsedManifestVersion {
    pub id: Version,
    pub url: String,
    pub time: DateTime<Utc>,
    #[serde(rename = "releaseTime")]
    pub release_time: DateTime<Utc>,
}

impl ParsedManifestVersion {
    pub fn released_before(&self, other: &ParsedManifestVersion) -> bool {
        self.release_time < other.release_time
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ParsedVersionData {
    pub downloads: ParsedVersionDownloads,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ParsedVersionDownloads {
    pub client: ParsedVersionDownload,
    pub server: ParsedVersionDownload,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ParsedVersionDownload {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}
