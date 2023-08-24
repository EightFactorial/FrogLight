use bevy::{prelude::AssetServer, utils::HashMap};
use log::error;

use crate::systems::blocks::block::{voxel_type::VoxelType, Block};

/// Insert a block into the block list
fn insert_voxel(
    blocks: &mut HashMap<u32, Block>,
    id: u32,
    name: &str,
    paths: &[&str],
    assets: &AssetServer,
) {
    if let Some(block) = Block::new_voxel(id, name, VoxelType::Opaque(id), paths, assets) {
        blocks.insert(id, block);
    } else {
        error!("Failed to create block with id {}", id);

        let fallback = blocks[&u32::MAX].block_type.clone();
        blocks.insert(id, Block::new(id, name, fallback));
    }
}

/// Insert a block with a voxel type into the block list
fn insert_voxel_type(
    blocks: &mut HashMap<u32, Block>,
    id: u32,
    name: &str,
    voxel_type: VoxelType,
    paths: &[&str],
    assets: &AssetServer,
) {
    if let Some(block) = Block::new_voxel(id, name, voxel_type, paths, assets) {
        blocks.insert(id, block);
    } else {
        error!("Failed to create block with id {}", id);

        let fallback = blocks[&u32::MAX].block_type.clone();
        blocks.insert(id, Block::new(id, name, fallback));
    }
}

/// Insert a block with a voxel type into the block list
fn insert_voxel_anim(
    blocks: &mut HashMap<u32, Block>,
    id: u32,
    name: &str,
    voxel_type: VoxelType,
    anim: &[Option<(u32, u32)>],
    paths: &[&str],
    assets: &AssetServer,
) {
    if let Some(block) = Block::new_voxel_anim(id, name, voxel_type, anim, paths, assets) {
        blocks.insert(id, block);
    } else {
        error!("Failed to create block with id {}", id);

        let fallback = blocks[&u32::MAX].block_type.clone();
        blocks.insert(id, Block::new(id, name, fallback));
    }
}

/// Insert a simple block into the block list
fn insert_simple(
    blocks: &mut HashMap<u32, Block>,
    id: u32,
    name: &str,
    dimensions: [f32; 6],
    collision: bool,
    paths: &[&str],
    assets: &AssetServer,
) {
    if let Some(block) = Block::new_simple(id, name, paths, dimensions, collision, assets) {
        blocks.insert(id, block);
    } else {
        error!("Failed to create block with id {}", id);

        let fallback = blocks[&u32::MAX].block_type.clone();
        blocks.insert(id, Block::new(id, name, fallback));
    }
}

/// TODO: Automatically generate this list
pub(super) fn initialize(blocks: &mut HashMap<u32, Block>, assets: &AssetServer) {
    insert_voxel_type(blocks, 0, "Air", VoxelType::Empty, &[], assets);

    insert_voxel(blocks, 1, "Stone", &["stone.png"], assets);

    insert_voxel(blocks, 2, "Granite", &["granite.png"], assets);
    insert_voxel(
        blocks,
        3,
        "Polished Granite",
        &["polished_granite.png"],
        assets,
    );

    insert_voxel(blocks, 4, "Diorite", &["diorite.png"], assets);
    insert_voxel(
        blocks,
        5,
        "Polished Diorite",
        &["polished_diorite.png"],
        assets,
    );

    insert_voxel(blocks, 6, "Andesite", &["andesite.png"], assets);
    insert_voxel(
        blocks,
        7,
        "Polished Andesite",
        &["polished_andesite.png"],
        assets,
    );

    for id in 8..=9 {
        insert_voxel(
            blocks,
            id,
            "Grass Block",
            &["grass_block_top.png", "dirt.png", "grass_block_side.png"],
            assets,
        );
    }
    insert_voxel(blocks, 10, "Dirt", &["dirt.png"], assets);
    insert_voxel(blocks, 11, "Coarse Dirt", &["coarse_dirt.png"], assets);
    for id in 12..=13 {
        insert_voxel(
            blocks,
            id,
            "Podzol",
            &["podzol_top.png", "dirt.png", "podzol_side.png"],
            assets,
        );
    }

    insert_voxel(blocks, 79, "Bedrock", &["bedrock.png"], assets);

    for id in 80..=95 {
        insert_voxel_anim(
            blocks,
            id,
            "Water",
            VoxelType::NoMesh(rand::random()),
            &[Some((32, 2))],
            &["water_still.png"],
            assets,
        );
    }
    for id in 96..=111 {
        insert_voxel_anim(
            blocks,
            id,
            "Lava",
            VoxelType::NoMesh(rand::random()),
            &[Some((20, 2))],
            &["lava_still.png"],
            assets,
        );
    }

    insert_voxel(blocks, 127, "Coal Ore", &["coal_ore.png"], assets);
    insert_voxel(
        blocks,
        128,
        "Deepslate Coal Ore",
        &["deepslate_coal_ore.png"],
        assets,
    );
    insert_voxel(blocks, 125, "Iron Ore", &["iron_ore.png"], assets);
    insert_voxel(
        blocks,
        126,
        "Deepslate Iron Ore",
        &["deepslate_iron_ore.png"],
        assets,
    );
    insert_voxel(blocks, 123, "Gold Ore", &["gold_ore.png"], assets);
    insert_voxel(
        blocks,
        124,
        "Deepslate Gold Ore",
        &["deepslate_gold_ore.png"],
        assets,
    );
    insert_voxel(blocks, 5734, "Redstone Ore", &["redstone_ore.png"], assets);
    insert_voxel(
        blocks,
        5735,
        "Deepslate Redstone Ore",
        &["deepslate_redstone_ore.png"],
        assets,
    );
    insert_voxel(blocks, 520, "Lapis Lazuli Ore", &["lapis_ore.png"], assets);
    insert_voxel(
        blocks,
        521,
        "Deepslate Lapis Lazuli Ore",
        &["deepslate_lapis_ore.png"],
        assets,
    );
    insert_voxel(blocks, 4274, "Diamond Ore", &["diamond_ore.png"], assets);
    insert_voxel(
        blocks,
        4275,
        "Deepslate Diamond Ore",
        &["deepslate_diamond_ore.png"],
        assets,
    );
    insert_voxel(blocks, 7511, "Emerald Ore", &["emerald_ore.png"], assets);
    insert_voxel(
        blocks,
        7512,
        "Deepslate Emerald Ore",
        &["deepslate_emerald_ore.png"],
        assets,
    );
    insert_voxel(blocks, 21558, "Copper Ore", &["copper_ore.png"], assets);
    insert_voxel(
        blocks,
        21559,
        "Deepslate Copper Ore",
        &["deepslate_copper_ore.png"],
        assets,
    );

    for id in 237..=264 {
        insert_voxel_type(
            blocks,
            id,
            "Oak Leaves",
            VoxelType::Translucent(id),
            &["oak_leaves.png"],
            assets,
        );
    }
    for id in 130..=132 {
        insert_voxel(
            blocks,
            id,
            "Oak Log",
            &["oak_log_top.png", "oak_log.png"],
            assets,
        );
    }
    for id in 321..=348 {
        insert_voxel_type(
            blocks,
            id,
            "Jungle Leaves",
            VoxelType::Translucent(id),
            &["jungle_leaves.png"],
            assets,
        );
    }
    for id in 139..=141 {
        insert_voxel(
            blocks,
            id,
            "Jungle Log",
            &["jungle_log_top.png", "jungle_log.png"],
            assets,
        );
    }

    for id in 11021..=11026 {
        insert_simple(
            blocks,
            id,
            "Oak Slab",
            [0.0, 0.0, 0.0, 1.0, 0.5, 1.0],
            true,
            &["oak_planks.png"],
            assets,
        )
    }

    insert_voxel(blocks, 112, "Sand", &["sand.png"], assets);
    insert_voxel(blocks, 118, "Gravel", &["gravel.png"], assets);
    insert_voxel(blocks, 5798, "Clay", &["clay.png"], assets);

    insert_voxel(
        blocks,
        20890,
        "Block of Amethyst",
        &["amethyst_block.png"],
        assets,
    );
    insert_voxel(
        blocks,
        20891,
        "Budding Amethyst",
        &["budding_amethyst.png"],
        assets,
    );

    insert_voxel(blocks, 20940, "Tuff", &["tuff.png"], assets);
    insert_voxel(blocks, 20941, "Calcite", &["calcite.png"], assets);
    for id in 22450..=22452 {
        insert_voxel(blocks, id, "Deepslate", &["deepslate.png"], assets);
    }
    insert_voxel(
        blocks,
        24102,
        "Smooth Basalt",
        &["smooth_basalt.png"],
        assets,
    );
}
