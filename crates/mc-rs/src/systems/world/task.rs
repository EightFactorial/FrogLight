use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    tasks::{AsyncComputeTaskPool, Task},
    utils::HashMap,
};
use bevy_rapier3d::prelude::{Collider, ComputedColliderShape, RigidBody, Sleeping};
use block_mesh::{
    greedy_quads,
    ndshape::{ConstShape, ConstShape3u32},
    GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};
use futures_lite::future::{block_on, poll_once};
use itertools::Itertools;
use mc_rs_proto::types::enums::Direction;

use crate::systems::{
    app_state::GameSet,
    blocks::{
        block::{voxel_type::VoxelType, BlockType},
        list::Blocks,
    },
};

use super::{
    chunk::ChunkSections,
    material::{BindlessMaterial, ATTRIBUTE_ANIMATION_INFO, ATTRIBUTE_TEXTURE_INDEX},
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
    ) -> Self {
        ChunkTask(AsyncComputeTaskPool::get().spawn(chunk_fn(chunk, neighbors, blocks)))
    }

    fn as_task(&mut self) -> &mut Task<ChunkTaskResult> { &mut self.0 }

    fn any_tasks_finished(mut query: Query<&ChunkTask>) -> bool {
        query.iter_mut().any(|task| task.is_finished())
    }

    pub(super) fn poll_tasks(
        mut query: Query<(Entity, &mut ChunkTask)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<BindlessMaterial>>,
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
                            if let Some((mesh, textures, collider)) = option {
                                // Create the material mesh bundle
                                let material = MaterialMeshBundle::<BindlessMaterial> {
                                    mesh: meshes.add(mesh),
                                    material: materials.add(BindlessMaterial::new(textures)),
                                    transform: Transform::from_xyz(
                                        0.,
                                        (index * SECTION_HEIGHT) as f32,
                                        0.,
                                    ),
                                    ..Default::default()
                                };

                                // Spawn a section
                                parent.spawn((
                                    RigidBody::Fixed,
                                    Sleeping {
                                        sleeping: true,
                                        ..Default::default()
                                    },
                                    collider,
                                    SectionComponent,
                                    material,
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
) -> ChunkTaskResult {
    let pool = AsyncComputeTaskPool::get();

    let mut results = Vec::with_capacity(SECTION_COUNT);
    let mut tasks = Vec::with_capacity(SECTION_COUNT);

    for index in 0..SECTION_COUNT {
        // If the section is empty, skip it
        if let Some(chunk) = chunk.read().unwrap().get(index) {
            if chunk.block_count == 0 {
                tasks.push(None);
                continue;
            }
        }

        let section_data = chunk.read().unwrap()[index].get_blocks();

        let neighbors = [
            neighbors[0]
                .as_ref()
                .map(|c| c.read().unwrap()[index].get_blocks()),
            neighbors[1]
                .as_ref()
                .map(|c| c.read().unwrap()[index].get_blocks()),
            neighbors[2]
                .as_ref()
                .map(|c| c.read().unwrap()[index].get_blocks()),
            neighbors[3]
                .as_ref()
                .map(|c| c.read().unwrap()[index].get_blocks()),
            if index > 0 {
                Some(chunk.read().unwrap()[index - 1].get_blocks())
            } else {
                None
            },
            if index < SECTION_COUNT - 1 {
                Some(chunk.read().unwrap()[index + 1].get_blocks())
            } else {
                None
            },
        ];

        // Spawn a new thread for the section
        let task = Some(pool.spawn(section_fn(section_data, neighbors, blocks.clone())));
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

type SectionResult = Option<(Mesh, SectionTextures, Collider)>;
type SectionTextures = Vec<Handle<Image>>;

const X: u32 = CHUNK_SIZE as u32;
const Y: u32 = SECTION_HEIGHT as u32;
const Z: u32 = CHUNK_SIZE as u32;
type ChunkShape = ConstShape3u32<X, Y, Z>;

const MESH_X: u32 = X + 2;
const MESH_Y: u32 = Y + 2;
const MESH_Z: u32 = Z + 2;
type MeshChunkShape = ConstShape3u32<MESH_X, MESH_Y, MESH_Z>;

static EMPTY_ID: u32 = 0;

/// Generates a mesh for a section
async fn section_fn(
    section_data: Vec<u32>,
    neighbors: [Option<Vec<u32>>; 6],
    blocks: Blocks,
) -> SectionResult {
    let blocks = blocks.read().unwrap();

    let mut textures = Vec::new();
    let mut texture_map = HashMap::new();
    for id in section_data.iter().unique() {
        let Some(block) = blocks.get(id) else {
            continue;
        };

        let Some(new_textures) = block.textures() else {
            continue;
        };

        let start = textures.len();
        textures.extend(new_textures.to_vec());
        texture_map.insert(*id, start);
    }

    // Insert the fallback block
    {
        let block = blocks.get(&u32::MAX).expect("Error getting fallback block");
        let start = textures.len();
        textures.extend(block.textures().unwrap().to_vec());
        texture_map.insert(u32::MAX, start);
    }

    let mut shape = [VoxelType::Empty; MeshChunkShape::SIZE as usize];
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

                // Get the index in the appropriate neighbor
                let block_id = match (x, z, y) {
                    (0, _, _) => {
                        if let Some(data) = &neighbors[0] {
                            &data[ChunkShape::linearize([X - 1, z - 1, y - 1]) as usize]
                        } else {
                            &EMPTY_ID
                        }
                    }
                    (_, 0, _) => {
                        if let Some(data) = &neighbors[2] {
                            &data[ChunkShape::linearize([x - 1, Z - 1, y - 1]) as usize]
                        } else {
                            &EMPTY_ID
                        }
                    }
                    (_, _, 0) => {
                        if let Some(data) = &neighbors[4] {
                            &data[ChunkShape::linearize([x - 1, z - 1, Y - 1]) as usize]
                        } else {
                            &EMPTY_ID
                        }
                    }
                    (17, _, _) => {
                        if let Some(data) = &neighbors[1] {
                            &data[ChunkShape::linearize([0, z - 1, y - 1]) as usize]
                        } else {
                            &EMPTY_ID
                        }
                    }
                    (_, 17, _) => {
                        if let Some(data) = &neighbors[3] {
                            &data[ChunkShape::linearize([x - 1, 0, y - 1]) as usize]
                        } else {
                            &EMPTY_ID
                        }
                    }
                    (_, _, 17) => {
                        if let Some(data) = &neighbors[5] {
                            &data[ChunkShape::linearize([x - 1, z - 1, 0]) as usize]
                        } else {
                            &EMPTY_ID
                        }
                    }
                    _ => &section_data[ChunkShape::linearize([x - 1, z - 1, y - 1]) as usize],
                };

                let block = blocks.get(block_id).unwrap_or(&blocks[&u32::MAX]);
                let shape_index = MeshChunkShape::linearize([x, y, z]) as usize;
                shape[shape_index] = block.voxel_type();
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

    if buffer.quads.num_quads() == 0 {
        return None;
    }

    let num_indices = buffer.quads.num_quads() * 6;
    let num_vertices = buffer.quads.num_quads() * 4;

    let mut collider_indices = Vec::with_capacity(num_vertices);
    let mut collider_positions = Vec::with_capacity(num_vertices);

    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    // let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_uvs = Vec::with_capacity(num_vertices);
    let mut tex_index = Vec::with_capacity(num_vertices);
    let mut tex_anim = Vec::with_capacity(num_vertices);

    for (group, face) in buffer.quads.groups.into_iter().zip(faces) {
        for quad in group.into_iter() {
            // Get the data
            let ind = face.quad_mesh_indices(positions.len() as u32);
            let mut pos = face.quad_mesh_positions(&quad, 1.0);
            // let norm = face.quad_mesh_normals();
            let mut uvs = face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad);

            // Get the block
            let [x, y, z] = quad.minimum;
            let data_index = ChunkShape::linearize([x - 1, z - 1, y - 1]) as usize;
            let block_id = &section_data[data_index];
            let block = blocks.get(block_id).unwrap_or(&blocks[&u32::MAX]);

            let direction = {
                let [x, y, z] = face.signed_normal().into();
                Direction::from([x, y, z])
            };

            // Rotate the uvs if it is facing up
            if direction == Direction::Up {
                for [u, v] in uvs.iter_mut() {
                    std::mem::swap(u, v);
                }
            }

            // Modify the quad
            match &block.block_type {
                BlockType::Simple { dimensions, .. } => {
                    mod_quad(&direction, &mut pos, dimensions);
                }
                BlockType::Complex { .. } => todo!("Append complex block mesh"),
                _ => {}
            }

            // Get the texture index
            let start = texture_map.get(block_id);
            let side_index = match &block.block_type {
                BlockType::Voxel { texture, .. } | BlockType::Simple { texture, .. } => {
                    texture.get_direction_index(&direction)
                }
                BlockType::Complex { .. } => todo!(),
            };
            let index = match (start, side_index) {
                (Some(start), Some(side_index)) => (start + side_index) as u32,
                _ => texture_map[&u32::MAX] as u32,
            };

            // If the block has collision, add it to the collider mesh
            if block.collision() {
                let col_ind = face.quad_mesh_indices(collider_positions.len() as u32);
                collider_indices.extend(col_ind);
                collider_positions.extend(pos);
            }

            // Get the texture animation information
            let anim = block.anim_info(&direction).unwrap_or([0, 0]);

            // Add the data to the mesh
            indices.extend(ind);
            positions.extend(pos);
            // normals.extend(norm);
            tex_uvs.extend(uvs);
            tex_index.extend([index; 4]);
            tex_anim.extend([anim; 4]);
        }
    }

    // Create the section mesh
    let render_mesh = {
        let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        render_mesh.set_indices(Some(Indices::U32(indices)));

        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        );
        // render_mesh.insert_attribute(
        //     Mesh::ATTRIBUTE_NORMAL,
        //     VertexAttributeValues::Float32x3(normals),
        // );
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(tex_uvs),
        );
        render_mesh.insert_attribute(
            ATTRIBUTE_TEXTURE_INDEX,
            VertexAttributeValues::Uint32(tex_index),
        );
        render_mesh.insert_attribute(
            ATTRIBUTE_ANIMATION_INFO,
            VertexAttributeValues::Uint32x2(tex_anim),
        );

        render_mesh
    };

    // Create the section collider
    let collider = {
        let mut collision_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        collision_mesh.set_indices(Some(Indices::U32(collider_indices)));
        collision_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(collider_positions),
        );

        Collider::from_bevy_mesh(&collision_mesh, &ComputedColliderShape::TriMesh)
    }?;

    Some((render_mesh, textures, collider))
}

/// Modifies the quad positions to fit the block
fn mod_quad(direction: &Direction, pos: &mut [[f32; 3]; 4], dim: &[f32; 6]) {
    let [min_x, min_y, min_z, max_x, max_y, max_z] = dim;

    match direction {
        Direction::Up => {
            pos[0][0] += *min_x;
            pos[2][0] += *min_x;
            pos[1][0] -= 1. - *max_x;
            pos[3][0] -= 1. - *max_x;

            pos[0][1] -= 1. - *max_y;
            pos[1][1] -= 1. - *max_y;
            pos[2][1] -= 1. - *max_y;
            pos[3][1] -= 1. - *max_y;

            pos[0][2] += *min_z;
            pos[1][2] += *min_z;
            pos[2][2] -= 1. - *max_z;
            pos[3][2] -= 1. - *max_z;
        }
        Direction::Down => {
            pos[0][0] += *min_x;
            pos[2][0] += *min_x;
            pos[1][0] -= 1. - *max_x;
            pos[3][0] -= 1. - *max_x;

            pos[0][1] += *min_y;
            pos[1][1] += *min_y;
            pos[2][1] += *min_y;
            pos[3][1] += *min_y;

            pos[0][2] += *min_z;
            pos[1][2] += *min_z;
            pos[2][2] -= 1. - *max_z;
            pos[3][2] -= 1. - *max_z;
        }
        Direction::North => {
            pos[0][0] += *min_x;
            pos[1][0] += *min_x;
            pos[2][0] += *min_x;
            pos[3][0] += *min_x;

            pos[0][1] += *min_y;
            pos[1][1] += *min_y;
            pos[2][1] -= 1. - *max_y;
            pos[3][1] -= 1. - *max_y;

            pos[0][2] += *min_z;
            pos[3][2] += *min_z;
            pos[1][2] -= 1. - *max_z;
            pos[2][2] -= 1. - *max_z;
        }
        Direction::South => {
            pos[0][0] += *min_x;
            pos[1][0] += *min_x;
            pos[2][0] += *min_x;
            pos[3][0] += *min_x;

            pos[0][1] += *min_y;
            pos[1][1] += *min_y;
            pos[2][1] -= 1. - *max_y;
            pos[3][1] -= 1. - *max_y;

            pos[0][2] -= 1. - *max_z;
            pos[3][2] -= 1. - *max_z;
            pos[1][2] += *min_z;
            pos[2][2] += *min_z;
        }
        Direction::East => {
            pos[0][0] += *min_x;
            pos[2][0] += *min_x;
            pos[1][0] -= 1. - *max_x;
            pos[3][0] -= 1. - *max_x;

            pos[0][1] += *min_y;
            pos[1][1] += *min_y;
            pos[2][1] -= 1. - *max_y;
            pos[3][1] -= 1. - *max_y;

            pos[0][2] += *min_z;
            pos[1][2] += *min_z;
            pos[2][2] += *min_z;
            pos[3][2] += *min_z;
        }
        Direction::West => {
            pos[0][0] += *min_x;
            pos[2][0] += *min_x;
            pos[1][0] -= 1. - *max_x;
            pos[3][0] -= 1. - *max_x;

            pos[0][1] += *min_y;
            pos[1][1] += *min_y;
            pos[2][1] -= 1. - *max_y;
            pos[3][1] -= 1. - *max_y;

            pos[0][2] -= 1. - *max_z;
            pos[1][2] -= 1. - *max_z;
            pos[2][2] -= 1. - *max_z;
            pos[3][2] -= 1. - *max_z;
        }
    }
}
