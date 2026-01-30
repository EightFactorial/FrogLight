use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use chrono::{DateTime, Utc};
use facet::Facet;
use miette::Result;

use crate::{
    common::{CACHE_DIR, DATA, REQWEST, Version, VersionStorage},
    source::Manifest,
};

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct VersionData {
    pub downloads: VersionDownloads,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct VersionDownloads {
    pub client: DownloadInfo,
    pub server: DownloadInfo,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct DownloadInfo {
    pub url: String,
}

impl VersionData {
    /// Get the [`VersionData`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V: 'static>(
        version: &Version,
        f: F,
    ) -> Result<V> {
        let mut version_data = {
            if !DATA.contains_key(version) {
                DATA.insert(version.clone(), VersionStorage::default());
            }
            DATA.get(version).unwrap()
        };

        let version_json = {
            if !version_data.contains::<Self>() {
                drop(version_data);

                let mut data_mut = DATA.get_mut(version).unwrap();
                if !data_mut.contains::<Self>() {
                    tracing::info!("Fetching `VersionData` for \"{}\"", version.as_str());
                    data_mut.insert(Self::fetch(version).await?);
                }
                drop(data_mut);

                version_data = DATA.get(version).unwrap();
            }
            version_data.get::<Self>().unwrap()
        };

        f(version_json).await
    }

    /// Fetch the [`VersionData`] for the given [`Version`].
    pub async fn fetch(version: &Version) -> Result<Self> {
        let mut path = CACHE_DIR.clone();
        let json = format!("{}.json", version.as_str());
        path.push(version.as_feature());
        path.push(&json);

        let file = if path.exists() {
            tracing::debug!("Using cached `VersionData` at {}", path.display());

            // Read from cache
            match tokio::fs::read_to_string(path).await {
                Ok(content) => content,
                Err(_err) => todo!(),
            }
        } else {
            let manifest = Manifest::get().await;
            let Some(manifest) = manifest.version(version) else { todo!() };

            tracing::debug!("Downloading `VersionData` from \"{}\"", &manifest.url);

            // Fetch from network
            let response = match REQWEST.get(&manifest.url).send().await {
                Ok(response) => response,
                Err(_err) => todo!(),
            };

            let content = match response.text().await {
                Ok(content) => content,
                Err(_err) => todo!(),
            };

            // Ensure parent directory exists
            if let Some(parent) = path.parent()
                && !parent.exists()
                && let Err(_err) = tokio::fs::create_dir_all(parent).await
            {
                todo!()
            }

            // Write to cache
            tracing::debug!("Caching `VersionData` at \"{}\"", path.display());
            if let Err(_err) = tokio::fs::write(path, &content).await {
                todo!()
            }

            content
        };

        match facet_json::from_str::<Self>(&file) {
            Ok(manifest) => {
                tracing::trace!("VersionData: {manifest:?}");
                Ok(manifest)
            }
            Err(_err) => todo!(),
        }
    }
}
