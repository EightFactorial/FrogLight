#![allow(clippy::module_inception)]

mod create_blocks;
pub(super) use create_blocks::generate_blocks;

mod create_impls;
pub(super) use create_impls::generate_block_impls;
