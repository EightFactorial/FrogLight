use bevy::prelude::*;
use convert_case::{Case, Casing};
use mc_rs_proto::types::ResourceLocation;

use self::{voxel_texture::VoxelTexture, voxel_type::VoxelType};

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
        collision: bool,
        assets: &AssetServer,
    ) -> Option<Self> {
        Some(Self::new_simple_with(
            id,
            name,
            VoxelTexture::from_paths(paths, assets)?,
            dimensions,
            collision,
        ))
    }

    /// Create a new simple block with the given id, name, and texture
    pub fn new_simple_with(
        id: u32,
        name: &str,
        texture: VoxelTexture,
        dimensions: [f32; 6],
        collision: bool,
    ) -> Self {
        Self::new(
            id,
            name,
            BlockType::new_simple(dimensions, texture, collision),
        )
    }

    /// Get the voxel type of the block
    pub fn voxel_type(&self) -> VoxelType {
        match &self.block_type {
            BlockType::Voxel {
                voxel_type: VoxelType::NoMesh(_),
                ..
            } => VoxelType::NoMesh(rand::random()),
            BlockType::Voxel { voxel_type, .. } => *voxel_type,
            BlockType::Simple { .. } => VoxelType::Translucent(self.id),
            BlockType::Complex { .. } => VoxelType::NoMesh(rand::random()),
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

    /// Get if the block has collision
    pub fn collision(&self) -> bool {
        match &self.block_type {
            BlockType::Voxel { .. } => true,
            BlockType::Simple { collision, .. } | BlockType::Complex { collision, .. } => {
                *collision
            }
        }
    }
}

impl Eq for Block {}
impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
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
        collision: bool,
    },
    /// A block that has it's own mesh
    #[allow(dead_code)]
    Complex {
        mesh: Mesh,
        textures: Vec<Handle<Image>>,
        collision: bool,
    },
}

impl BlockType {
    pub const fn new_voxel(voxel_type: VoxelType, texture: VoxelTexture) -> Self {
        Self::Voxel {
            voxel_type,
            texture,
        }
    }

    pub const fn new_simple(dimensions: [f32; 6], texture: VoxelTexture, collision: bool) -> Self {
        Self::Simple {
            texture,
            dimensions,
            collision,
        }
    }

    #[allow(dead_code)]
    pub const fn new_complex(mesh: Mesh, textures: Vec<Handle<Image>>, collision: bool) -> Self {
        Self::Complex {
            mesh,
            textures,
            collision,
        }
    }
}
