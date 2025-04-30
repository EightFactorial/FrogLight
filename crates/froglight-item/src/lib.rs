#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(const_type_id)]
#![no_std]

extern crate alloc;

pub mod generated;
pub mod item;
pub mod resolve;
pub mod storage;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use froglight_macros::StaticItem;

    pub use crate::{
        generated::{item, property},
        item::{Item, ItemRarity, ItemType, ItemTypeExt, StaticItem, UntypedItem},
        storage::{AppItemStorage, GlobalItemId, ItemStorage},
    };
}
