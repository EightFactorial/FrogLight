use std::{io::Cursor, path::Path};

use miette::Result;
use tokio::fs::{File, OpenOptions};

use crate::{common::WORKSPACE_DIR, config::ConfigBundle};

pub struct CargoHelper;

impl CargoHelper {
    /// The list of crates to manage and features to enable.
    const CRATES: &'static [(&'static str, &[&'static str])] = &[
        ("froglight", &["froglight-internal/"]),
        ("froglight-biome", &[]),
        ("froglight-block", &[]),
        ("froglight-common", &[]),
        (
            "froglight-internal",
            &[
                "froglight-biome/",
                "froglight-block/",
                "froglight-item/",
                "froglight-network?/",
                "froglight-packet/",
                "froglight-registry/",
            ],
        ),
        ("froglight-item", &[]),
        (
            "froglight-network",
            &["froglight-block/", "froglight-item/", "froglight-packet/", "froglight-registry/"],
        ),
        ("froglight-packet", &[]),
        ("froglight-registry", &[]),
    ];

    /// Generate `Cargo.toml` file feature sets for enabled versions.
    pub async fn generate(config: &ConfigBundle) -> Result<()> {
        for (crate_name, features) in Self::CRATES {
            let path = WORKSPACE_DIR.join(crate_name).join("Cargo.toml");
            Self::update_file(&path, features, config).await?;
        }
        Ok(())
    }

    /// Update a single `Cargo.toml` file with the given features.
    async fn update_file(path: &Path, features: &[&str], config: &ConfigBundle) -> Result<()> {
        let input = match tokio::fs::read_to_string(path).await {
            Ok(data) => data,
            Err(err) => miette::bail!("Failed to read Cargo.toml at \"{}\", {err}", path.display()),
        };

        // Find the last occurrence of the @generated marker.
        let Some(marker) = input.lines().position(|l| l.contains("@generated")) else {
            tracing::error!("No @generated marker found in Cargo.toml at \"{}\"", path.display());
            return Ok(());
        };

        // Generate a new set of features after the marker.
        let mut content = input.lines().take(marker + 1).collect::<Vec<&str>>().join("\n");
        content.push('\n');
        content.push('\n');

        for pair in &config.versions {
            let version_feature = pair.base.as_feature();

            content.push_str(&version_feature);
            content.push_str(" = [");
            for (i, feature) in features.iter().enumerate() {
                content.push('\"');
                content.push_str(feature);
                content.push_str(&version_feature);
                content.push('\"');
                if i + 1 != features.len() {
                    content.push_str(", ");
                }
            }
            content.push_str("]\n");
        }

        // Write the updated content back to the Cargo.toml file.
        match tokio::fs::write(path, content).await {
            Ok(()) => Ok(()),
            Err(err) => {
                miette::bail!("Failed to write updated Cargo.toml at \"{}\", {err}", path.display())
            }
        }
    }
}
