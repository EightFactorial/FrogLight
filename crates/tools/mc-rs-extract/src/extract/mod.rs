use itertools::Itertools;
use json::JsonValue;
use strum::IntoEnumIterator;
use tracing::{error, info, warn};

use crate::{
    extract::datasets::Dataset,
    types::{ClassMap, Manifest, Version},
};

use self::datasets::Datasets;

pub mod datasets;

/// Extract data from the given version
pub fn extract_data(
    version: &Version,
    manifest: &Manifest,
    datasets: Option<Vec<Datasets>>,
    unstable: bool,
) -> Option<JsonValue> {
    info!("Extracting data for version {}", version);

    // Get jar data
    let map = match ClassMap::new_with_mappings(version, manifest) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return None;
        }
    };

    // Prepare datasets
    let datasets = get_datasets(datasets, version, manifest, unstable);

    // Collect data
    let mut data = JsonValue::new_object();
    for dataset in datasets {
        dataset.parse(version, manifest, &map, &mut data);
    }

    Some(data)
}

/// Get the list of datasets to extract
///
/// This will filter out unsupported datasets and add dependencies.
fn get_datasets(
    datasets: Option<Vec<Datasets>>,
    version: &Version,
    manifest: &Manifest,
    unstable: bool,
) -> Vec<Datasets> {
    // Get the list of sets
    let mut manual = false;
    let mut datasets = if let Some(sets) = datasets {
        manual = true;
        sets.into_iter().unique().collect_vec()
    } else {
        Datasets::iter().collect_vec()
    };

    // If unstable flag is not set, filter out unsupported sets
    if !unstable {
        let mut filtered = false;
        datasets.retain(|d| {
            let supported = if let Some(ver) = d.min() {
                version.is_same_or_newer(ver, manifest)
            } else {
                true
            };

            if !supported && manual {
                warn!("Dataset {:?} is not supported for version {}", d, version);
                filtered = true;
            }

            supported
        });
        if filtered && manual {
            warn!("Use -u or --unstable to forcefully extract unsupported datasets");
        }
    }

    add_deps(&mut datasets);

    #[cfg(debug_assertions)]
    {
        tracing::debug!("Datasets: {:?}", datasets);
    }

    datasets
}

/// Recursively add dependencies before the sets that depend on them
fn add_deps(datasets: &mut Vec<Datasets>) {
    let mut added = false;
    for (index, set) in datasets.clone().iter().enumerate().rev() {
        for dep in set.deps() {
            if let Some(pos) = datasets.iter().position(|s| s == dep) {
                if pos > index {
                    datasets.remove(pos);
                    datasets.insert(index, *dep);
                    added = true;
                }
            } else {
                datasets.insert(index, *dep);
                added = true;
            }
        }
    }

    if added {
        add_deps(datasets);
    }
}
