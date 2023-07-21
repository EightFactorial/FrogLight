// use std::{fs::File, io::Write};

use log::{debug, error, info};

use crate::types::{apply_mappings, create_hashmap, Manifest, Version};

pub fn extract_data(version: Version, _manifest: Manifest, _path: Option<String>) {
    info!("Extracting data for version {}", version);
    let map = match create_hashmap(&version) {
        Ok(m) => match apply_mappings(m, &version) {
            Ok(m) => m,
            Err(err) => {
                error!("Failed to apply mappings: {}", err);
                return;
            }
        },
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return;
        }
    };
    debug!("Found {} classes", map.len());
}
