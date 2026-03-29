use facet::Facet;
use indexmap::IndexMap;
use miette::Result;

use crate::{
    common::{CACHE_DIR, REQWEST, Version, VersionStorage},
    source::VersionData,
};

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct VersionAssets {
    pub objects: IndexMap<String, AssetContent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct AssetContent {
    pub hash: String,
    pub size: u64,
}

impl AssetContent {
    /// Get the URL for this asset.
    pub fn url(&self) -> String {
        format!("https://resources.download.minecraft.net/{}/{}", &self.hash[0..2], self.hash)
    }
}

impl VersionAssets {
    /// Get the [`VersionAssets`] for the given [`Version`], fetching it if
    /// necessary.
    pub async fn get_for<F: AsyncFnOnce(&Self) -> Result<V>, V: 'static>(
        version: &Version,
        storage: &mut VersionStorage,
        f: F,
    ) -> Result<V> {
        if !storage.contains::<Self>() {
            tracing::info!("Fetching `VersionAssets` for \"{}\"", version.as_str());
            let data = Self::fetch(version, storage).await?;
            storage.insert(data);
        }

        f(storage.get::<Self>().unwrap()).await
    }

    /// Fetch the [`VersionAssets`] for the given [`Version`].
    pub async fn fetch(version: &Version, storage: &mut VersionStorage) -> Result<Self> {
        let mut path = CACHE_DIR.clone();
        path.push(version.as_feature());
        path.push("assets.json");

        let file = if path.exists() {
            tracing::debug!("Using cached `VersionAssets` at {}", path.display());

            // Read from cache
            match tokio::fs::read_to_string(path).await {
                Ok(content) => content,
                Err(_err) => todo!(),
            }
        } else {
            tracing::debug!("Downloading `VersionAssets` for \"{}\"", version.as_str());

            VersionData::get_for(version, storage, async |data| {
                // Fetch from network
                let response = match REQWEST.get(&data.asset_index.url).send().await {
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
                tracing::debug!("Caching `VersionAssets` at \"{}\"", path.display());
                if let Err(_err) = tokio::fs::write(path, &content).await {
                    todo!()
                }

                Ok(content)
            })
            .await?
        };

        match facet_json::from_str::<Self>(&file) {
            Ok(manifest) => Ok(manifest),
            Err(_err) => todo!(),
        }
    }
}
