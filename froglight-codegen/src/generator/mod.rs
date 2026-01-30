use miette::Result;
use tokio::sync::RwLock;

use crate::{
    common::DATA,
    config::{ConfigBundle, VersionPair},
    source::JarData,
};

pub mod cargo_toml;
pub mod version_type;

pub async fn generate(
    VersionPair { base: _base, real }: &VersionPair,
    _config: &ConfigBundle,
) -> Result<()> {
    let pinned = DATA.pin_owned();
    let storage_lock = pinned.get_or_insert_with(real.clone(), RwLock::default);

    let mut storage = storage_lock.write().await;
    JarData::get_for(real, &mut storage, async |_data| Ok(())).await
}
