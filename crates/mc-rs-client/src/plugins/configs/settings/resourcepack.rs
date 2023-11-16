use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePackSettings {
    #[serde(default = "ResourcePackSettings::default_resourcepacks")]
    pub paths: Vec<CompactString>,
}

impl Default for ResourcePackSettings {
    fn default() -> Self {
        Self {
            paths: Self::default_resourcepacks(),
        }
    }
}

impl ResourcePackSettings {
    fn default_resourcepacks() -> Vec<CompactString> {
        vec![CompactString::new_inline("minecraft.jar")]
    }
}
