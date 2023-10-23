use std::{fs::File, io::read_to_string};

use json::JsonValue;
use log::error;

use crate::{
    types::{ClassMap, Manifest, Version},
    util::minecraft_jar,
};

use super::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Info;

impl Dataset for Info {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        version: &Version,
        manifest: &Manifest,
        _classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let Some(path) = minecraft_jar(version, manifest) else {
            error!("Failed to find jar for version {}", version);
            return;
        };

        let jar = match File::open(&path) {
            Ok(jar) => jar,
            Err(err) => {
                error!("Failed to open jar {}: {}", path.display(), err);
                return;
            }
        };

        let mut zip = match zip::ZipArchive::new(jar) {
            Ok(zip) => zip,
            Err(err) => {
                error!("Failed to open jar {}: {}", path.display(), err);
                return;
            }
        };

        if let Ok(file) = zip.by_name("version.json") {
            let file = read_to_string(file).unwrap();
            let json = json::parse(&file).unwrap();
            data["version"] = json;
        } else {
            error!("Failed to find version.json in jar {}", path.display());
        };
    }
}
