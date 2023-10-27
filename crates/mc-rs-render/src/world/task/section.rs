use std::fmt::Debug;

use bevy::{self, prelude::*};
use bevy_rapier3d::prelude::*;
use mc_rs_core::{
    blocks::BlockData,
    world::{CHUNK_SIZE, SECTION_HEIGHT},
};
use ndshape::{ConstShape, ConstShape3u32};

pub(super) type SectionResult = Option<SectionData>;
pub struct SectionData {
    pub opaque: Option<MeshData>,
    pub transparent: Option<MeshData>,
    pub terrain_collider: Option<Collider>,
    pub fluid_collider: Option<Collider>,
}

pub struct MeshData {
    pub mesh: Mesh,
    pub textures: Vec<Handle<Image>>,
}

impl Debug for SectionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SectionData")
            .field("opaque_mesh", &self.opaque.is_some())
            .field("transparent_mesh", &self.transparent.is_some())
            .field("terrain_collider", &self.terrain_collider.is_some())
            .field("fluid_collider", &self.fluid_collider.is_some())
            .finish()
    }
}

impl Debug for MeshData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MeshData")
            .field("textures", &self.textures.len())
            .finish()
    }
}

const X: u32 = CHUNK_SIZE as u32;
const Y: u32 = SECTION_HEIGHT as u32;
const Z: u32 = CHUNK_SIZE as u32;
type ChunkShape = ConstShape3u32<X, Y, Z>;

const MESH_X: u32 = X + 2;
const MESH_Y: u32 = Y + 2;
const MESH_Z: u32 = Z + 2;
type MeshChunkShape = ConstShape3u32<MESH_X, MESH_Y, MESH_Z>;

static EMPTY_ID: u32 = 0;

macro_rules! get_mesh_blockstate {
    ($x:expr, $y:expr, $z:expr, $data:expr, $n_data:expr) => {
        match ($x, $z, $y) {
            (0, _, _) => get_mesh_blockstate!($n_data[0], [X - 1, $z - 1, $y - 1]),
            (_, 0, _) => get_mesh_blockstate!($n_data[2], [$x - 1, Z - 1, $y - 1]),
            (_, _, 0) => get_mesh_blockstate!($n_data[4], [$x - 1, $z - 1, Y - 1]),
            (17, _, _) => get_mesh_blockstate!($n_data[1], [0, $z - 1, $y - 1]),
            (_, 17, _) => get_mesh_blockstate!($n_data[3], [$x - 1, 0, $y - 1]),
            (_, _, 17) => get_mesh_blockstate!($n_data[5], [$x - 1, $z - 1, 0]),
            _ => &$data[ChunkShape::linearize([$x - 1, $z - 1, $y - 1]) as usize],
        }
    };
    ($data:expr, $index:expr) => {
        match &$data {
            Some(data) => &data[ChunkShape::linearize($index) as usize],
            None => &EMPTY_ID,
        }
    };
}

/// Generates a mesh for a section
// TODO: Write custom greedy meshing algorithm to properly handle non-cubic blocks
pub(super) async fn section_fn(
    section_data: Vec<u32>,
    neighbor_data: [Option<Vec<u32>>; 6],
    block_data: BlockData,
) -> SectionResult {
    let _blocks = block_data.blocks.read();

    let mut shape = [0u32; MeshChunkShape::SIZE as usize];
    for y in 0..MESH_Y {
        for z in 0..MESH_Z {
            for x in 0..MESH_X {
                // Ignore all corners
                if [
                    (x == 0 || x == MESH_X - 1),
                    (y == 0 || y == MESH_Y - 1),
                    (z == 0 || z == MESH_Z - 1),
                ]
                .into_iter()
                .fold(0u8, |acc, f| acc + f as u8)
                    > 1
                {
                    continue;
                }

                let state_id = get_mesh_blockstate!(x, y, z, section_data, neighbor_data);
                let shape_index = MeshChunkShape::linearize([x, y, z]) as usize;

                shape[shape_index] = *state_id;
            }
        }
    }

    todo!("Write custom greedy meshing algorithm to properly handle non-cubic blocks")
}
