#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use froglight_block as block;
#[cfg(feature = "brigadier")]
pub use froglight_brigadier as brigadier;
pub use froglight_common as common;
pub use froglight_item as item;
pub use froglight_nbt as nbt;
pub use froglight_network as network;
pub use froglight_registry as registry;
pub use froglight_world as world;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_block::prelude::*;
    #[cfg(feature = "brigadier")]
    pub use froglight_brigadier::prelude::*;
    pub use froglight_common::prelude::*;
    pub use froglight_nbt::prelude::*;
    pub use froglight_network::prelude::*;
    pub use froglight_registry::prelude::*;
    pub use froglight_world::prelude::*;
}
