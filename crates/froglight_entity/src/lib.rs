#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

pub mod entity;
pub mod generated;
pub mod info;
pub mod storage;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    pub use crate::{
        entity::{Entity, GlobalEntityId},
        storage::Entities,
    };

    pub mod entities {
        //! Re-exports of all entity types, attributes, and components.

        pub use crate::generated::{attribute as attributes, component as components, entity::*};
    }
}
