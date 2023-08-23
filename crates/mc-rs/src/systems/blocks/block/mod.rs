use std::hash::Hash;

use bevy::prelude::*;
use convert_case::{Case, Casing};
use mc_rs_proto::types::ResourceLocation;

use self::{voxel_texture::VoxelTexture, voxel_type::VoxelType};

pub mod complex;
pub mod complex_model;
pub mod complex_texture;
pub mod voxel_texture;
pub mod voxel_type;

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u32,
    pub name: String,
    pub key: ResourceLocation,
    pub block_type: BlockType,
}

impl Block {
    pub fn new(id: u32, name: &str, block_type: BlockType) -> Self {
        Self {
            id,
            name: name.to_case(Case::Title),
            key: ResourceLocation::new(name.to_case(Case::Snake)),
            block_type,
        }
    }

    /// Create a new voxel block with the given id, name, and texture path(s)
    pub fn new_voxel(
        id: u32,
        name: &str,
        voxel_type: VoxelType,
        paths: &[&str],
        assets: &AssetServer,
    ) -> Option<Self> {
        Some(Self::new_voxel_with(
            id,
            name,
            voxel_type,
            VoxelTexture::from_paths(paths, assets)?,
        ))
    }

    /// Create a new voxel block with the given id, name, and texture
    pub fn new_voxel_with(
        id: u32,
        name: &str,
        voxel_type: VoxelType,
        texture: VoxelTexture,
    ) -> Self {
        Self::new(id, name, BlockType::new_voxel(voxel_type, texture))
    }

    /// Create a new simple block with the given id, name, and texture path(s)
    pub fn new_simple(
        id: u32,
        name: &str,
        paths: &[&str],
        dimensions: [f32; 6],
        assets: &AssetServer,
    ) -> Option<Self> {
        Some(Self::new_simple_with(
            id,
            name,
            VoxelTexture::from_paths(paths, assets)?,
            dimensions,
        ))
    }

    /// Create a new simple block with the given id, name, and texture
    pub fn new_simple_with(
        id: u32,
        name: &str,
        texture: VoxelTexture,
        dimensions: [f32; 6],
    ) -> Self {
        Self::new(id, name, BlockType::new_simple(dimensions, texture))
    }

    /// Get the voxel type of the block
    pub fn voxel_type(&self) -> VoxelType {
        match &self.block_type {
            BlockType::Voxel { voxel_type, .. } => *voxel_type,
            BlockType::Simple { .. } | BlockType::Complex { .. } => VoxelType::NoMesh,
        }
    }

    /// Get the textures of the block
    pub fn textures(&self) -> Option<&[Handle<Image>]> {
        match &self.block_type {
            BlockType::Voxel { texture, .. } | BlockType::Simple { texture, .. } => {
                texture.get_textures()
            }
            BlockType::Complex { .. } => todo!(),
        }
    }
}

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.name == other.name && self.key == other.key
    }
}

impl Hash for Block {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.name.hash(state);
        self.key.hash(state);
    }
}

#[derive(Debug, Clone)]
pub enum BlockType {
    /// A whole block
    Voxel {
        voxel_type: VoxelType,
        texture: VoxelTexture,
    },
    /// A block that takes up a portion of a block
    Simple {
        texture: VoxelTexture,
        dimensions: [f32; 6],
    },
    /// A block that has it's own mesh
    #[allow(dead_code)]
    Complex {
        mesh: Mesh,
        textures: Vec<Handle<Image>>,
    },
}

impl BlockType {
    pub const fn new_voxel(voxel_type: VoxelType, texture: VoxelTexture) -> Self {
        Self::Voxel {
            voxel_type,
            texture,
        }
    }

    pub fn new_simple(dimensions: [f32; 6], texture: VoxelTexture) -> Self {
        debug_assert!(dimensions.iter().all(|d| *d >= 0.0 && *d <= 1.0));
        debug_assert!(dimensions[0] < dimensions[3]);
        debug_assert!(dimensions[1] < dimensions[4]);
        debug_assert!(dimensions[2] < dimensions[5]);

        Self::Simple {
            texture,
            dimensions,
        }
    }
}
