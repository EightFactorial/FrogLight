use std::{fs::File, path::PathBuf};

use crate::{
    types::{MappingsError, Version},
    util::minecraft_dir,
};

const MAPPINGS_URL: &str =
    "https://maven.fabricmc.net/net/fabricmc/intermediary/{VER}/intermediary-{VER}-v2.jar";

pub fn get_mappings(ver: &Version) -> Result<PathBuf, MappingsError> {
    let mut path = minecraft_dir().ok_or(MappingsError::PathNotFound)?;
    path.push(format!("versions/{}/intermediary-{}-v2.jar", ver, ver));

    if !path.exists() {
        let url = MAPPINGS_URL.replace("{VER}", &ver.to_string());
        let mut resp = reqwest::blocking::get(url)?;
        resp.copy_to(&mut File::create(&path)?)?;
    }

    Ok(path)
}

const MAPPER_VERSION: &str = "0.8.7";
const MAPPER_URL: &str =
    "https://maven.fabricmc.net/net/fabricmc/tiny-remapper/{VER}/tiny-remapper-{VER}-fat.jar";

pub fn get_tiny_remapper() -> Result<PathBuf, MappingsError> {
    let mut path = minecraft_dir().ok_or(MappingsError::PathNotFound)?;
    path.push("versions/tiny-remapper.jar");

    if !path.exists() {
        let url = MAPPER_URL.replace("{VER}", MAPPER_VERSION);
        let mut resp = reqwest::blocking::get(url)?;
        resp.copy_to(&mut File::create(&path)?)?;
    }

    Ok(path)
}
