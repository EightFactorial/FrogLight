use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, VertexAttributeValues},
        render_resource::PrimitiveTopology,
    },
    tasks::{AsyncComputeTaskPool, Task},
};
use block_mesh::{
    greedy_quads,
    ndshape::{ConstShape, ConstShape3u32},
    GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG,
};
use futures_lite::future::{block_on, poll_once};

use crate::systems::{
    blocks::{block::VoxelType, block_list::Blocks},
    world::{CHUNK_SIZE, SECTION_HEIGHT},
};

use super::{section::Section, CHUNK_VERT_DISPLACEMENT};

/// A marker component for sections
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Component)]
pub struct SectionMarker(pub usize);

/// A task that generates a mesh for a section
#[derive(Deref, DerefMut, Component)]
pub struct SectionTask(pub Task<Mesh>);

const X: u32 = CHUNK_SIZE as u32;
const Y: u32 = SECTION_HEIGHT as u32;
const Z: u32 = CHUNK_SIZE as u32;
type ChunkShape = ConstShape3u32<X, Y, Z>;
type MeshChunkShape = ConstShape3u32<{ X + 2 }, { Y + 2 }, { Z + 2 }>;

impl SectionTask {
    pub(super) fn new(section: Section, blocks: Blocks) -> Self {
        Self(AsyncComputeTaskPool::get().spawn(Self::mesh_task(section, blocks)))
    }

    pub fn task_mut(&mut self) -> &mut Task<Mesh> { &mut self.0 }

    pub(super) fn poll_tasks(
        mut query: Query<(Entity, &SectionMarker, &mut SectionTask)>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut commands: Commands,
    ) {
        for (entity, marker, mut task) in query.iter_mut() {
            if let Some(mesh) = block_on(poll_once(task.task_mut())) {
                let pbr = PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(Color::DARK_GRAY.into()),
                    ..Default::default()
                };

                commands
                    .entity(entity)
                    .insert(pbr)
                    .insert(TransformBundle {
                        local: Transform::from_xyz(
                            0.,
                            (**marker * SECTION_HEIGHT) as f32 - CHUNK_VERT_DISPLACEMENT as f32,
                            0.,
                        ),
                        ..Default::default()
                    })
                    .remove::<SectionTask>();
            }
        }
    }

    pub(super) fn destory_tasks(query: Query<Entity, With<SectionTask>>, mut commands: Commands) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }

    async fn mesh_task(section: Section, blocks: Blocks) -> Mesh {
        let blocks = blocks.read().unwrap();
        let section_blocks = section.get_blocks();

        let mut shape = [VoxelType::Empty; MeshChunkShape::SIZE as usize];
        for y in 0..16 {
            for z in 0..16 {
                for x in 0..16 {
                    // Why is it x,z,y
                    let shape_index = MeshChunkShape::linearize([x + 1, z + 1, y + 1]) as usize;
                    let section_index = ChunkShape::linearize([x, y, z]) as usize;

                    let block = blocks
                        .get(&section_blocks[section_index])
                        .unwrap_or_else(|| {
                            blocks.get(&u32::MAX).expect("Error getting fallback block")
                        });

                    shape[shape_index] = block.voxel_type;
                }
            }
        }

        let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
        let mut buffer = GreedyQuadsBuffer::new(shape.len());
        greedy_quads(
            &shape,
            &MeshChunkShape {},
            [0; 3],
            [X + 1, Y + 1, Z + 1],
            &faces,
            &mut buffer,
        );

        let num_indices = buffer.quads.num_quads() * 6;
        let num_vertices = buffer.quads.num_quads() * 4;
        let mut indices = Vec::with_capacity(num_indices);
        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        for (group, face) in buffer.quads.groups.into_iter().zip(faces) {
            for quad in group.into_iter() {
                indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
                positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
                normals.extend_from_slice(&face.quad_mesh_normals());
            }
        }

        let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::Float32x3(positions),
        );
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::Float32x3(normals),
        );
        render_mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices]),
        );
        render_mesh.set_indices(Some(Indices::U32(indices.clone())));

        render_mesh
    }
}
