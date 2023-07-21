use std::{fs::File, io::Write};

use log::{error, info};

use crate::types::{ClassMap, Manifest, Version};

pub fn print_data(version: Version, _manifest: Manifest, path: Option<String>, class: String) {
    info!("Extracting data for version {}", version);
    let map = match ClassMap::new_mapped(&version) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return;
        }
    };

    let output = if &class == "*" {
        format!("{:#?}", map)
    } else if let Some(class) = map.get(&class) {
        format!("{:#?}", class)
    } else {
        error!("Class {} not found!", class);
        return;
    };

    if let Some(path) = path {
        info!("Writing to {}", path);
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(err) => {
                error!("Failed to create file: {}", err);
                return;
            }
        };

        file.write_all(output.as_bytes()).unwrap();
    } else {
        info!("Writing to console");
        println!("{:#?}", output);
    }
}
