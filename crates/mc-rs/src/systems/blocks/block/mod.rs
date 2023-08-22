use std::hash::Hash;

use bevy::prelude::*;
use convert_case::{Case, Casing};
use mc_rs_proto::types::ResourceLocation;

use self::{voxel_texture::VoxelTexture, voxel_type::VoxelType};

pub mod complex;
pub mod complex_texture;
pub mod model;
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
        voxel: VoxelType,
        paths: &[&str],
        assets: &AssetServer,
    ) -> Option<Self> {
        Some(Self {
            id,
            name: name.to_case(Case::Title),
            key: ResourceLocation::new(name.to_case(Case::Snake)),
            block_type: BlockType::new_voxel(voxel, VoxelTexture::from_paths(paths, assets)?),
        })
    }

    /// Create a new voxel block with the given id, name, and texture
    #[allow(dead_code)]
    pub fn new_voxel_with(
        id: u32,
        name: &str,
        voxel_type: VoxelType,
        texture: VoxelTexture,
    ) -> Self {
        Self {
            id,
            name: name.to_case(Case::Title),
            key: ResourceLocation::new(name.to_case(Case::Snake)),
            block_type: BlockType::new_voxel(voxel_type, texture),
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

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Voxel {
        voxel_type: VoxelType,
        texture: VoxelTexture,
    },
    Simple {
        id: u32,
        texture: VoxelTexture,

        min_width: f32,
        max_width: f32,

        min_height: f32,
        max_height: f32,

        min_depth: f32,
        max_depth: f32,
    },
    #[allow(dead_code)]
    Complex {
        // TODO
        id: u32,
    },
}

impl BlockType {
    pub const fn new_voxel(voxel_type: VoxelType, texture: VoxelTexture) -> Self {
        Self::Voxel {
            voxel_type,
            texture,
        }
    }

    #[allow(dead_code)]
    pub fn new_simple(dimensions: [f32; 6], id: u32, texture: VoxelTexture) -> Self {
        debug_assert!(dimensions.iter().all(|d| *d >= 0.0 && *d <= 1.0));
        debug_assert!(dimensions[0] <= dimensions[1]);
        debug_assert!(dimensions[2] <= dimensions[3]);
        debug_assert!(dimensions[4] <= dimensions[5]);

        Self::Simple {
            id,
            texture,
            min_width: dimensions[0],
            max_width: dimensions[1],
            min_height: dimensions[2],
            max_height: dimensions[3],
            min_depth: dimensions[4],
            max_depth: dimensions[5],
        }
    }

    pub fn voxel_type(&self) -> VoxelType {
        match self {
            Self::Voxel { voxel_type, .. } => *voxel_type,
            Self::Simple { .. } | Self::Complex { .. } => VoxelType::NoMesh,
        }
    }

    pub fn textures(&self) -> Option<&[Handle<Image>]> {
        match self {
            Self::Voxel { texture, .. } | Self::Simple { texture, .. } => texture.get_textures(),
            Self::Complex { .. } => None,
        }
    }
}

impl From<(VoxelType, VoxelTexture)> for BlockType {
    fn from(value: (VoxelType, VoxelTexture)) -> Self { Self::new_voxel(value.0, value.1) }
}
