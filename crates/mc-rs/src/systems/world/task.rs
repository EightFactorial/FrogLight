use std::sync::Arc;

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

use crate::systems::{
    app_state::GameSet,
    blocks::{BlockStates, Blocks},
};

use super::{
    chunk::ChunkSections,
    material::{BlockMaterial, ATTRIBUTE_BLOCK_ID, ATTRIBUTE_TEXTURE_INDEX},
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
                                let SectionTaskResult {
                                    opaque_mesh,
                                    transparent_mesh,
                                    terrain_collider,
                                    fluid_collider,
                                    textures,
                                } = result;

                                // Create the opaque mesh bundle
                                let opaque_material = MaterialMeshBundle::<BlockMaterial> {
                                    mesh: meshes.add(opaque_mesh),
                                    material: materials.add(BlockMaterial::new(textures.clone())),
                                    transform: Transform::from_xyz(
                                        0.,
                                        (index * SECTION_HEIGHT) as f32,
                                        0.,
                                    ),
                                    ..Default::default()
                                };

                                // Create the transparent mesh bundle
                                let transparent_material = MaterialMeshBundle::<BlockMaterial> {
                                    mesh: meshes.add(transparent_mesh),
                                    material: materials.add(BlockMaterial::new_blend(textures)),
                                    transform: Transform::from_xyz(
                                        0.,
                                        (index * SECTION_HEIGHT) as f32,
                                        0.,
                                    ),
                                    ..Default::default()
                                };

                                // Spawn the terrain
                                parent.spawn((SectionComponent, opaque_material));
                                parent.spawn((SectionComponent, transparent_material));

                                // Spawn the terrain collider
                                parent.spawn((
                                    SectionComponent,
                                    terrain_collider,
                                    RigidBody::Fixed,
                                    Sleeping {
                                        sleeping: true,
                                        ..Default::default()
                                    },
                                ));

                                // Spawn the fluid collider
                                parent.spawn((
                                    SectionComponent,
                                    fluid_collider,
                                    RigidBody::Fixed,
                                    Sensor,
                                    Sleeping {
                                        sleeping: true,
                                        ..Default::default()
                                    },
                                ));
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

        let section_data = chunk.read()[index].get_blocks();

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
            section_data,
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

type SectionResult = Option<SectionTaskResult>;
pub struct SectionTaskResult {
    opaque_mesh: Mesh,
    transparent_mesh: Mesh,
    terrain_collider: Collider,
    fluid_collider: Collider,
    textures: Vec<Handle<Image>>,
}

const X: u32 = CHUNK_SIZE as u32;
const Y: u32 = SECTION_HEIGHT as u32;
const Z: u32 = CHUNK_SIZE as u32;
type ChunkShape = ConstShape3u32<X, Y, Z>;

const MESH_X: u32 = X + 2;
const MESH_Y: u32 = Y + 2;
const MESH_Z: u32 = Z + 2;
type MeshChunkShape = ConstShape3u32<MESH_X, MESH_Y, MESH_Z>;

static _EMPTY_ID: u32 = 0;

/// Generates a mesh for a section
async fn section_fn(
    _section_data: Vec<u32>,
    _neighbors: [Option<Vec<u32>>; 6],
    blocks: Blocks,
    blockstates: BlockStates,
) -> SectionResult {
    let blocks = blocks.read();
    let blockstates = blockstates.read();

    let mut textures = Vec::new();

    todo!("Write a custom greedy meshing algorithm");

    let mut shape = [0u8; MeshChunkShape::SIZE as usize];
    for y in 0..MESH_Y {
        for z in 0..MESH_Z {
            for x in 0..MESH_X {
                // Ignore all corners
                if [
                    (x == 0 || x == MESH_X - 1),
                    (y == 0 || y == MESH_Y - 1),
                    (z == 0 || z == MESH_Z - 1),
                ]
                .iter()
                .fold(0u8, |acc, f| acc + *f as u8)
                    > 1
                {
                    continue;
                }

                // TODO
            }
        }
    }

    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    let mut buffer = GreedyQuadsBuffer::new(shape.len());
    // greedy_quads(
    //     &shape,
    //     &MeshChunkShape {},
    //     [0; 3],
    //     [MESH_X - 1, MESH_Y - 1, MESH_Z - 1],
    //     &faces,
    //     &mut buffer,
    // );

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

    for (group, face) in buffer.quads.groups.into_iter().zip(faces) {
        for quad in group.into_iter() {
            // TODO: Insert quad data into the appropriate buffers
        }
    }

    // Create the meshes
    let opaque_mesh = {
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

        opaque_mesh
    };

    let transparent_mesh = {
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

        trans_mesh
    };

    // Create the colliders
    let terrain_collider = {
        let mut collision_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        collision_mesh.set_indices(Some(Indices::U32(collider_indices)));
        collision_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(collider_positions),
        );

        Collider::from_bevy_mesh(&collision_mesh, &ComputedColliderShape::TriMesh)
    }?;

    let fluid_collider = {
        let mut collision_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        collision_mesh.set_indices(Some(Indices::U32(fluid_indices)));
        collision_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(fluid_positions),
        );

        Collider::from_bevy_mesh(&collision_mesh, &ComputedColliderShape::TriMesh)
    }?;

    Some(SectionTaskResult {
        opaque_mesh,
        transparent_mesh,
        terrain_collider,
        fluid_collider,
        textures,
    })
}

// /// Modifies the quad positions to fit the block
// fn mod_quad(direction: &Direction, pos: &mut [[f32; 3]; 4], dim: &[f32; 6]) {
//     let [min_x, min_y, min_z, max_x, max_y, max_z] = dim;

//     match direction {
//         Direction::Up => {
//             pos[0][0] += *min_x;
//             pos[2][0] += *min_x;
//             pos[1][0] -= 1. - *max_x;
//             pos[3][0] -= 1. - *max_x;

//             pos[0][1] -= 1. - *max_y;
//             pos[1][1] -= 1. - *max_y;
//             pos[2][1] -= 1. - *max_y;
//             pos[3][1] -= 1. - *max_y;

//             pos[0][2] += *min_z;
//             pos[1][2] += *min_z;
//             pos[2][2] -= 1. - *max_z;
//             pos[3][2] -= 1. - *max_z;
//         }
//         Direction::Down => {
//             pos[0][0] += *min_x;
//             pos[2][0] += *min_x;
//             pos[1][0] -= 1. - *max_x;
//             pos[3][0] -= 1. - *max_x;

//             pos[0][1] += *min_y;
//             pos[1][1] += *min_y;
//             pos[2][1] += *min_y;
//             pos[3][1] += *min_y;

//             pos[0][2] += *min_z;
//             pos[1][2] += *min_z;
//             pos[2][2] -= 1. - *max_z;
//             pos[3][2] -= 1. - *max_z;
//         }
//         Direction::North => {
//             pos[0][0] += *min_x;
//             pos[1][0] += *min_x;
//             pos[2][0] += *min_x;
//             pos[3][0] += *min_x;

//             pos[0][1] += *min_y;
//             pos[1][1] += *min_y;
//             pos[2][1] -= 1. - *max_y;
//             pos[3][1] -= 1. - *max_y;

//             pos[0][2] += *min_z;
//             pos[3][2] += *min_z;
//             pos[1][2] -= 1. - *max_z;
//             pos[2][2] -= 1. - *max_z;
//         }
//         Direction::South => {
//             pos[0][0] += *min_x;
//             pos[1][0] += *min_x;
//             pos[2][0] += *min_x;
//             pos[3][0] += *min_x;

//             pos[0][1] += *min_y;
//             pos[1][1] += *min_y;
//             pos[2][1] -= 1. - *max_y;
//             pos[3][1] -= 1. - *max_y;

//             pos[0][2] -= 1. - *max_z;
//             pos[3][2] -= 1. - *max_z;
//             pos[1][2] += *min_z;
//             pos[2][2] += *min_z;
//         }
//         Direction::East => {
//             pos[0][0] += *min_x;
//             pos[2][0] += *min_x;
//             pos[1][0] -= 1. - *max_x;
//             pos[3][0] -= 1. - *max_x;

//             pos[0][1] += *min_y;
//             pos[1][1] += *min_y;
//             pos[2][1] -= 1. - *max_y;
//             pos[3][1] -= 1. - *max_y;

//             pos[0][2] += *min_z;
//             pos[1][2] += *min_z;
//             pos[2][2] += *min_z;
//             pos[3][2] += *min_z;
//         }
//         Direction::West => {
//             pos[0][0] += *min_x;
//             pos[2][0] += *min_x;
//             pos[1][0] -= 1. - *max_x;
//             pos[3][0] -= 1. - *max_x;

//             pos[0][1] += *min_y;
//             pos[1][1] += *min_y;
//             pos[2][1] -= 1. - *max_y;
//             pos[3][1] -= 1. - *max_y;

//             pos[0][2] -= 1. - *max_z;
//             pos[1][2] -= 1. - *max_z;
//             pos[2][2] -= 1. - *max_z;
//             pos[3][2] -= 1. - *max_z;
//         }
//     }
// }
