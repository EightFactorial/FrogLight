#![allow(dead_code)]

use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use mc_rs_proto::types::ResourceLocation;

use super::block::{Block, BlockTexture};

/// Adds all block systems to the app
pub(super) fn add_systems(app: &mut App) { app.add_systems(Startup, Blocks::init_blocks); }

#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct Blocks(pub HashMap<u32, Block>);

impl Blocks {
    /// Initialize the block list and load the block textures
    fn init_blocks(mut commands: Commands, assets: Res<AssetServer>) {
        let mut blocks = Self(HashMap::new());

        blocks.insert_block(0, "Air", BlockTexture::None);

        blocks.insert_block(1, "Stone", BlockTexture::new_single("stone.png", &assets));

        blocks.insert_block(
            2,
            "Granite",
            BlockTexture::new_single("granite.png", &assets),
        );
        blocks.insert_block(
            3,
            "Polished Granite",
            BlockTexture::new_single("polished_granite.png", &assets),
        );

        blocks.insert_block(
            4,
            "Diorite",
            BlockTexture::new_single("diorite.png", &assets),
        );
        blocks.insert_block(
            5,
            "Polished Diorite",
            BlockTexture::new_single("polished_diorite.png", &assets),
        );

        blocks.insert_block(
            6,
            "Andesite",
            BlockTexture::new_single("andesite.png", &assets),
        );
        blocks.insert_block(
            7,
            "Polished Andesite",
            BlockTexture::new_single("polished_andesite.png", &assets),
        );

        blocks.insert_block(
            8,
            "Grass Block",
            BlockTexture::new_top_bottom_sides(
                "grass_block_top.png",
                "dirt.png",
                "grass_block_side.png",
                &assets,
            ),
        );
        blocks.insert_block(9, "Dirt", BlockTexture::new_single("dirt.png", &assets));

        for id in 22047..=22049 {
            blocks.insert_block(
                id,
                "Deepslate",
                BlockTexture::new_single("deepslate.png", &assets),
            );
        }

        blocks.insert(
            u32::MAX,
            Block {
                id: u32::MAX,
                name: "Error".to_string(),
                key: ResourceLocation::new("mc-rs:error"),
                textures: BlockTexture::new_single("light_blue_wool.png", &assets),
            },
        );

        commands.insert_resource(blocks);
    }

    /// Insert a block into the block list
    fn insert_block(&mut self, id: u32, name: &str, textures: BlockTexture) {
        self.insert(id, Block::new(id, name, textures));
    }

    /// Returns true if all the block textures are loaded
    pub fn is_loaded(&self, assets: &AssetServer) -> bool {
        let (l, t) = self.get_progress(assets);
        l == t
    }

    /// Return the progress of loading the block textures
    pub fn get_progress(&self, assets: &AssetServer) -> (u32, u32) {
        // Get the number of blocks with all textures loaded
        let blocks_loaded = self.values().fold(0u32, |acc, block| {
            let Some(textures) = block.textures.get_textures() else {
                return acc;
            };

            let ids = textures.iter().map(|t| t.id());
            acc + matches!(assets.get_group_load_state(ids), LoadState::Loaded) as u32
        });

        // Get the number of blocks with textures
        let blocks_with_textures = self
            .values()
            .filter(|b| b.textures.get_textures().is_some())
            .count() as u32;

        (blocks_loaded, blocks_with_textures)
    }
}
