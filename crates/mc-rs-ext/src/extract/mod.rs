// use std::{fs::File, io::Write};

use log::{error, info};

use crate::types::{ClassMap, Manifest, Version};

pub fn extract_data(version: Version, manifest: Manifest, _path: Option<String>) {
    info!("Extracting data for version {}", version);
    let _map = match ClassMap::new_mapped(&version, &manifest) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return;
        }
    };
}
