use froglight_macros::FrogTest;
use glam::{DVec3, Vec3};

use crate::{
    common::{BlockPosition, Direction},
    protocol::{FrogRead, FrogWrite, ReadError, WriteError},
};

/// A hit on a block
#[derive(Debug, Clone, Copy, PartialEq, FrogTest)]
#[frog(tests = ["read_example"], bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])]
pub struct BlockHit {
    /// The position of the block that was hit
    pub block_position: BlockPosition,
    /// The direction the block was hit from
    pub hit_direction: Direction,
    /// The position of the hit
    pub hit_position: DVec3,
    /// Whether the hit was from inside the block
    pub inside: bool,
}

impl FrogRead for BlockHit {
    fn fg_read(buf: &mut std::io::Cursor<&[u8]>) -> Result<Self, ReadError>
    where
        Self: Sized,
    {
        let block_position = BlockPosition::fg_read(buf)?;
        let hit_direction = Direction::fg_read(buf)?;

        // Convert relative hit position to absolute hit position
        let relative_position = Vec3::fg_read(buf)?;
        let hit_position = relative_position.as_dvec3() + block_position.as_dvec3();

        Ok(Self { block_position, hit_direction, hit_position, inside: bool::fg_read(buf)? })
    }
}

impl FrogWrite for BlockHit {
    fn fg_write(&self, buf: &mut (impl std::io::Write + ?Sized)) -> Result<(), WriteError> {
        self.block_position.fg_write(buf)?;
        self.hit_direction.fg_write(buf)?;

        // Convert absolute hit position to relative hit position
        let relative_position = self.hit_position - self.block_position.as_dvec3();
        relative_position.as_vec3().fg_write(buf)?;

        self.inside.fg_write(buf)
    }
}
