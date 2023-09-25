use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

use crate::systems::blocks::block::Block;

use super::{model::BlockModel, BlockState};

#[derive(Debug, Clone, Copy)]
pub struct BlockMeshData {
    pub rid: u32,
    pub voxel: BlockVoxel,
    pub meshing: BlockMesh,
}

impl Default for BlockMeshData {
    fn default() -> Self {
        Self {
            rid: rand::random(),
            voxel: Default::default(),
            meshing: Default::default(),
        }
    }
}

impl Eq for BlockMeshData {}
impl PartialEq for BlockMeshData {
    fn eq(&self, other: &Self) -> bool {
        if self.rid == other.rid {
            true
        } else {
            self.voxel == other.voxel && self.meshing == other.meshing
        }
    }
}

impl BlockMeshData {
    pub fn from_state(state: &BlockState, block: &Block) -> BlockMeshData {
        BlockMeshData {
            rid: rand::random(),
            voxel: block.into(),
            meshing: match &state.model {
                BlockModel::Standard => BlockMesh::Always,
                _ => BlockMesh::Never,
            },
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum BlockMesh {
    #[default]
    Never,
    Always,
}

impl Eq for BlockMesh {}
impl PartialEq for BlockMesh {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Never, _) | (_, Self::Never) => false,
            (Self::Always, Self::Always) => true,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BlockVoxel {
    Opaque(u32),
    Translucent(u32),
    #[default]
    Empty,
}

impl From<&Block> for BlockVoxel {
    fn from(value: &Block) -> Self {
        if value.properties.is_air {
            return BlockVoxel::Empty;
        }

        match value.properties.opaque {
            true => BlockVoxel::Opaque(value.block_id),
            false => BlockVoxel::Translucent(value.block_id),
        }
    }
}

impl Voxel for BlockMeshData {
    fn get_visibility(&self) -> VoxelVisibility {
        match self.voxel {
            BlockVoxel::Opaque(_) => VoxelVisibility::Opaque,
            BlockVoxel::Translucent(_) => VoxelVisibility::Translucent,
            BlockVoxel::Empty => VoxelVisibility::Empty,
        }
    }
}

impl MergeVoxel for BlockMeshData {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue { *self }
}
