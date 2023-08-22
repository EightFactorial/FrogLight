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
        block_list::Blocks,
    },
};

use super::{
    chunk::ChunkSections,
    material::{BindlessMaterial, ATTRIBUTE_TEXTURE_INDEX},
    section::{Section, SectionComponent},
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
                            if let Some((mesh, textures)) = option {
                                // Create the section collider
                                let Some(collider) = Collider::from_bevy_mesh(
                                    &mesh,
                                    &ComputedColliderShape::TriMesh,
                                ) else {
                                    error!("Error creating collider from section mesh!");
                                    continue;
                                };

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
    let mut results = Vec::with_capacity(SECTION_COUNT);

    for index in 0..SECTION_COUNT {
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

        results.push(section_fn(
            &chunk.read().unwrap()[index],
            neighbors,
            &blocks,
        ));
    }

    results
}

type SectionResult = Option<(Mesh, SectionTextures)>;
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
fn section_fn(
    section: &Section,
    neighbors: [Option<Vec<u32>>; 6],
    blocks: &Blocks,
) -> SectionResult {
    if section.block_count == 0 {
        return None;
    }

    let blocks = blocks.read().unwrap();
    let section_data = section.get_blocks();

    let mut textures = Vec::new();
    let mut texture_map = HashMap::new();
    for id in section_data.iter().unique() {
        let Some(block) = blocks.get(id) else {
            continue;
        };

        let Some(new_textures) = block.block_type.textures() else {
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
        textures.extend(block.block_type.textures().unwrap().to_vec());
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
                shape[shape_index] = block.block_type.voxel_type();
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
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    // let mut normals = Vec::with_capacity(num_vertices);
    let mut tex_index = Vec::with_capacity(num_vertices);
    let mut tex_uvs = Vec::with_capacity(num_vertices);
    for (group, face) in buffer.quads.groups.into_iter().zip(faces) {
        for quad in group.into_iter() {
            // Get the data
            let ind = face.quad_mesh_indices(positions.len() as u32);
            let pos = face.quad_mesh_positions(&quad, 1.0);
            // let norm = face.quad_mesh_normals();
            let uvs = face.tex_coords(RIGHT_HANDED_Y_UP_CONFIG.u_flip_face, true, &quad);

            {
                // Get the block
                let [x, y, z] = quad.minimum;
                let data_index = ChunkShape::linearize([x - 1, z - 1, y - 1]) as usize;
                let block_id = &section_data[data_index];
                let block = blocks.get(block_id).unwrap_or(&blocks[&u32::MAX]);

                // Get the texture index
                let start = texture_map.get(block_id);
                let direction = {
                    let [x, y, z] = face.signed_normal().into();
                    Direction::from([x, y, z])
                };
                let index = match &block.block_type {
                    BlockType::Voxel { texture, .. } | BlockType::Simple { texture, .. } => {
                        texture.get_texture_index(direction)
                    }
                    BlockType::Complex { .. } => todo!(),
                };

                match (start, index) {
                    (Some(start), Some(index)) => {
                        // Add the texture index
                        tex_index.extend([(*start + index) as u32; 4]);
                    }
                    _ => {
                        // Push the fallback texture index
                        tex_index.extend([texture_map[&u32::MAX] as u32; 4]);
                    }
                }
            }

            // Add the data
            indices.extend(ind);
            positions.extend(pos);
            // normals.extend(norm);
            tex_uvs.extend(uvs);
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
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

    render_mesh.set_indices(Some(Indices::U32(indices)));

    Some((render_mesh, textures))
}
