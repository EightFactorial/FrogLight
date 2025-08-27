#![doc = include_str!("../README.md")]
#![allow(unreachable_pub, unused_imports, reason = "Empty modules unused and unreachable")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

pub mod crates {
    //! Re-exports of all sub-crates.

    pub use froglight_api as api;
    pub use froglight_block as block;
    pub use froglight_common as common;
    pub use froglight_entity as entity;
    pub use froglight_item as item;
    pub use froglight_network as network;
    pub use froglight_packet as packet;
    pub use froglight_registry as registry;
    pub use froglight_world as world;
}

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::crates::{
        api::prelude::*, block::prelude::*, common::prelude::*, entity::prelude::*,
        item::prelude::*, network::prelude::*, packet::prelude::*, registry::prelude::*,
        world::prelude::*,
    };
}
