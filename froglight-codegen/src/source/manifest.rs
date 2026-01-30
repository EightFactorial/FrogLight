use chrono::{DateTime, Utc};
use facet::Facet;
use miette::Result;
use tokio::sync::OnceCell;

use crate::common::{CACHE_DIR, REQWEST, Version};

/// A version manifest, containing information about all available versions.
#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct Manifest {
    pub latest: ManifestLatest,
    pub versions: Vec<ManifestVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct ManifestLatest {
    pub release: Version,
}

#[derive(Debug, Clone, PartialEq, Eq, Facet)]
pub struct ManifestVersion {
    pub id: Version,
    pub url: String,
    #[facet(rename = "releaseTime")]
    pub release_time: DateTime<Utc>,
}

impl Manifest {
    /// Get the global [`Manifest`].
    pub async fn get() -> &'static Manifest {
        static MANIFEST: OnceCell<Manifest> = OnceCell::const_new();

        MANIFEST
            .get_or_init(|| async {
                tracing::info!("Fetching `Manifest`");
                Self::fetch().await.unwrap()
            })
            .await
    }

    /// Fetch the [`Manifest`] from the cache or network.
    pub async fn fetch() -> Result<Manifest> {
        const URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

        let mut path = CACHE_DIR.clone();
        path.push("version_manifest_v2.json");

        let file = if path.exists() {
            tracing::debug!("Using cached `Manifest` at {}", path.display());

            // Read from cache
            match tokio::fs::read_to_string(path).await {
                Ok(content) => content,
                Err(_err) => todo!(),
            }
        } else {
            tracing::debug!("Downloading `Manifest` from \"{URL}\"");

            // Fetch from network
            let response = match REQWEST.get(URL).send().await {
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
            tracing::debug!("Caching `Manifest` at \"{}\"", path.display());
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

    /// Get the [`ManifestVersion`] for the given [`Version`], if it exists.
    #[must_use]
    pub fn version(&self, version: &Version) -> Option<&ManifestVersion> {
        self.versions.iter().find(|v| &v.id == version)
    }
}
