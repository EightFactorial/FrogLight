use bevy::prelude::AssetServer;

use crate::systems::world::material::StateAnimation;

use super::{model::BlockModel, textures::BlockTextures, BlockState, StatesMap};

macro_rules! add_state {
    ($states:expr, $asset_server:expr, $block_id:expr, $state_id:expr, $textures:expr) => {
        add_state!(
            $states,
            $asset_server,
            $block_id,
            $state_id,
            $textures,
            match $textures.is_empty() {
                true => BlockModel::None,
                false => BlockModel::Standard,
            }
        );
    };
    ($states:expr, $asset_server:expr, $block_id:expr, $state_id:expr, $textures:expr, $model:expr) => {
        $states.insert(
            $state_id,
            BlockState {
                block_id: $block_id,
                state_id: $state_id,
                textures: match $textures.is_empty() {
                    true => BlockTextures::NONE,
                    false => BlockTextures::new($textures, $asset_server),
                },
                model: $model,
            },
        );
    };
}

macro_rules! add_state_range {
    ($states:expr, $asset_server:expr, $block_id:expr, $state_id_range:expr, $textures:expr) => {
        for state_id in $state_id_range {
            add_state!($states, $asset_server, $block_id, state_id, $textures);
        }
    };
    ($states:expr, $asset_server:expr, $block_id:expr, $state_id_range:expr, $textures:expr, $models:expr) => {
        for ((state_id, model), textures) in $state_id_range.zip_eq($models).zip_eq($textures) {
            add_state!($states, $asset_server, $block_id, state_id, textures, model);
        }
    };
}

static EMPTY: &[&str] = &[];

pub(super) fn create_states(states: &mut StatesMap, asset_server: &AssetServer) {
    add_state!(states, asset_server, 0u32, 0u32, EMPTY);
    add_state!(states, asset_server, 1u32, 1u32, &["stone.png"]);
    add_state!(states, asset_server, 2u32, 2u32, &["granite.png"]);
    add_state!(states, asset_server, 3u32, 3u32, &["polished_granite.png"]);
    add_state!(states, asset_server, 4u32, 4u32, &["diorite.png"]);
    add_state!(states, asset_server, 5u32, 5u32, &["polished_diorite.png"]);
    add_state!(states, asset_server, 6u32, 6u32, &["andesite.png"]);
    add_state!(states, asset_server, 7u32, 7u32, &["polished_andesite.png"]);
    add_state_range!(
        states,
        asset_server,
        8u32,
        8u32..=9u32,
        &["grass_block_top.png", "dirt.png", "grass_block_side.png"]
    );
    add_state!(states, asset_server, 9u32, 10u32, &["dirt.png"]);
    add_state!(states, asset_server, 10u32, 11u32, &["coarse_dirt.png"]);
    add_state_range!(
        states,
        asset_server,
        11u32,
        12u32..=13u32,
        &["podzol_top.png", "dirt.png", "podzol_side.png"]
    );

    // TODO: Add instructions to material for animated textures
    for id in 80..=95 {
        states.insert(
            id,
            BlockState {
                block_id: 32,
                state_id: id,
                model: BlockModel::Standard,
                textures: BlockTextures::new_with_animations(
                    &[("water_still.png", StateAnimation::new(0.2, 0..32))],
                    asset_server,
                ),
            },
        );
    }
    // add_state_range!(
    //     states,
    //     asset_server,
    //     32u32,
    //     80u32..=95u32,
    //     &["water_still.png"]
    // );
    add_state_range!(
        states,
        asset_server,
        33u32,
        96u32..=111u32,
        &["lava_still.png"]
    );

    add_state_range!(
        states,
        asset_server,
        46u32,
        130..=132,
        &["oak_log_top.png", "oak_log.png"]
    );
    add_state_range!(
        states,
        asset_server,
        47u32,
        133..=135,
        &["spruce_log_top.png", "spruce_log.png"]
    );
    add_state_range!(
        states,
        asset_server,
        48u32,
        136..=138,
        &["birch_log_top.png", "birch_log.png"]
    );
    add_state_range!(
        states,
        asset_server,
        49u32,
        139u32..=141u32,
        &["jungle_log_top.png", "jungle_log.png"]
    );
    add_state_range!(
        states,
        asset_server,
        50u32,
        142u32..=144u32,
        &["acacia_log_top.png", "acacia_log.png"]
    );
    add_state_range!(
        states,
        asset_server,
        51u32,
        145u32..=147u32,
        &["cherry_log_top.png", "cherry_log.png"]
    );
    add_state_range!(
        states,
        asset_server,
        52u32,
        148u32..=150u32,
        &["dark_oak_log_top.png", "dark_oak_log.png"]
    );

    add_state_range!(states, asset_server, 82u32, 237..=264, &["oak_leaves.png"]);
    add_state_range!(
        states,
        asset_server,
        83u32,
        265..=292,
        &["spruce_leaves.png"]
    );
    add_state_range!(
        states,
        asset_server,
        84u32,
        293..=320,
        &["birch_leaves.png"]
    );
    add_state_range!(
        states,
        asset_server,
        85u32,
        321..=348,
        &["jungle_leaves.png"]
    );
    add_state_range!(
        states,
        asset_server,
        86u32,
        349..=376,
        &["acacia_leaves.png"]
    );
    add_state_range!(
        states,
        asset_server,
        87u32,
        377..=404,
        &["cherry_leaves.png"]
    );
    add_state_range!(
        states,
        asset_server,
        88u32,
        405..=432,
        &["dark_oak_leaves.png"]
    );

    for id in 11021..=11026 {
        add_state!(
            states,
            asset_server,
            539u32,
            id,
            &["oak_planks.png"],
            BlockModel::slab_lower()
        );
    }
}
