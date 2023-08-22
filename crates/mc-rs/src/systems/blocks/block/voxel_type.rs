use std::hash::Hash;

use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

/// The type of voxel the block is
#[derive(Debug, Clone, Copy)]
pub enum VoxelType {
    /// An empty voxel
    Empty,
    /// A voxel with geometry and light can pass through
    Translucent(u32),
    /// A voxel with geometry
    Opaque(u32),
    /// TODO: A voxel that does not mesh and light can pass through
    NoMesh,
}

impl Eq for VoxelType {}
impl PartialEq for VoxelType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Translucent(l0), Self::Translucent(r0)) => l0 == r0,
            (Self::Opaque(l0), Self::Opaque(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Hash for VoxelType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Voxel for VoxelType {
    fn get_visibility(&self) -> VoxelVisibility {
        match self {
            VoxelType::Empty => VoxelVisibility::Empty,
            VoxelType::Translucent(_) | VoxelType::NoMesh => VoxelVisibility::Translucent,
            VoxelType::Opaque(_) => VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for VoxelType {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue { *self }
}
