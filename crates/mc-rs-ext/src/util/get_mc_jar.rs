use std::{
    fs::File,
    path::PathBuf,
    process::{Command, Stdio},
};

use log::error;
use zip::ZipArchive;

use crate::types::{Manifest, MappingsError, Version, VersionData};

use super::{get_mappings, get_tiny_remapper};

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

pub fn mapped_minecraft_jar(ver: &Version, man: &Manifest) -> Result<PathBuf, MappingsError> {
    let jar_path = minecraft_jar(ver, man).ok_or(MappingsError::JarNotFound)?;
    let mut folder_path = jar_path.clone();
    folder_path.pop();

    let mapped_jar_path = folder_path.join(format!("{}-mapped.jar", ver));
    if !mapped_jar_path.exists() {
        // Check if the mappings file exists
        let mappings_file_path = folder_path.join("mappings.tiny");
        if !mappings_file_path.exists() {
            let mappings_path = get_mappings(ver)?;
            let mut mappings_jar = File::open(mappings_path)?;
            let mut mappings_jar = ZipArchive::new(&mut mappings_jar)?;
            let mut mappings_file = mappings_jar.by_name("mappings/mappings.tiny")?;

            std::io::copy(&mut mappings_file, &mut File::create(&mappings_file_path)?)?;
        }

        // Run the remapper
        let remapper_path = get_tiny_remapper()?;
        let command = Command::new("java")
            .arg("-jar")
            .arg(remapper_path)
            .arg(jar_path)
            .arg(&mapped_jar_path)
            .arg(mappings_file_path)
            .arg("official")
            .arg("intermediary")
            .current_dir(folder_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn();

        match command {
            Ok(mut child) => {
                let status = child.wait()?;
                if !status.success() {
                    return Err(MappingsError::MappingFailed);
                }
            }
            Err(err) => {
                error!("Failed to run java: {}", err);
                return Err(err.into());
            }
        }
    }

    Ok(mapped_jar_path)
}
