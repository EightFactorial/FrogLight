#![allow(dead_code)]

use std::sync::{Arc, RwLock};

use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use mc_rs_proto::types::ResourceLocation;

use crate::systems::app_state::MenuSet;

use super::block::{Block, BlockTexture, VoxelType};

/// Adds all block systems to the app
pub(super) fn add_systems(app: &mut App) {
    app.add_systems(Startup, Blocks::init_blocks);

    app.init_resource::<BlocksLoaded>();

    app.add_systems(
        Update,
        (
            BlocksLoaded::check_loaded
                .run_if(resource_exists::<BlocksLoaded>().and_then(not(BlocksLoaded::is_loaded))),
            BlocksLoaded::destroy
                .run_if(resource_exists::<BlocksLoaded>().and_then(BlocksLoaded::is_loaded)),
        )
            .in_set(MenuSet),
    );
}

#[derive(Clone, Default, Resource, Deref, DerefMut)]
pub struct Blocks(Arc<RwLock<HashMap<u32, Block>>>);

impl Blocks {
    /// Initialize the block list and load the block textures
    fn init_blocks(mut commands: Commands, assets: Res<AssetServer>) {
        let mut blocks = Self::default();

        // Insert the error block
        blocks.write().unwrap().insert(
            u32::MAX,
            Block {
                id: u32::MAX,
                name: "Error".to_string(),
                key: ResourceLocation::new("mc-rs:error"),
                texture: BlockTexture::from_paths(&["light_blue_wool.png"], &assets).unwrap(),
                voxel_type: VoxelType::Opaque(u32::MAX),
            },
        );

        blocks.insert_block_type(0, "Air", VoxelType::Empty, &Vec::new(), &assets);

        blocks.insert_block(1, "Stone", &["stone.png"], &assets);

        blocks.insert_block(2, "Granite", &["granite.png"], &assets);
        blocks.insert_block(3, "Polished Granite", &["polished_granite.png"], &assets);

        blocks.insert_block(4, "Diorite", &["diorite.png"], &assets);
        blocks.insert_block(5, "Polished Diorite", &["polished_diorite.png"], &assets);

        blocks.insert_block(6, "Andesite", &["andesite.png"], &assets);
        blocks.insert_block(7, "Polished Andesite", &["polished_andesite.png"], &assets);

        blocks.insert_block(
            8,
            "Grass Block",
            &["grass_block_top.png", "dirt.png", "grass_block_side.png"],
            &assets,
        );
        blocks.insert_block(9, "Dirt", &["dirt.png"], &assets);

        for id in 22047..=22049 {
            blocks.insert_block(id, "Deepslate", &["deepslate.png"], &assets);
        }

        commands.insert_resource(blocks);
    }

    /// Insert a block into the block list
    fn insert_block(&mut self, id: u32, name: &str, paths: &[&str], assets: &AssetServer) {
        if let Some(block) = Block::new(id, name, VoxelType::Opaque(id), paths, assets) {
            self.write().unwrap().insert(id, block);
        } else {
            error!("Failed to create block with id {}", id);

            let fallback = self.read().unwrap().get(&u32::MAX).unwrap().texture.clone();
            self.write().unwrap().insert(
                id,
                Block::new_with(id, name, VoxelType::Opaque(id), fallback),
            );
        }
    }

    /// Insert a block with a voxel type into the block list
    fn insert_block_type(
        &mut self,
        id: u32,
        name: &str,
        voxel: VoxelType,
        paths: &[&str],
        assets: &AssetServer,
    ) {
        if let Some(block) = Block::new(id, name, voxel, paths, assets) {
            self.write().unwrap().insert(id, block);
        } else {
            error!("Failed to create block with id {}", id);

            let fallback = self.read().unwrap().get(&u32::MAX).unwrap().texture.clone();
            self.write()
                .unwrap()
                .insert(id, Block::new_with(id, name, voxel, fallback));
        }
    }

    /// Returns true if all the block textures are loaded
    pub fn is_loaded(&self, assets: &AssetServer) -> bool {
        self.blocks_loaded(assets) == self.blocks_with_textures()
    }

    // Get the number of blocks with all textures loaded
    pub fn blocks_loaded(&self, assets: &AssetServer) -> u32 {
        self.read().unwrap().values().fold(0u32, |acc, block| {
            let Some(textures) = block.texture.get_textures() else {
                return acc;
            };

            let ids = textures.iter().map(|t| t.id());
            acc + matches!(
                assets.get_group_load_state(ids),
                LoadState::Loaded | LoadState::Failed
            ) as u32
        })
    }

    // Get the number of blocks with textures
    pub fn blocks_with_textures(&self) -> u32 {
        self.read()
            .unwrap()
            .values()
            .filter(|b| b.texture.get_textures().is_some())
            .count() as u32
    }

    /// Return the progress of loading the block textures
    pub fn progress(&self, assets: &AssetServer) -> f32 {
        self.blocks_loaded_f32(assets) / self.blocks_with_textures_f32()
    }

    // Get the number of blocks with textures
    pub fn blocks_with_textures_f32(&self) -> f32 { self.blocks_with_textures() as f32 }

    // Get the number of blocks with all textures loaded
    pub fn blocks_loaded_f32(&self, assets: &AssetServer) -> f32 {
        self.blocks_loaded(assets) as f32
    }

    /// Replaces all failed block textures with the error block texture
    ///
    /// Returns the number of blocks that were fixed
    pub fn replace_errors(&mut self, assets: &AssetServer) -> u32 {
        let fallback = self.read().unwrap().get(&u32::MAX).unwrap().texture.clone();
        let mut acc = 0;

        for block in self.write().unwrap().values_mut() {
            if let Some(textures) = block.texture.get_textures() {
                // Check if any of the textures failed to load
                let ids = textures.iter().map(|t| t.id());
                if assets.get_group_load_state(ids) == LoadState::Failed {
                    // Replace the block texture with the error block texture
                    block.texture = fallback.clone();
                    acc += 1;
                }
            }
        }

        acc
    }
}

/// A resource that is true when all blocks are loaded
#[derive(Clone, Default, PartialEq, Resource, Deref, DerefMut)]
pub struct BlocksLoaded {
    #[deref]
    pub bool: bool,
    pub percent: f32,
}

impl BlocksLoaded {
    /// A system that checks if all the block textures are loaded
    /// and replaces any broken textures with the fallback
    fn check_loaded(
        mut blocks: ResMut<Blocks>,
        mut loaded: ResMut<BlocksLoaded>,
        assets: Res<AssetServer>,
    ) {
        if blocks.is_loaded(&assets) {
            // Replace any failed textures with the error block texture
            let fixed = blocks.replace_errors(&assets);

            if fixed > 0 {
                // TODO: Some sort of error popup?
                error!("{fixed} blocks failed to load textures");
            } else {
                info!(
                    "All blocks ({}) loaded successfully",
                    blocks.read().unwrap().len()
                );
            }

            // Set the blocks loaded resource to true
            loaded.bool = true;
            loaded.percent = 1.0;
        } else {
            let p = blocks.progress(&assets);
            loaded.percent = p;

            info!("Loaded {p}% of blocks");
        }
    }

    fn destroy(mut commands: Commands) { commands.remove_resource::<BlocksLoaded>(); }

    /// Get the percent of blocks loaded
    pub fn get_percent(loaded: Res<BlocksLoaded>) -> f32 { loaded.percent }

    /// Get if all blocks are loaded
    pub fn is_loaded(loaded: Res<BlocksLoaded>) -> bool { loaded.bool }
}
