use std::path::PathBuf;

use crate::types::Version;

pub fn minecraft_jar(ver: &Version) -> Option<PathBuf> {
    let mut path = crate::util::minecraft_dir()?;
    path.push(format!("versions/{}/{}.jar", ver, ver));

    // TODO: Download jar if it doesn't exist

    Some(path)
}
