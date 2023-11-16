use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePackSettings {
    #[serde(default = "ResourcePackSettings::default_paths")]
    pub paths: Vec<CompactString>,
}

impl Default for ResourcePackSettings {
    fn default() -> Self {
        Self {
            paths: Self::default_paths(),
        }
    }
}

impl ResourcePackSettings {
    fn default_paths() -> Vec<CompactString> { vec![CompactString::new_inline("minecraft.jar")] }
}
