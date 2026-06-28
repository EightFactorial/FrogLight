//! TODO

use froglight_common::version::Version;
pub use froglight_registry_template::{types::LazyLock, version_implement};

use crate::storage::ItemStorage;

froglight_registry_template::version_subtrait! {
    pub trait ItemVersion {
        const ITEMS: ItemStorage;
        fn items();
        fn new_items();
    }
}
