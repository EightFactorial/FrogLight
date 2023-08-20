#![allow(dead_code)]

use std::sync::{Arc, RwLock};

use bevy::{asset::LoadState, prelude::*, utils::HashMap};
use mc_rs_proto::types::ResourceLocation;

use crate::systems::app_state::MenuSet;

use super::block::{Block, BlockTexture, VoxelType};

/// Adds all block systems to the app
pub(super) fn add_systems(app: &mut App) {
    app.init_resource::<BlocksLoaded>();
    app.add_systems(Startup, Blocks::init_blocks);

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

        for id in 8..=9 {
            blocks.insert_block(
                id,
                "Grass Block",
                &["grass_block_top.png", "dirt.png", "grass_block_side.png"],
                &assets,
            );
        }
        blocks.insert_block(10, "Dirt", &["dirt.png"], &assets);
        blocks.insert_block(11, "Coarse Dirt", &["coarse_dirt.png"], &assets);
        for id in 12..=13 {
            blocks.insert_block(
                id,
                "Podzol",
                &["podzol_top.png", "dirt.png", "podzol_side.png"],
                &assets,
            );
        }

        blocks.insert_block(79, "Bedrock", &["bedrock.png"], &assets);

        for id in 80..=95 {
            blocks.insert_block_type(
                id,
                "Water",
                VoxelType::Translucent(id),
                &["water_still.png"],
                &assets,
            );
        }
        for id in 96..=111 {
            blocks.insert_block_type(
                id,
                "Lava",
                VoxelType::Translucent(id),
                &["lava_still.png"],
                &assets,
            );
        }

        blocks.insert_block(127, "Coal Ore", &["coal_ore.png"], &assets);
        blocks.insert_block(
            128,
            "Deepslate Coal Ore",
            &["deepslate_coal_ore.png"],
            &assets,
        );
        blocks.insert_block(125, "Iron Ore", &["iron_ore.png"], &assets);
        blocks.insert_block(
            126,
            "Deepslate Iron Ore",
            &["deepslate_iron_ore.png"],
            &assets,
        );
        blocks.insert_block(123, "Gold Ore", &["gold_ore.png"], &assets);
        blocks.insert_block(
            124,
            "Deepslate Gold Ore",
            &["deepslate_gold_ore.png"],
            &assets,
        );
        blocks.insert_block(5734, "Redstone Ore", &["redstone_ore.png"], &assets);
        blocks.insert_block(
            5735,
            "Deepslate Redstone Ore",
            &["deepslate_redstone_ore.png"],
            &assets,
        );
        blocks.insert_block(520, "Lapis Lazuli Ore", &["lapis_ore.png"], &assets);
        blocks.insert_block(
            521,
            "Deepslate Lapis Lazuli Ore",
            &["deepslate_lapis_ore.png"],
            &assets,
        );
        blocks.insert_block(4274, "Diamond Ore", &["diamond_ore.png"], &assets);
        blocks.insert_block(
            4275,
            "Deepslate Diamond Ore",
            &["deepslate_diamond_ore.png"],
            &assets,
        );
        blocks.insert_block(7511, "Emerald Ore", &["emerald_ore.png"], &assets);
        blocks.insert_block(
            7512,
            "Deepslate Emerald Ore",
            &["deepslate_emerald_ore.png"],
            &assets,
        );
        blocks.insert_block(21558, "Copper Ore", &["copper_ore.png"], &assets);
        blocks.insert_block(
            21559,
            "Deepslate Copper Ore",
            &["deepslate_copper_ore.png"],
            &assets,
        );

        for id in 237..=264 {
            blocks.insert_block_type(
                id,
                "Oak Leaves",
                VoxelType::Translucent(id),
                &["oak_leaves.png"],
                &assets,
            );
        }
        for id in 130..=132 {
            blocks.insert_block(id, "Oak Log", &["oak_log_top.png", "oak_log.png"], &assets);
        }
        for id in 321..=348 {
            blocks.insert_block_type(
                id,
                "Jungle Leaves",
                VoxelType::Translucent(id),
                &["jungle_leaves.png"],
                &assets,
            );
        }
        for id in 139..=141 {
            blocks.insert_block(
                id,
                "Jungle Log",
                &["jungle_log_top.png", "jungle_log.png"],
                &assets,
            );
        }

        blocks.insert_block(112, "Sand", &["sand.png"], &assets);
        blocks.insert_block(118, "Gravel", &["gravel.png"], &assets);
        blocks.insert_block(5798, "Clay", &["clay.png"], &assets);

        blocks.insert_block(20890, "Block of Amethyst", &["amethyst_block.png"], &assets);
        blocks.insert_block(
            20891,
            "Budding Amethyst",
            &["budding_amethyst.png"],
            &assets,
        );

        blocks.insert_block(20940, "Tuff", &["tuff.png"], &assets);
        blocks.insert_block(20941, "Calcite", &["calcite.png"], &assets);
        for id in 22450..=22452 {
            blocks.insert_block(id, "Deepslate", &["deepslate.png"], &assets);
        }
        blocks.insert_block(24102, "Smooth Basalt", &["smooth_basalt.png"], &assets);

        commands.insert_resource(blocks);
    }

    /// Insert a block into the block list
    fn insert_block(&mut self, id: u32, name: &str, paths: &[&str], assets: &AssetServer) {
        if let Some(block) = Block::new(id, name, VoxelType::Opaque(id), paths, assets) {
            self.write().unwrap().insert(id, block);
        } else {
            error!("Failed to create block with id {}", id);

            let fallback = self.read().unwrap()[&u32::MAX].texture.clone();
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
        voxel_type: VoxelType,
        paths: &[&str],
        assets: &AssetServer,
    ) {
        if let Some(block) = Block::new(id, name, voxel_type, paths, assets) {
            self.write().unwrap().insert(id, block);
        } else {
            error!("Failed to create block with id {}", id);

            let fallback = self.read().unwrap()[&u32::MAX].texture.clone();
            self.write()
                .unwrap()
                .insert(id, Block::new_with(id, name, voxel_type, fallback));
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
