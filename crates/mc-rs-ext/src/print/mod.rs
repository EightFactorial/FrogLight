use std::{fs::File, io::Write};

use log::{debug, error, info};

use crate::types::{apply_mappings, create_hashmap, Manifest, Version};

pub fn print_data(version: Version, _manifest: Manifest, path: Option<String>, class: String) {
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

    if let Some(class) = map.get(&class) {
        if let Some(path) = path {
            info!("Writing to {}", path);
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(err) => {
                    error!("Failed to create file: {}", err);
                    return;
                }
            };

            file.write_all(format!("{:#?}", class).as_bytes()).unwrap();
        } else {
            println!("{:#?}", class);
        }
    } else {
        error!("Class {} not found!", class);
    }
}
