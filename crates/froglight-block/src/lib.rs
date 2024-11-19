#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![feature(build_hasher_default_const_new)]
#![feature(const_type_name)]
#![feature(const_type_id)]

mod generated;
pub use generated::{attribute, block};

mod traits;
pub use traits::{BlockAttribute, BlockState, BlockStateExt};

mod storage;
pub use storage::BlockStorage;
#[cfg(feature = "bevy")]
pub use storage::{
    BlockBuilder, BlockPlugin, BlockStorageArc, ReflectBlockBuilder, VanillaBuilder,
};

#[cfg(test)]
mod test;
