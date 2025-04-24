#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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
