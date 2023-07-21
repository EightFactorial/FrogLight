use std::{fs::File, path::PathBuf};

use log::error;

use crate::types::{Manifest, Version, VersionData};

pub fn minecraft_jar(ver: &Version, man: &Manifest) -> Option<PathBuf> {
    let mut path = crate::util::minecraft_dir()?;

    path.push(format!("versions/{}/", ver));

    if !path.exists() {
        match std::fs::create_dir_all(&path) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to create directory {}: {}", path.display(), err);
                return None;
            }
        }
    }

    let path_jar = path.join(format!("{}.jar", ver));
    if !path_jar.exists() {
        let path_data = path.join(format!("{}.json", ver));

        // Check for the version data
        if !path_data.exists() {
            let man_ver = man.versions.iter().find(|v| v.id == *ver)?;

            let mut response = match reqwest::blocking::get(&man_ver.url) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to fetch version data: {}", e);
                    return None;
                }
            };

            let mut file = match File::create(&path_data) {
                Ok(f) => f,
                Err(err) => {
                    error!(
                        "Failed to create version data file {}: {}",
                        path_data.display(),
                        err
                    );
                    return None;
                }
            };
            response.copy_to(&mut file).ok()?;
            drop(file);
        }

        // Read the version data
        let mut file = match File::open(&path_data) {
            Ok(f) => f,
            Err(err) => {
                error!(
                    "Failed to read version data {}: {}",
                    path_data.display(),
                    err
                );
                return None;
            }
        };

        // Get the jar from the version data
        let ver_data: VersionData = serde_json::from_reader(&mut file).ok()?;
        let mut response = match reqwest::blocking::get(ver_data.downloads.client.url) {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to fetch jar: {}", e);
                return None;
            }
        };

        // Write the jar
        let mut file = match File::create(&path_jar) {
            Ok(f) => f,
            Err(err) => {
                error!("Failed to create file {}: {}", path_jar.display(), err);
                return None;
            }
        };
        response.copy_to(&mut file).ok()?;
    }

    Some(path_jar)
}
