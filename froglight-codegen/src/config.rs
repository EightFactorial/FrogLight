use facet::Facet;
use miette::{Result, bail};

use crate::common::{Version, WORKSPACE_DIR};

#[derive(Debug, Facet)]
pub struct ConfigBundle {
    #[facet(rename = "version")]
    pub versions: Vec<VersionPair>,
}

#[derive(Debug, Facet)]
pub struct VersionPair {
    pub base: Version,
    pub real: Version,
}

pub async fn load() -> Result<ConfigBundle> {
    let path = WORKSPACE_DIR.join("codegen.toml");
    let content = match tokio::fs::read_to_string(&path).await {
        Ok(content) => content,
        Err(err) => bail!("Failed to open \"{}\", {err}", path.display()),
    };
    match facet_toml::from_str::<ConfigBundle>(&content) {
        Ok(config) => {
            tracing::debug!("`ConfigBundle`: {config:?}");
            Ok(config)
        }
        Err(err) => bail!("Failed to parse \"codegen.toml\", {err}"),
    }
}
