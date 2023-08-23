use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

/// The type of voxel the block is
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VoxelType {
    /// An empty voxel
    Empty,
    /// A voxel with geometry and light can pass through
    Translucent(u32),
    /// A voxel with geometry
    Opaque(u32),
    /// A voxel that does not mesh and light can pass through
    ///
    /// The value is a random number used to differentiate between blocks
    NoMesh(i32),
}

impl Voxel for VoxelType {
    fn get_visibility(&self) -> VoxelVisibility {
        match self {
            VoxelType::Empty => VoxelVisibility::Empty,
            VoxelType::Translucent(_) | VoxelType::NoMesh(_) => VoxelVisibility::Translucent,
            VoxelType::Opaque(_) => VoxelVisibility::Opaque,
        }
    }
}

impl MergeVoxel for VoxelType {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue { *self }
}
