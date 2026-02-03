use miette::Result;

use crate::{
    common::Version,
    config::{ConfigBundle, VersionPair},
};

pub struct VersionHelper;

impl VersionHelper {
    /// Run the given async function for all [`Version`]s in the
    /// [`ConfigBundle`].
    ///
    /// ## Warning
    ///
    /// May deadlock if the current function holds a lock that is needed inside
    /// the provided function.
    pub async fn for_all<F: AsyncFnMut(&VersionPair) -> Result<R>, R>(
        config: &ConfigBundle,
        mut f: F,
    ) -> Result<Vec<R>> {
        let mut output = Vec::with_capacity(config.versions.len());

        for versions in &config.versions {
            output.push((f)(versions).await?);
        }

        Ok(output)
    }
}
