use mc_rs_proto::types::ResourceLocation;

use super::{properties::BlockProperties, Block, BlocksMap};

macro_rules! add_block {
    ($blocks:expr, $block_id:expr, $block_states:expr, $name:expr, $properties:expr) => {
        $blocks.insert(
            $block_id,
            Block {
                block_id: $block_id,
                block_states: $block_states,
                name: $name.to_string(),
                key: ResourceLocation::new($name.to_ascii_lowercase()),
                properties: $properties,
            },
        );
    };
    ($blocks:expr, $block_id:expr, $block_states:expr, $name:expr, $key:expr, $properties:expr) => {
        $blocks.insert(
            $block_id,
            Block {
                block_id: $block_id,
                block_states: $block_states,
                name: $name.to_string(),
                key: ResourceLocation::new($key),
                properties: $properties,
            },
        );
    };
}

pub(super) fn create_blocks(blocks: &mut BlocksMap) {
    add_block!(
        blocks,
        0u32,
        0..0,
        "Air",
        BlockProperties {
            is_air: true,
            ..Default::default()
        }
    );
    add_block!(
        blocks,
        1u32,
        1..1,
        "Stone",
        BlockProperties {
            ..Default::default()
        }
    );
    add_block!(
        blocks,
        2u32,
        2..2,
        "Granite",
        BlockProperties {
            ..Default::default()
        }
    );
}
