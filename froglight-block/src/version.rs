//! TODO

use froglight_common::version::Version;
pub use froglight_registry_template::version_implement;

use crate::storage::BlockStorage;

froglight_registry_template::version_subtrait! {
    pub trait BlockVersion {
        const BLOCKS: BlockStorage;
        fn blocks();
        fn new_blocks();
    }
}
