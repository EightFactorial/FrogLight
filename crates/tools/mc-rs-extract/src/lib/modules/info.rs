use std::io::Read;

use tracing::{error, info, warn};

use crate::data::ModuleData;

use super::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct InfoModule;

impl ModuleExt for InfoModule {
    fn run(&self, data: &mut ModuleData) {
        let Ok(mut zip_file) = data.zip.by_name("version.json") else {
            warn!("Could not find version.json in jar file");
            return;
        };

        let mut file_contents = String::new();
        if let Err(err) = zip_file.read_to_string(&mut file_contents) {
            error!("Could not read version.json: {err}");
            return;
        }

        match json::parse(&file_contents) {
            Err(err) => error!("Could not parse version.json: {err}"),
            Ok(parsed) => {
                info!("Copying data from version.json");
                data.output["version"] = parsed;
            }
        }
    }
}
