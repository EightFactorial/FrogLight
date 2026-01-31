use miette::Result;
use tokio::sync::RwLock;

use crate::{
    common::DATA,
    config::{ConfigBundle, VersionPair},
};

pub mod cargo_toml;
pub mod crates;
pub mod version_type;

pub async fn generate(version: &VersionPair, _config: &ConfigBundle) -> Result<()> {
    let pinned = DATA.pin_owned();
    let storage_lock = pinned.get_or_insert_with(version.real.clone(), RwLock::default);
    let mut storage = storage_lock.write().await;

    crates::biome::generate(version, &mut storage).await?;
    crates::block::generate(version, &mut storage).await?;
    // crates::item::generate(version, &mut storage).await?;
    // crates::registry::generate(version, &mut storage).await?;

    Ok(())
}
