#![doc = include_str!("../README.md")]
#![no_std]

#[cfg(feature = "network")]
pub use froglight_api as api;
pub use froglight_biome as biome;
pub use froglight_block as block;
pub use froglight_common as common;
pub use froglight_entity as entity;
pub use froglight_inventory as inventory;
pub use froglight_math as math;
#[cfg(feature = "network")]
pub use froglight_network as network;
pub use froglight_packet as packet;
pub use froglight_physics as physics;
pub use froglight_player as player;
pub use froglight_registry as registry;
pub use froglight_world as world;

#[cfg(feature = "bevy")]
mod bevy;
#[cfg(feature = "bevy")]
pub use bevy::*;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    #[cfg(feature = "bevy")]
    pub use crate::plugins::FroglightPlugins;
    #[cfg(feature = "network")]
    pub use crate::{api::prelude::*, network::prelude::*};
    pub use crate::{
        biome::prelude::*, block::prelude::*, common::prelude::*, entity::prelude::*,
        inventory::prelude::*, math::prelude::*, packet::prelude::*, physics::prelude::*,
        player::prelude::*, registry::prelude::*, world::prelude::*,
    };
}
