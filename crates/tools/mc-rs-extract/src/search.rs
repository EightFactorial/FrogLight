use tracing::{error, info};

use crate::types::{ClassMap, Manifest, Version};

pub fn search_data(version: &Version, manifest: &Manifest, query: String) -> Option<String> {
    info!("Searching for data from version {}", version);
    let map = match ClassMap::new_with_mappings(version, manifest) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return None;
        }
    };

    let mut vec = Vec::new();
    for class in map.values() {
        let string = format!("{:#?}", class);
        if string.contains(&query) {
            vec.push(string);
        }
    }

    Some(vec.join("\n"))
}
