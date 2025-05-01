#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "bevy")]
pub mod bevy;
pub mod inventory;
pub mod slot;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    // #[cfg(feature = "bevy")]
    // pub use crate::bevy::{Inventory, InventoryMut};
    pub use crate::{
        inventory::{EntityEquipment, PlayerInventory, PlayerInventoryMenu},
        slot::InventorySlot,
    };
}
