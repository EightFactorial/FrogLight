use std::fs::File;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::util::minecraft_dir;

use super::Version;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub latest: ManifestLatest,
    pub versions: Vec<ManifestVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestLatest {
    pub release: Version,
    pub snapshot: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestVersion {
    pub id: Version,
    pub url: String,
    pub time: DateTime<Utc>,
    #[serde(rename = "releaseTime")]
    pub release_time: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Unable to find Minecraft directory")]
    MinecraftDirNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

impl Manifest {
    const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

    pub fn get_latest(&self, unstable: bool) -> Version {
        if unstable {
            self.latest.snapshot.clone()
        } else {
            self.latest.release.clone()
        }
    }

    pub fn get_manifest(refresh: bool) -> Result<Manifest, ManifestError> {
        let mut path = minecraft_dir().ok_or(ManifestError::MinecraftDirNotFound)?;
        path.push("versions/version_manifest_v2.json");

        if refresh || !path.exists() {
            let mut response = reqwest::blocking::get(Self::MANIFEST_URL)?;
            response.copy_to(&mut File::create(path.clone())?)?;
        }

        let file = File::open(path)?;
        let manifest: Manifest = serde_json::from_reader(file)?;
        Ok(manifest)
    }
}
