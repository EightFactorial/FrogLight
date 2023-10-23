use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

/// Data for a version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionData {
    #[serde(rename = "assetIndex")]
    pub asset_index: VersionAssetIndex,
    pub downloads: VersionDownloads,
}

/// Asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAssetIndex {
    pub id: String,
    pub sha1: String,
    pub url: String,
}

/// Download data for the client and server jars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: VersionDownload,
    pub server: VersionDownload,
}

/// A URL to a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownload {
    pub url: String,
}

/// The assets of a version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAssets {
    pub objects: HashMap<String, VersionAsset>,
}

/// An asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAsset {
    pub hash: String,
    pub size: u64,
}
