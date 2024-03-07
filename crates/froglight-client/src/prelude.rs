//! use `froglight-client::prelude::*;` to import common types and traits.

pub use froglight_assets::*;
pub use froglight_core::{common::*, components::*, events::*, resources::*, systemsets::*};
pub use froglight_interface::{
    materials::*,
    plugins::{
        inspector::InspectorEnable,
        uiscale::{UiScaleEnable, UiScaleMaximum},
    },
};
pub use froglight_world::{
    blocks::{block_list as blocks, BlockExt, BlockRegistry, BlockType, ReflectBlockType},
    maps::*,
    world::{Chunk, Section},
};
