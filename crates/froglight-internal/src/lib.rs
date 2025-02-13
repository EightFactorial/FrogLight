#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use froglight_block as block;
pub use froglight_common as common;
pub use froglight_network as network;
pub use froglight_registry as registry;
pub use froglight_world as world;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_block::prelude::*;
    pub use froglight_common::{EntityId, EntityUuid, Identifier, version::*};
    pub use froglight_network::prelude::*;
    // pub use froglight_registry::prelude::*;
    pub use froglight_world::prelude::*;
}
