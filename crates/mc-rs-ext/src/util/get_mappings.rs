use std::{fs::File, path::PathBuf};

use crate::{
    types::{MappingsError, Version},
    util::minecraft_dir,
};

const MAPPINGS_URL: &str =
    "https://maven.fabricmc.net/net/fabricmc/intermediary/VER/intermediary-VER-v2.jar";

pub fn get_mappings(ver: &Version) -> Result<PathBuf, MappingsError> {
    let mut path = minecraft_dir().ok_or(MappingsError::McDirNotFound)?;
    path.push(format!("versions/{}/intermediary-{}-v2.jar", ver, ver));

    if !path.exists() {
        let url = MAPPINGS_URL.replace("VER", &ver.to_string());
        let mut resp = reqwest::blocking::get(url).map_err(MappingsError::Http)?;
        resp.copy_to(&mut File::create(&path).map_err(MappingsError::Io)?)
            .map_err(MappingsError::Http)?;
    }

    Ok(path)
}
