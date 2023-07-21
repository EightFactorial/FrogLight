use json::JsonValue;
use log::{error, info};
use strum::IntoEnumIterator;

use crate::{
    extract::datasets::Dataset,
    types::{ClassMap, Manifest, Version},
};

use self::datasets::Datasets;

pub mod datasets;

/// Extract data from the given version
pub fn extract_data(
    version: Version,
    manifest: Manifest,
    datasets: Option<Vec<Datasets>>,
) -> Option<JsonValue> {
    info!("Extracting data for version {}", version);

    // Read jar
    let map = match ClassMap::new_mapped(&version, &manifest) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return None;
        }
    };

    // Resolve datasets
    let mut datasets = datasets.unwrap_or_else(|| Datasets::iter().collect());
    filter_datasets(&mut datasets);

    // Iterate over datasets
    let mut data = JsonValue::new_object();
    for dataset in datasets {
        dataset.parse(&version, &manifest, &map, &mut data);
    }

    Some(data)
}

/// Add dependencies and deduplicate
///
/// Some datasets depend on other datasets, so we need to run those first.
fn filter_datasets(datasets: &mut Vec<Datasets>) {
    // Add dependencies
    let mut i = 0;
    while i < datasets.len() {
        let dataset = datasets[i];
        for dep in dataset.deps() {
            if !datasets.contains(dep) {
                datasets.push(*dep);
            }
        }
        i += 1;
    }

    // Deduplicate
    datasets.dedup();

    #[cfg(debug_assertions)]
    {
        log::debug!("Datasets: {:?}", datasets);
    }
}
