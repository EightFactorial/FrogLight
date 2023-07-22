use log::{error, info};

use crate::types::{ClassMap, Manifest, Version};

pub fn print_data(version: &Version, manifest: &Manifest, class: String) -> Option<String> {
    info!("Printing class from version {}", version);
    let map = match ClassMap::new_mapped(version, manifest) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return None;
        }
    };

    if &class == "*" {
        Some(format!("{:#?}", map))
    } else if let Some(class) = map.get(&class) {
        Some(format!("{:#?}", class))
    } else {
        error!("Class {} not found!", class);
        None
    }
}
