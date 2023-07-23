use chrono::Utc;
use json::JsonValue;

use crate::types::{ClassMap, Manifest, Version};

use super::{Dataset, Datasets};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Diagnostics;

impl Dataset for Diagnostics {
    fn min(&self) -> &'static Option<Version> { &None }

    fn deps(&self) -> &'static [Datasets] { &[] }

    fn parse(
        &self,
        _version: &Version,
        _manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    ) {
        let json = json::object! {
            "version": option_env!("CARGO_PKG_VERSION"),
            "date": Utc::now().date_naive().to_string(),
            "classes": classmap.len(),
        };

        data["diagnostics"] = json;
    }
}
