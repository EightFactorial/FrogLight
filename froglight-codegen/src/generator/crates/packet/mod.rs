use miette::Result;
use tokio::sync::RwLock;

use crate::{common::DATA, config::ConfigBundle};

mod data;
pub use data::PacketData;

/// Generate packets for all [`Version`]s.
pub async fn generate_global(config: &ConfigBundle) -> Result<()> {
    let guard = DATA.owned_guard();

    for version in &config.versions {
        let storage = DATA.get_or_insert_with(version.real.clone(), RwLock::default, &guard);
        let mut storage = storage.write().await;

        PacketData::get_for(&version.real, &mut storage, async |data| {
            tracing::info!("{data:#?}");
            Ok(())
        })
        .await?;
    }

    Ok(())
}
