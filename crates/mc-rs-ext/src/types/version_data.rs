use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionData {
    #[serde(rename = "assetIndex")]
    pub asset_index: VersionAssetIndex,
    pub downloads: VersionDownloads,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAssetIndex {
    pub id: String,
    pub sha1: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownloads {
    pub client: VersionDownload,
    pub server: VersionDownload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDownload {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAssets {
    pub objects: HashMap<String, VersionAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionAsset {
    pub hash: String,
    pub size: u64,
}
