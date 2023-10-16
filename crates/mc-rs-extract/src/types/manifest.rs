use std::fs::File;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::util::minecraft_dir;

use super::Version;

/// The manifest contains information about the
/// latest and all available versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub latest: ManifestLatest,
    pub versions: Vec<ManifestVersion>,
}

/// The latest versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestLatest {
    pub release: Version,
    pub snapshot: Version,
}

/// A version in the manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestVersion {
    pub id: Version,
    pub url: String,
    pub time: DateTime<Utc>,
    #[serde(rename = "releaseTime")]
    pub release_time: DateTime<Utc>,
}

/// An error that can occur while fetching the manifest
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
    /// The url of the manifest
    const MANIFEST_URL: &'static str =
        "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

    /// Get the latest version
    pub fn get_latest(&self, unstable: bool) -> Version {
        if !unstable || self.latest.release.is_newer(&self.latest.snapshot, self) {
            self.latest.release.clone()
        } else {
            self.latest.snapshot.clone()
        }
    }

    /// Get the manifest
    pub fn get(refresh: bool) -> Result<Manifest, ManifestError> {
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

    /// Get the manifest without caching it
    pub fn get_uncached() -> Result<Manifest, ManifestError> {
        let response = reqwest::blocking::get(Self::MANIFEST_URL)?;
        let manifest: Manifest = serde_json::from_reader(response)?;
        Ok(manifest)
    }
}

// TODO: Fix this test
// #[test]
// fn get_and_parse() { Manifest::get_uncached().unwrap(); }
