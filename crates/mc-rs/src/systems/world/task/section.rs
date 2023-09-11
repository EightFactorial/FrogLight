use std::fmt::Debug;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
};
use bevy_rapier3d::prelude::*;
use block_mesh::{
    ndshape::{ConstShape, ConstShape3u32},
    GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};
use mc_rs_proto::types::enums::Direction;

use crate::systems::{
    blocks::{
        state::{meshing::BlockMeshData, model::BlockModel, StatesMapFn},
        BlockData,
    },
    world::{
        material::{StateAnimation, ATTRIBUTE_ANIMATION_INDEX, ATTRIBUTE_TEXTURE_INDEX},
        CHUNK_SIZE, SECTION_HEIGHT,
    },
};

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
    pub animations: Vec<StateAnimation>,
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
            .field("animations", &self.animations.len())
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
    let blocks = block_data.blocks.read();
    let blockstates = block_data.states.read();

    let mut shape = [BlockMeshData::default(); MeshChunkShape::SIZE as usize];
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
                let blockstate = blockstates.get_state(state_id);
                let block = blockstate.get_block(&blocks);

                let shape_index = MeshChunkShape::linearize([x, y, z]) as usize;
                shape[shape_index] = BlockMeshData::from_state(blockstate, block);
            }
        }
    }

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    let mut buffer = GreedyQuadsBuffer::new(shape.len());
    block_mesh::greedy_quads(
        &shape,
        &MeshChunkShape {},
        [0; 3],
        [MESH_X - 1, MESH_Y - 1, MESH_Z - 1],
        &faces,
        &mut buffer,
    );

    // Skip the section if it has no quads
    if buffer.quads.num_quads() == 0 {
        return None;
    }

    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;

    // Terrain collider data
    let mut collider_indices = Vec::with_capacity(num_indices);
    let mut collider_positions = Vec::with_capacity(num_vertices);

    // Fluid collider data
    let mut fluid_indices = Vec::with_capacity(num_indices);
    let mut fluid_positions = Vec::with_capacity(num_vertices);

    // Opaque mesh data
    let mut opaque_indices = Vec::with_capacity(num_indices);
    let mut opaque_positions = Vec::with_capacity(num_vertices);
    let mut opaque_normals = Vec::with_capacity(num_vertices);
    let mut opaque_tex_uvs = Vec::with_capacity(num_vertices);

    let mut opaque_tex_index = Vec::with_capacity(num_vertices);
    let mut opaque_textures: Vec<Handle<Image>> = Vec::with_capacity(8);

    let mut opaque_anim_index = Vec::with_capacity(num_vertices);
    let mut opaque_animations: Vec<StateAnimation> = Vec::new();

    // Transparent mesh data
    let mut trans_indices = Vec::with_capacity(num_indices);
    let mut trans_positions = Vec::with_capacity(num_vertices);
    let mut trans_normals = Vec::with_capacity(num_vertices);
    let mut trans_tex_uvs = Vec::with_capacity(num_vertices);

    let mut trans_tex_index = Vec::with_capacity(num_vertices);
    let mut trans_textures: Vec<Handle<Image>> = Vec::with_capacity(8);

    let mut trans_anim_index = Vec::with_capacity(num_vertices);
    let mut trans_animations: Vec<StateAnimation> = Vec::new();

    for (group, face) in buffer.quads.groups.into_iter().zip(faces) {
        let direction = Direction::from(face.signed_normal().to_array());
        let norm = face.quad_mesh_normals();

        for quad in group.into_iter() {
            // Get the blockstate data
            let [x, y, z] = quad.minimum.map(|i| i - 1);
            let state_id = section_data[ChunkShape::linearize([x, z, y]) as usize];
            let blockstate = blockstates.get_state(&state_id);

            // Get the block data
            let block = blockstate.get_block(&blocks);
            let prop = &block.properties;

            // Skip the block if it has no model
            if matches!(blockstate.model, BlockModel::None) {
                continue;
            }

            // Get the quad mesh positions
            let mut pos = face.quad_mesh_positions(&quad, 1.0);
            blockstate.model.mod_mesh_positions(&direction, &mut pos);

            // Add the block to the terrain collider
            if prop.collidable {
                collider_indices.extend(face.quad_mesh_indices(collider_positions.len() as u32));
                collider_positions.extend_from_slice(&pos);
            }

            // Add the block to the fluid collider
            if prop.is_fluid {
                fluid_indices.extend(face.quad_mesh_indices(fluid_positions.len() as u32));
                fluid_positions.extend_from_slice(&pos);
            }

            // Determine the block model
            match &blockstate.model {
                BlockModel::Custom { mesh: _mesh, .. } => {
                    // TODO: Insert texture data into the texture map
                    // TODO: Append the blockstate mesh data to the terrain mesh
                }
                _ => {
                    // Get the blockface texture
                    let texture = match blockstate.textures.get_texture(&direction) {
                        Some(texture) => texture.clone(),
                        None => {
                            error!(
                                "Block {}:{state_id} has no texture for face {direction:?}",
                                block.name
                            );
                            Handle::<Image>::default()
                        }
                    };

                    let uvs = face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad);

                    match prop.opaque {
                        // Add the block to the opaque mesh
                        true => {
                            // Get the texture index or insert it into the textures list
                            let tex_index = match opaque_textures.iter().position(|p| p == &texture)
                            {
                                Some(index) => index as u32,
                                None => {
                                    opaque_textures.push(texture);
                                    opaque_textures.len() as u32 - 1
                                }
                            };

                            // Get the animation index or insert it into the animations list
                            let anim_index = match blockstate.textures.get_animation(&direction) {
                                None => 0,
                                Some(anim) => {
                                    match opaque_animations.iter().position(|a| a == anim) {
                                        Some(index) => index as u32,
                                        None => {
                                            opaque_animations.push(*anim);
                                            opaque_animations.len() as u32
                                        }
                                    }
                                }
                            };

                            opaque_indices
                                .extend(face.quad_mesh_indices(opaque_positions.len() as u32));
                            opaque_positions.extend(pos);
                            opaque_normals.extend(norm);
                            opaque_tex_uvs.extend(uvs);
                            opaque_anim_index.extend([anim_index; 4]);
                            opaque_tex_index.extend([tex_index; 4]);
                        }
                        // Add the block to the transparent mesh
                        false => {
                            // Get the texture index or insert it into the textures list
                            let tex_index = match trans_textures.iter().position(|p| p == &texture)
                            {
                                Some(index) => index as u32,
                                None => {
                                    trans_textures.push(texture);
                                    trans_textures.len() as u32 - 1
                                }
                            };

                            // Get the animation index or insert it into the animations list
                            let anim_index = match blockstate.textures.get_animation(&direction) {
                                None => 0,
                                Some(anim) => match trans_animations.iter().position(|a| a == anim)
                                {
                                    Some(index) => index as u32,
                                    None => {
                                        trans_animations.push(*anim);
                                        trans_animations.len() as u32
                                    }
                                },
                            };

                            trans_indices
                                .extend(face.quad_mesh_indices(trans_positions.len() as u32));
                            trans_positions.extend(pos);
                            trans_normals.extend(norm);
                            trans_tex_uvs.extend(uvs);
                            trans_anim_index.extend([anim_index; 4]);
                            trans_tex_index.extend([tex_index; 4]);
                        }
                    }
                }
            }
        }
    }

    // Create the meshes
    let opaque = match opaque_indices.is_empty() {
        true => None,
        false => {
            let mut opaque_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            opaque_mesh.set_indices(Some(Indices::U32(opaque_indices)));

            opaque_mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float32x3(opaque_positions),
            );
            opaque_mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float32x3(opaque_normals),
            );
            opaque_mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                VertexAttributeValues::Float32x2(opaque_tex_uvs),
            );
            opaque_mesh.insert_attribute(
                ATTRIBUTE_ANIMATION_INDEX,
                VertexAttributeValues::Uint32(opaque_anim_index),
            );
            opaque_mesh.insert_attribute(
                ATTRIBUTE_TEXTURE_INDEX,
                VertexAttributeValues::Uint32(opaque_tex_index),
            );

            Some(MeshData {
                mesh: opaque_mesh,
                textures: opaque_textures,
                animations: opaque_animations,
            })
        }
    };

    let transparent = match trans_indices.is_empty() {
        true => None,
        false => {
            let mut trans_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            trans_mesh.set_indices(Some(Indices::U32(trans_indices)));

            trans_mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float32x3(trans_positions),
            );
            trans_mesh.insert_attribute(
                Mesh::ATTRIBUTE_NORMAL,
                VertexAttributeValues::Float32x3(trans_normals),
            );
            trans_mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                VertexAttributeValues::Float32x2(trans_tex_uvs),
            );
            trans_mesh.insert_attribute(
                ATTRIBUTE_ANIMATION_INDEX,
                VertexAttributeValues::Uint32(trans_anim_index),
            );
            trans_mesh.insert_attribute(
                ATTRIBUTE_TEXTURE_INDEX,
                VertexAttributeValues::Uint32(trans_tex_index),
            );

            Some(MeshData {
                mesh: trans_mesh,
                textures: trans_textures,
                animations: trans_animations,
            })
        }
    };

    // Create the colliders
    let terrain_collider = match collider_indices.is_empty() {
        true => None,
        false => {
            let mut collision_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            collision_mesh.set_indices(Some(Indices::U32(collider_indices)));
            collision_mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float32x3(collider_positions),
            );

            Collider::from_bevy_mesh(&collision_mesh, &ComputedColliderShape::TriMesh)
        }
    };

    let fluid_collider = match fluid_indices.is_empty() {
        true => None,
        false => {
            let mut fluid_mesh = Mesh::new(PrimitiveTopology::TriangleList);
            fluid_mesh.set_indices(Some(Indices::U32(fluid_indices)));
            fluid_mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                VertexAttributeValues::Float32x3(fluid_positions),
            );

            Collider::from_bevy_mesh(&fluid_mesh, &ComputedColliderShape::TriMesh)
        }
    };

    Some(SectionData {
        opaque,
        transparent,
        terrain_collider,
        fluid_collider,
    })
}
