use bevy::{
    self,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_rapier3d::prelude::{RigidBody, Sensor, Sleeping};
use futures_lite::future::{block_on, poll_once};

use crate::systems::{app_state::GameSet, blocks::BlockData};

use self::{
    chunk::chunk_fn,
    section::{SectionData, SectionResult},
};

use super::{
    material::BlockMaterial,
    structure::{chunk::ChunkSections, section::SectionComponent},
    SECTION_HEIGHT,
};

mod chunk;
mod section;

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
        block_data: BlockData,
    ) -> Self {
        ChunkTask(AsyncComputeTaskPool::get().spawn(chunk_fn(chunk, neighbors, block_data)))
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
