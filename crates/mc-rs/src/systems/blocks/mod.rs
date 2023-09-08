use bevy::prelude::{App, AssetServer, Commands, Res, Resource, Startup};

pub use self::{block::Blocks, state::BlockStates};

pub mod attributes;
pub mod block;
pub mod state;

#[derive(Debug, Clone, Resource)]
pub struct BlockData {
    pub blocks: Blocks,
    pub states: BlockStates,
}

impl BlockData {
    fn create(asset_server: Res<AssetServer>, mut commands: Commands) {
        let blocks = Blocks::create();
        let states = BlockStates::create(&asset_server);

        commands.insert_resource(BlockData { blocks, states });
    }
}

/// Add the [Blocks] and [BlockStates] resources to the app.
pub(super) fn add_systems(app: &mut App) { app.add_systems(Startup, BlockData::create); }
