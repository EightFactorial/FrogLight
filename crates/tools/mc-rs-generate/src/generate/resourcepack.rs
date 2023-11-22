use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

use git2::Repository;
use json::JsonValue;
use log::{error, info, trace, warn};
use mc_rs_extract::{
    extract::datasets::Datasets,
    types::{Manifest, Version},
};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use tempfile::tempdir;
use zip::{ZipArchive, ZipWriter};

use super::Generator;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourcePack;

impl Generator for ResourcePack {
    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(&self, version: &Version, _data: &JsonValue, repo: &Repository) {
        // Get manifest
        let manifest = match Manifest::get(false) {
            Ok(m) => m,
            Err(err) => {
                error!("Failed to get manifest: {err}");
                return;
            }
        };

        // Get versions folder
        let version_folder = match Self::get_version_folder(version) {
            Some(folder) => folder,
            None => {
                error!("Failed to get version folder");
                return;
            }
        };

        // Get the asset index url
        let asset_index_url = match Self::get_asset_index_url(version, &version_folder, &manifest) {
            Some(url) => url,
            None => {
                error!("Failed to get asset index url");
                return;
            }
        };

        // Get the asset index
        let asset_index = match Self::get_asset_index(&asset_index_url, &version_folder) {
            Some(index) => index,
            None => {
                error!("Failed to get asset index");
                return;
            }
        };
        info!("Downloading {} assets...", asset_index.objects.len());

        // Create temporary directory with zip file
        let Ok(tempdir) = tempdir() else {
            error!("Failed to create temporary directory");
            return;
        };
        let zip_path = tempdir.path().join("minecraft.zip");
        let mut zip = ZipWriter::new(File::create(&zip_path).unwrap());

        // Download assets from index to zip
        Self::download_assets_to_zip(&asset_index, &mut zip);

        // Get version jar
        let Some(jar) = mc_rs_extract::util::minecraft_jar(version, &manifest) else {
            error!("Failed to get version jar");
            return;
        };
        let mut jar = ZipArchive::new(File::open(jar).unwrap()).unwrap();

        // Add assets from jar to zip
        Self::jar_assets_to_zip(&mut jar, &mut zip);

        // Finish writing to zip
        if let Err(err) = zip.finish() {
            error!("Failed to finish writing to zip: {err}");
            return;
        }

        // Move the zip to the repo directory
        let final_path = repo.path().parent().unwrap().join("minecraft.zip");
        if let Err(err) = fs::copy(zip_path, &final_path) {
            error!("Failed to move resourcepack to repo directory: {err}");
            return;
        }

        info!("Created resourcepack at {}", final_path.display());
    }
}

impl ResourcePack {
    fn get_version_folder(version: &Version) -> Option<PathBuf> {
        let versions_folder = match mc_rs_extract::util::minecraft_dir() {
            Some(folder) => folder.join("versions"),
            None => {
                error!("Failed to get minecraft directory");
                return None;
            }
        };

        // Get the folder for the version given
        let version_folder = versions_folder.join(version.to_string());

        // Try to create the folder if it doesn't exist
        if !version_folder.exists() {
            if let Err(err) = fs::create_dir(&version_folder) {
                error!("Failed to create version folder: {err}");
                return None;
            }
        }

        Some(version_folder)
    }

    fn get_asset_index_url(
        version: &Version,
        version_folder: &Path,
        manifest: &Manifest,
    ) -> Option<String> {
        let version_json = match Self::get_version_json(version, version_folder, manifest) {
            Some(json) => json,
            None => {
                error!("Failed to get version json");
                return None;
            }
        };

        let JsonValue::Object(version_json) = version_json else {
            error!("Version json is not an object");
            return None;
        };

        let Some(asset_index) = version_json.get("assetIndex") else {
            error!("Version json does not have an assetIndex");
            return None;
        };

        let JsonValue::Object(asset_index) = asset_index else {
            error!("Asset index is not an object");
            return None;
        };

        let Some(asset_index_url) = asset_index.get("url") else {
            error!("Asset index does not have a url");
            return None;
        };

        let JsonValue::String(asset_index_url) = asset_index_url else {
            error!("Asset index url is not a string");
            return None;
        };

        Some(asset_index_url.to_string())
    }

    fn get_version_json(
        version: &Version,
        version_folder: &Path,
        manifest: &Manifest,
    ) -> Option<JsonValue> {
        // Get the version json
        let version_json_path = version_folder.join(format!("{version}.json"));

        // Try to download the version json if it doesn't exist
        if !version_json_path.exists() {
            let version_manifest = manifest
                .versions
                .iter()
                .find(|&v| &v.id == version)
                .unwrap();

            let mut response = match reqwest::blocking::get(&version_manifest.url) {
                Ok(r) => r,
                Err(err) => {
                    error!("Failed to fetch version data: {err}");
                    return None;
                }
            };

            let mut file = match File::create(&version_json_path) {
                Ok(f) => f,
                Err(err) => {
                    error!(
                        "Failed to create version data file {}: {err}",
                        version_json_path.display(),
                    );
                    return None;
                }
            };
            if let Err(err) = response.copy_to(&mut file) {
                error!("Failed to write version data: {err}");
                return None;
            }
        }

        // Read the version json
        let version_json = match fs::read_to_string(&version_json_path) {
            Ok(s) => s,
            Err(err) => {
                error!(
                    "Failed to read version data file {}: {err}",
                    version_json_path.display(),
                );
                return None;
            }
        };

        match json::parse(&version_json) {
            Ok(json) => Some(json),
            Err(err) => {
                error!(
                    "Failed to parse version data file {}: {err}",
                    version_json_path.display(),
                );
                None
            }
        }
    }

    fn get_asset_index(asset_index_url: &str, version_folder: &Path) -> Option<AssetIndexFile> {
        // Get the asset index path
        let file_name = asset_index_url.split('/').last().unwrap();
        let asset_index_path = version_folder.join(file_name);

        // Try to download the asset index if it doesn't exist
        if !asset_index_path.exists() {
            let mut response = match reqwest::blocking::get(asset_index_url) {
                Ok(r) => r,
                Err(err) => {
                    error!("Failed to fetch asset index: {err}");
                    return None;
                }
            };

            let Ok(mut file) = File::create(&asset_index_path) else {
                error!(
                    "Failed to create asset index file {}",
                    asset_index_path.display(),
                );
                return None;
            };

            if let Err(err) = response.copy_to(&mut file) {
                error!("Failed to write asset index: {err}");
                return None;
            }
        }

        let Ok(file) = File::open(&asset_index_path) else {
            error!(
                "Failed to open asset index file {}",
                asset_index_path.display(),
            );
            return None;
        };

        match serde_json::from_reader(file) {
            Ok(index) => Some(index),
            Err(err) => {
                error!(
                    "Failed to parse asset index file {}: {err}",
                    asset_index_path.display(),
                );
                None
            }
        }
    }

    fn download_assets_to_zip(
        asset_index: &AssetIndexFile,
        zip: &mut ZipWriter<File>,
    ) -> Option<()> {
        let client = Client::builder().build().unwrap();
        let mut percent = 0f32;

        'asset_loop: for (index, (path, object)) in asset_index.objects.iter().enumerate() {
            let mut path = path.clone();

            if !path.starts_with("pack") {
                path = format!("assets/{}", path);
            }

            if let Err(err) = zip.start_file(&path, Default::default()) {
                error!("Failed to start asset: {err}");
                continue;
            }

            let url = format!(
                "https://resources.download.minecraft.net/{}/{}",
                object.hash.chars().take(2).collect::<String>(),
                object.hash
            );
            trace!("Downloading {path} from {url}");

            let mut response = client.get(&url).send();

            let mut errors = 0;
            while match &response {
                Ok(r) => {
                    if !r.status().is_success() {
                        error!("Error {} for {path}", r.status());
                        true
                    } else {
                        false
                    }
                }
                Err(err) => {
                    warn!("Failed to download {path}: {err}");
                    true
                }
            } {
                errors += 1;
                if errors >= 5 {
                    error!("Skipping {path}");
                    continue 'asset_loop;
                }

                response = client.get(&url).send();
            }

            if let Err(err) = response.unwrap().copy_to(zip) {
                error!("Failed to write asset: {err}");
                continue;
            }

            let new_percent = (index as f32 / asset_index.objects.len() as f32) * 100f32;
            if new_percent - percent >= 10f32 {
                percent = new_percent.floor();
                info!("{percent}% ...");
            }
        }

        Some(())
    }

    fn jar_assets_to_zip(jar: &mut ZipArchive<File>, zip: &mut ZipWriter<File>) -> Option<()> {
        for i in 0..jar.len() {
            let file = match jar.by_index(i) {
                Ok(f) => f,
                Err(err) => {
                    error!("Failed to get file from version jar: {err}");
                    continue;
                }
            };

            let file_name = file.name().to_string();
            if !file_name.starts_with("assets") && !file_name.starts_with("pack") {
                trace!("Skipping file {file_name}");
                continue;
            }

            if let Err(err) = zip.raw_copy_file(file) {
                error!("Failed to copy file {file_name}: {err}");
            }

            trace!("Copied {file_name}");
        }

        Some(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetIndexFile {
    objects: HashMap<String, AssetIndexObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetIndexObject {
    hash: String,
    size: u64,
}
