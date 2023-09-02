use std::fmt::Debug;

use bevy::{
    self,
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, RigidBody, Sensor, Sleeping};
use block_mesh::{
    greedy_quads,
    ndshape::{ConstShape, ConstShape3u32},
    GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};
use futures_lite::future::{block_on, poll_once};
use mc_rs_proto::types::enums::Direction;

use crate::systems::{
    app_state::GameSet,
    blocks::{
        state::{meshing::BlockMeshData, model::BlockModel, StatesMapFn},
        BlockStates, Blocks,
    },
};

use super::{
    chunk::ChunkSections,
    material::{BlockMaterial, ATTRIBUTE_BLOCK_ID, ATTRIBUTE_TEXTURE_INDEX, MAX_TEXTURE_COUNT},
    section::SectionComponent,
    CHUNK_SIZE, SECTION_COUNT, SECTION_HEIGHT,
};

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Update,
        ChunkTask::poll_tasks
            .run_if(ChunkTask::any_tasks_finished)
            .in_set(GameSet),
    );
}

/// A task that generates a mesh for a section
#[derive(Deref, DerefMut, Component)]
pub struct ChunkTask(pub Task<ChunkTaskResult>);
type ChunkTaskResult = Vec<SectionResult>;

impl ChunkTask {
    pub(super) fn create(
        chunk: ChunkSections,
        neighbors: [Option<ChunkSections>; 4],
        blocks: Blocks,
        blockstates: BlockStates,
    ) -> Self {
        ChunkTask(AsyncComputeTaskPool::get().spawn(chunk_fn(
            chunk,
            neighbors,
            blocks,
            blockstates,
        )))
    }

    fn as_task(&mut self) -> &mut Task<ChunkTaskResult> { &mut self.0 }

    fn any_tasks_finished(mut query: Query<&ChunkTask>) -> bool {
        query.iter_mut().any(|task| task.is_finished())
    }

    pub(super) fn poll_tasks(
        mut query: Query<(Entity, &mut ChunkTask)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<BlockMaterial>>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(results) = block_on(poll_once(task.as_task())) {
                commands
                    .entity(entity)
                    // Remove all children
                    .despawn_descendants()
                    // Add the new children
                    .with_children(|parent| {
                        for (index, option) in results.into_iter().enumerate() {
                            if let Some(result) = option {
                                let SectionData {
                                    opaque_mesh,
                                    transparent_mesh,
                                    terrain_collider,
                                    fluid_collider,
                                    textures,
                                } = result;

                                let transform =
                                    Transform::from_xyz(0.0, (index * SECTION_HEIGHT) as f32, 0.0);

                                // Create the opaque mesh bundle
                                if let Some(opaque_mesh) = opaque_mesh {
                                    parent.spawn((
                                        SectionComponent,
                                        MaterialMeshBundle::<BlockMaterial> {
                                            mesh: meshes.add(opaque_mesh),
                                            material: materials
                                                .add(BlockMaterial::new(textures.clone())),
                                            transform,
                                            ..Default::default()
                                        },
                                    ));
                                }

                                // Create the transparent mesh bundle
                                if let Some(transparent_mesh) = transparent_mesh {
                                    parent.spawn((
                                        SectionComponent,
                                        MaterialMeshBundle::<BlockMaterial> {
                                            mesh: meshes.add(transparent_mesh),
                                            material: materials
                                                .add(BlockMaterial::new_blended(textures)),
                                            transform,
                                            ..Default::default()
                                        },
                                    ));
                                }

                                // Spawn the terrain collider
                                if let Some(terrain_collider) = terrain_collider {
                                    parent.spawn((
                                        SectionComponent,
                                        terrain_collider,
                                        RigidBody::Fixed,
                                        Sleeping {
                                            sleeping: true,
                                            ..Default::default()
                                        },
                                        TransformBundle::from_transform(transform),
                                    ));
                                }

                                // Spawn the fluid collider
                                if let Some(fluid_collider) = fluid_collider {
                                    parent.spawn((
                                        SectionComponent,
                                        fluid_collider,
                                        RigidBody::Fixed,
                                        Sensor,
                                        Sleeping {
                                            sleeping: true,
                                            ..Default::default()
                                        },
                                        TransformBundle::from_transform(transform),
                                    ));
                                }
                            }
                        }
                    })
                    .remove::<ChunkTask>();
            };
        }
    }
}

/// Generates a mesh for all sections in a chunk
async fn chunk_fn(
    chunk: ChunkSections,
    neighbors: [Option<ChunkSections>; 4],
    blocks: Blocks,
    blockstates: BlockStates,
) -> ChunkTaskResult {
    let pool = AsyncComputeTaskPool::get();

    let mut results = Vec::with_capacity(SECTION_COUNT);
    let mut tasks = Vec::with_capacity(SECTION_COUNT);

    for index in 0..SECTION_COUNT {
        // If the section is empty, skip it
        if let Some(chunk) = chunk.read().get(index) {
            if chunk.block_count == 0 {
                tasks.push(None);
                continue;
            }
        }

        let neighbors = [
            neighbors[0].as_ref().map(|c| c.read()[index].get_blocks()),
            neighbors[1].as_ref().map(|c| c.read()[index].get_blocks()),
            neighbors[2].as_ref().map(|c| c.read()[index].get_blocks()),
            neighbors[3].as_ref().map(|c| c.read()[index].get_blocks()),
            if index > 0 {
                Some(chunk.read()[index - 1].get_blocks())
            } else {
                None
            },
            if index < SECTION_COUNT - 1 {
                Some(chunk.read()[index + 1].get_blocks())
            } else {
                None
            },
        ];

        // Spawn a new thread for the section
        let task = Some(pool.spawn(section_fn(
            chunk.read()[index].get_blocks(),
            neighbors,
            blocks.clone(),
            blockstates.clone(),
        )));

        tasks.push(task);
    }

    // Wait for all sections to finish
    for task in tasks {
        match task {
            Some(task) => results.push(task.await),
            None => results.push(None),
        }
    }

    results
}

type SectionResult = Option<SectionData>;
pub struct SectionData {
    opaque_mesh: Option<Mesh>,
    transparent_mesh: Option<Mesh>,
    terrain_collider: Option<Collider>,
    fluid_collider: Option<Collider>,
    textures: Vec<Handle<Image>>,
}

impl Debug for SectionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SectionData")
            .field("opaque_mesh", &self.opaque_mesh.is_some())
            .field("transparent_mesh", &self.transparent_mesh.is_some())
            .field("terrain_collider", &self.terrain_collider.is_some())
            .field("fluid_collider", &self.fluid_collider.is_some())
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
async fn section_fn(
    section_data: Vec<u32>,
    neighbor_data: [Option<Vec<u32>>; 6],
    blocks: Blocks,
    blockstates: BlockStates,
) -> SectionResult {
    let blocks = blocks.read();
    let blockstates = blockstates.read();

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

                let shape_index = MeshChunkShape::linearize([x, y, z]) as usize;
                shape[shape_index] = blockstate.get_mesh_data(&blocks);
            }
        }
    }

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    let mut buffer = GreedyQuadsBuffer::new(shape.len());
    greedy_quads(
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
    let mut opaque_tex_ids = Vec::with_capacity(num_vertices);
    let mut opaque_tex_index = Vec::with_capacity(num_vertices);

    // Transparent mesh data
    let mut trans_indices = Vec::with_capacity(num_indices);
    let mut trans_positions = Vec::with_capacity(num_vertices);
    let mut trans_normals = Vec::with_capacity(num_vertices);
    let mut trans_tex_uvs = Vec::with_capacity(num_vertices);
    let mut trans_tex_ids = Vec::with_capacity(num_vertices);
    let mut trans_tex_index = Vec::with_capacity(num_vertices);

    let mut textures: Vec<Handle<Image>> = Vec::with_capacity(8);

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
                        Some(texture) => texture,
                        None => {
                            error!(
                                "Block {}:{state_id} has no texture for face {direction:?}",
                                block.name
                            );
                            Handle::<Image>::default()
                        }
                    };

                    // Get the texture index or insert it into the textures list
                    let tex_index = match textures.iter().position(|p| p == &texture) {
                        Some(index) => index as u32,
                        None => {
                            textures.push(texture);
                            textures.len() as u32 - 1
                        }
                    };

                    let uvs = face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad);

                    match prop.opaque {
                        // Add the block to the opaque mesh
                        true => {
                            opaque_indices
                                .extend(face.quad_mesh_indices(opaque_positions.len() as u32));
                            opaque_positions.extend(pos);
                            opaque_normals.extend(norm);
                            opaque_tex_uvs.extend(uvs);
                            opaque_tex_ids.extend([block.block_id; 4]);
                            opaque_tex_index.extend([tex_index; 4]);
                        }
                        // Add the block to the transparent mesh
                        false => {
                            trans_indices
                                .extend(face.quad_mesh_indices(trans_positions.len() as u32));
                            trans_positions.extend(pos);
                            trans_normals.extend(norm);
                            trans_tex_uvs.extend(uvs);
                            trans_tex_ids.extend([block.block_id; 4]);
                            trans_tex_index.extend([tex_index; 4]);
                        }
                    }
                }
            }
        }
    }

    // Create the meshes
    let opaque_mesh = match opaque_indices.is_empty() {
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
                ATTRIBUTE_BLOCK_ID,
                VertexAttributeValues::Uint32(opaque_tex_ids),
            );
            opaque_mesh.insert_attribute(
                ATTRIBUTE_TEXTURE_INDEX,
                VertexAttributeValues::Uint32(opaque_tex_index),
            );

            Some(opaque_mesh)
        }
    };

    let transparent_mesh = match trans_indices.is_empty() {
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
                ATTRIBUTE_BLOCK_ID,
                VertexAttributeValues::Uint32(trans_tex_ids),
            );
            trans_mesh.insert_attribute(
                ATTRIBUTE_TEXTURE_INDEX,
                VertexAttributeValues::Uint32(trans_tex_index),
            );

            Some(trans_mesh)
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

    if textures.len() > MAX_TEXTURE_COUNT {
        error!(
            "Section has {} textures, but the maximum is {}",
            textures.len(),
            MAX_TEXTURE_COUNT
        );
    }

    Some(SectionData {
        opaque_mesh,
        transparent_mesh,
        terrain_collider,
        fluid_collider,
        textures,
    })
}
