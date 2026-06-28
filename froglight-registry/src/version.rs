//! TODO

use froglight_common::version::Version;
pub use froglight_registry_template::{types::LazyLock, version_implement};

use crate::storage::RegistryStorage;

froglight_registry_template::version_subtrait! {
    pub trait RegistryVersion {
        const REGISTRY: RegistryStorage;
        fn registry();
        fn new_registry();
    }
}
