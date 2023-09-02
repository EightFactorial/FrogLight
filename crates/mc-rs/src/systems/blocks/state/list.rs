use bevy::prelude::AssetServer;

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
        for (textures, (state_id, model)) in $state_id_range.zip($models).zip($textures) {
            add_state!($states, $asset_server, $block_id, state_id, textures, model);
        }
    };
}

static EMPTY: &[&str] = &[];

pub(super) fn create_states(states: &mut StatesMap, asset_server: &AssetServer) {
    add_state!(states, asset_server, 0u32, 0u32, EMPTY);
    add_state!(states, asset_server, 1u32, 1u32, &["stone.png"]);
    add_state!(states, asset_server, 2u32, 2u32, &["granite.png"]);
}
