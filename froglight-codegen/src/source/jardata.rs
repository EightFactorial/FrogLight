use dashmap::Entry;
use miette::Result;

use crate::common::{DATA, Version, VersionData};

pub struct JarData {}

impl JarData {
    /// Get the [`JarData`] for the given [`Version`], fetching it if necessary.
    pub async fn get<F: FnOnce(&Self) -> Fut, Fut: Future<Output = Result<V>>, V>(
        version: &Version,
        f: F,
    ) -> Result<V> {
        let mut version_data = {
            if !DATA.contains_key(version) {
                DATA.insert(version.clone(), VersionData::default());
            }
            DATA.get(version).unwrap()
        };

        let jar_data = {
            if !version_data.contains::<Self>() {
                drop(version_data);
                let jardata = Self::fetch(version).await?;
                DATA.get_mut(version).unwrap().insert(jardata);
                version_data = DATA.get(version).unwrap();
            }
            version_data.get::<Self>().unwrap()
        };

        f(jar_data).await
    }

    /// Fetch the [`JarData`] for the given [`Version`].
    pub async fn fetch(_version: &Version) -> Result<Self> { todo!() }
}
