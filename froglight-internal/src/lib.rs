#![doc = include_str!("../README.md")]
#![no_std]

pub use froglight_biome as biome;
pub use froglight_block as block;
pub use froglight_common as common;
pub use froglight_math as math;
pub use froglight_registry as registry;
pub use froglight_world as world;

#[cfg(feature = "bevy")]
pub mod plugins {
    //! Re-exports of all provided bevy plugins.

    pub use crate::{common::bevy::CommonPlugin, world::bevy::WorldPlugin};
}

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        biome::prelude::*, block::prelude::*, common::prelude::*, math::prelude::*,
        registry::prelude::*, world::prelude::*,
    };
}
