use bevy::{
    self,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_rapier3d::prelude::{RigidBody, Sensor, Sleeping};
use futures_lite::future::{block_on, poll_once};
use mc_rs_core::{
    schedule::set::GameSet,
    world::{structure::section::SectionComponent, ChunkUpdateEvent, SECTION_HEIGHT},
};

use self::{
    chunk::chunk_fn,
    section::{MeshData, SectionData, SectionResult},
};

mod chunk;
mod section;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Update,
        (
            ChunkTask::listener,
            ChunkTask::poll_tasks.run_if(ChunkTask::any_tasks_finished),
        )
            .chain()
            .in_set(GameSet),
    );
}

/// A task that generates a mesh for a section
#[derive(Deref, DerefMut, Component)]
pub struct ChunkTask(pub Task<ChunkTaskResult>);
type ChunkTaskResult = Vec<SectionResult>;

impl ChunkTask {
    fn listener(mut events: EventReader<ChunkUpdateEvent>, mut commands: Commands) {
        events.iter().for_each(|event| {
            let ChunkUpdateEvent {
                entity,
                chunk,
                neighbors,
                block_data,
            } = event.clone();

            commands.entity(entity).insert(ChunkTask(
                AsyncComputeTaskPool::get().spawn(chunk_fn(chunk, neighbors, block_data)),
            ));
        });
    }

    fn as_task(&mut self) -> &mut Task<ChunkTaskResult> { &mut self.0 }

    fn any_tasks_finished(mut query: Query<&ChunkTask>) -> bool {
        query.iter_mut().any(|task| task.is_finished())
    }

    fn poll_tasks(
        mut query: Query<(Entity, &mut ChunkTask)>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
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
                        for (index, data) in results.into_iter().enumerate() {
                            if let Some(result) = data {
                                let SectionData {
                                    opaque,
                                    transparent,
                                    terrain_collider,
                                    fluid_collider,
                                } = result;

                                let transform =
                                    Transform::from_xyz(0.0, (index * SECTION_HEIGHT) as f32, 0.0);

                                // Create the opaque mesh bundle
                                if let Some(opaque) = opaque {
                                    let MeshData { mesh, .. } = opaque;

                                    parent.spawn((
                                        SectionComponent,
                                        PbrBundle {
                                            mesh: meshes.add(mesh),
                                            material: materials.add(Color::GRAY.into()),
                                            transform,
                                            ..Default::default()
                                        },
                                    ));
                                }

                                // Create the transparent mesh bundle
                                if let Some(transparent_mesh) = transparent {
                                    let MeshData { mesh, .. } = transparent_mesh;

                                    parent.spawn((
                                        SectionComponent,
                                        PbrBundle {
                                            mesh: meshes.add(mesh),
                                            material: materials.add(Color::GRAY.into()),
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
