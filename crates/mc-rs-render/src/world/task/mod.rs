use bevy::{
    self,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_rapier3d::prelude::{RigidBody, Sensor, Sleeping};
use futures_lite::future::{block_on, poll_once};
use mc_rs_core::{
    blocks::BlockData,
    schedule::set::GameSet,
    world::{
        structure::{
            chunk::{Chunk, ChunkSections},
            section::SectionComponent,
        },
        worlds::Worlds,
        SECTION_HEIGHT,
    },
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
            ChunkTask::create_tasks.run_if(resource_exists::<Worlds>()),
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
    fn as_task(&mut self) -> &mut Task<ChunkTaskResult> { &mut self.0 }

    fn any_tasks_finished(mut query: Query<&ChunkTask>) -> bool {
        query.iter_mut().any(|task| task.is_finished())
    }

    pub fn new(
        chunk: ChunkSections,
        neighbors: [Option<ChunkSections>; 4],
        block_data: BlockData,
    ) -> Self {
        ChunkTask(AsyncComputeTaskPool::get().spawn(chunk_fn(chunk, neighbors, block_data)))
    }

    /// Regenerate chunk meshes.
    fn create_tasks(
        query: Query<Ref<Chunk>>,
        block_data: Res<BlockData>,
        mut worlds: ResMut<Worlds>,
        mut commands: Commands,
    ) {
        for chunk in query.iter() {
            if chunk.is_changed() {
                let Some(world) = worlds.get_world_mut(&chunk.world_type) else {
                    error!(
                        "Failed to get world {:?} with chunk {:?}!",
                        chunk.world_type, chunk.position
                    );
                    continue;
                };

                // If the chunk is empty, despawn it
                if chunk.block_count() == 0 {
                    if let Some(entity) = world.get_chunk_id(&chunk.position) {
                        let entity = **entity;
                        commands.entity(entity).despawn_recursive();
                        world.remove(&chunk.position);
                    } else {
                        error!(
                            "Failed to get chunk entity for empty chunk {:?}",
                            chunk.position
                        );
                    }
                    continue;
                }

                let mut neighbors = [None, None, None, None];
                for (pos, val) in chunk
                    .position
                    .around()
                    .into_iter()
                    .zip(neighbors.iter_mut())
                {
                    if let Some(chunk) = world.get_chunk_ref(&query, &pos) {
                        if chunk.block_count() != 0 {
                            *val = Some(chunk.sections.clone());
                        }
                    }
                }

                if let Some(entity) = world.get_chunk_id(&chunk.position) {
                    commands.entity(**entity).insert(ChunkTask::new(
                        chunk.sections.clone(),
                        neighbors,
                        block_data.clone(),
                    ));
                } else {
                    error!(
                        "Failed to get chunk entity for changed chunk {:?}",
                        chunk.position
                    );
                }
            }

            if chunk.is_added() {
                // For each neighboring chunk
                let world = worlds.get_world(&chunk.world_type).unwrap();
                for neighbor_pos in chunk.position.around().into_iter() {
                    if let Some(neighbor_chunk) = world.get_chunk_ref(&query, &neighbor_pos) {
                        // Skip chunks with no blocks
                        if neighbor_chunk.block_count() == 0 {
                            continue;
                        }

                        // Get that chunk's neighbors
                        let mut neighbors = [None, None, None, None];
                        for (pos, val) in
                            neighbor_pos.around().into_iter().zip(neighbors.iter_mut())
                        {
                            if let Some(chunk) = world.get_chunk_ref(&query, &pos) {
                                *val = Some(chunk.sections.clone());
                            }
                        }

                        // Update the neighbor chunk mesh
                        if let Some(entity) = world.get_chunk_id(&neighbor_pos) {
                            commands.entity(**entity).insert(ChunkTask::new(
                                chunk.sections.clone(),
                                neighbors,
                                block_data.clone(),
                            ));
                        } else {
                            error!("Failed to get chunk entity for added chunk {neighbor_pos:?}");
                        }
                    }
                }
            }
        }
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
