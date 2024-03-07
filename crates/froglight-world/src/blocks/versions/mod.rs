use froglight_protocol::versions::{v1_20_0::V1_20_0, v1_20_2::V1_20_2, v1_20_3::V1_20_3};
use rangemap::RangeMap;

use super::registry::BlockRegistryInner;

pub(crate) mod v1_20_0;
pub(crate) mod v1_20_2;
pub(crate) mod v1_20_3;

impl Default for BlockRegistryInner<V1_20_0> {
    fn default() -> Self {
        // Create the registry
        let mut registry = Self {
            blocks: Vec::new(),
            block_states: RangeMap::new(),
            _version: std::marker::PhantomData,
        };

        // Register the vanilla blocks
        crate::blocks::versions::v1_20_0::register(&mut registry);

        registry
    }
}

impl Default for BlockRegistryInner<V1_20_2> {
    fn default() -> Self {
        // Create the registry
        let mut registry = Self {
            blocks: Vec::new(),
            block_states: RangeMap::new(),
            _version: std::marker::PhantomData,
        };

        // Register the vanilla blocks
        crate::blocks::versions::v1_20_2::register(&mut registry);

        registry
    }
}

impl Default for BlockRegistryInner<V1_20_3> {
    fn default() -> Self {
        // Create the registry
        let mut registry = Self {
            blocks: Vec::new(),
            block_states: RangeMap::new(),
            _version: std::marker::PhantomData,
        };

        // Register the vanilla blocks
        crate::blocks::versions::v1_20_3::register(&mut registry);

        registry
    }
}
