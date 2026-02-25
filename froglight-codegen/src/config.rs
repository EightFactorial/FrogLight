use facet::Facet;
use tokio::sync::OnceCell;

use crate::common::{Version, WORKSPACE_DIR};

#[derive(Debug, Default, Clone, PartialEq, Eq, Facet)]
pub struct ConfigBundle {
    #[facet(rename = "version")]
    pub versions: Vec<VersionPair>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Facet)]
pub struct VersionPair {
    /// The name to refer to this version as.
    pub base: Version,
    /// The real version data is sourced from.
    pub real: Version,
}

impl ConfigBundle {
    pub async fn load() -> &'static Self {
        static CONFIG: OnceCell<ConfigBundle> = OnceCell::const_new();

        CONFIG
            .get_or_init(|| async {
                let path = WORKSPACE_DIR.join("codegen.toml");
                let content = match tokio::fs::read_to_string(&path).await {
                    Ok(content) => content,
                    Err(err) => panic!("Failed to open \"{}\", {err}", path.display()),
                };
                match facet_toml::from_str::<ConfigBundle>(&content) {
                    Ok(config) => {
                        tracing::trace!("{config:?}");
                        config
                    }
                    Err(err) => {
                        panic!("Failed to parse \"{}\", {err}", path.display());
                    }
                }
            })
            .await
    }
}
