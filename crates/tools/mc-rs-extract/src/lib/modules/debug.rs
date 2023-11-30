use chrono::Utc;
use tracing::info;

use crate::data::ModuleData;

use super::ModuleExt;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DebugModule;

impl ModuleExt for DebugModule {
    fn run(&self, data: &mut ModuleData) {
        info!("Adding debug info");
        data.output["debug"] = json::object! {
            "date-generated": Utc::now().date_naive().to_string(),
            "extractor-version": env!("CARGO_PKG_VERSION"),
            "total-classes": data.classmap.len(),
        }
    }
}
