use miette::Result;

use crate::{
    config::{ConfigBundle, VersionPair},
    source::JarData,
};

pub mod cargo_toml;
pub mod version_type;

pub async fn generate(
    VersionPair { base: _base, real }: &VersionPair,
    _config: &ConfigBundle,
) -> Result<()> {
    JarData::get_for(real, async |_data| Ok(())).await
}
