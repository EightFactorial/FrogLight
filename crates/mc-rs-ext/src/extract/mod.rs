use itertools::Itertools;
use json::JsonValue;
use log::{error, info, warn};
use strum::IntoEnumIterator;

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

    // Read jar
    let map = match ClassMap::new_mapped(version, manifest) {
        Ok(m) => m,
        Err(err) => {
            error!("Failed to read jar: {}", err);
            return None;
        }
    };

    // Prepare datasets
    let mut datasets = datasets.unwrap_or_else(|| Datasets::iter().collect());
    add_dependencies(&mut datasets);
    datasets = datasets.into_iter().unique().collect();

    // Filter and warn about unsupported datasets
    let mut filtered = false;
    if !unstable {
        datasets.retain(|d| {
            let supported = if let Some(min) = d.min() {
                version.is_same_or_newer(min, manifest)
            } else {
                true
            };

            if !supported {
                warn!("Dataset {} is not supported in version {}", d, version);
                filtered = true;
            }
            supported
        });
    }
    if filtered {
        warn!("Some datasets were filtered out, use --unstable to include them");
    }

    #[cfg(debug_assertions)]
    {
        log::debug!("Datasets: {:?}", datasets);
    }

    // Collect data
    let mut data = JsonValue::new_object();
    for dataset in datasets {
        dataset.parse(version, manifest, &map, &mut data);
    }

    Some(data)
}

/// Add dependencies
///
/// Some datasets depend on other datasets, so we need to run those first.
fn add_dependencies(datasets: &mut Vec<Datasets>) {
    for (index, set) in datasets.clone().iter().enumerate().rev() {
        for dep in set.deps() {
            if let Some(pos) = datasets.iter().position(|s| s == dep) {
                if pos > index {
                    datasets.remove(pos);
                    datasets.insert(index, *dep);
                }
            } else {
                datasets.insert(index, *dep);
            }
        }
    }
}
