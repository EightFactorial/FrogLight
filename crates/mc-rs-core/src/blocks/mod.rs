use bevy::prelude::{App, Commands, Resource, Startup};

pub use self::block::Blocks;

pub mod attributes;
pub mod block;

#[derive(Debug, Clone, Resource)]
pub struct BlockData {
    pub blocks: Blocks,
}

impl BlockData {
    fn create(mut commands: Commands) {
        let blocks = Blocks::create();
        // let states = BlockStates::create(&asset_server);

        commands.insert_resource(BlockData { blocks });
    }
}

/// Add the [Blocks] and [BlockStates] resources to the app.
pub(super) fn setup(app: &mut App) { app.add_systems(Startup, BlockData::create); }
