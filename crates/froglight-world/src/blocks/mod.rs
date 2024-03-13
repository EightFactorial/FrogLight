//! All blocks and block data
//!
//! @generated by `froglight-generator #dc73c64`

use bevy_app::App;

pub mod attributes;

pub mod block_list;
use block_list::BlockEnum;

mod traits;
pub use traits::{BlockAttribute, BlockExt, BlockType};

mod reflect;
pub use reflect::ReflectBlockType;

pub(crate) mod registry;
pub use registry::{BlockEnumV1_20_0, BlockRegistry, InnerBlockRegistry};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register all blocks for reflection
    BlockEnum::register(app);
    // Register all block attributes for reflection
    attributes::register(app);

    // Build and register the block registry
    registry::build(app);
}
