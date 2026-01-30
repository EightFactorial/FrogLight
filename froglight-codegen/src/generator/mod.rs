use miette::Result;

use crate::config::{ConfigBundle, VersionPair};

pub async fn generate(
    VersionPair { base: _base, real: _real }: &VersionPair,
    _config: &ConfigBundle,
) -> Result<()> {
    Ok(())
}
