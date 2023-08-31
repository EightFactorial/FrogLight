use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

use crate::systems::blocks::block::Block;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BlockMeshData {
    pub voxel: BlockVoxel,
    pub meshing: BlockMesh,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum BlockMesh {
    #[default]
    Never,
    Always,
    Custom([bool; 6]),
}

impl Eq for BlockMesh {}
impl PartialEq for BlockMesh {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Never, _) | (_, Self::Never) => false,
            (Self::Always, _) | (_, Self::Always) => true,
            (Self::Custom([l0, l1, l2, l3, l4, l5]), Self::Custom([r0, r1, r2, r3, r4, r5])) => {
                // Sides are in order: -Y, +Y, -Z, +Z, -X, +X
                // True if any of the opposite sides are equal
                l0 == r1 || l1 == r0 || l2 == r3 || l3 == r2 || l4 == r5 || l5 == r4
            }
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
