//! TODO

use froglight_common::version::Version;
pub use froglight_registry_template::{types::LazyLock, version_implement};

use crate::storage::BiomeStorage;

froglight_registry_template::version_subtrait! {
    pub trait BiomeVersion {
        const BIOMES: BiomeStorage;
        fn biomes();
        fn new_biomes();
    }
}
