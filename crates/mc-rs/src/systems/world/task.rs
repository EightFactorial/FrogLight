use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future::{block_on, poll_once};

use super::section::Section;

#[derive(Deref, DerefMut, Component)]
pub struct SectionTask(pub Task<Mesh>);

impl SectionTask {
    pub(super) fn new(section: Section) -> Self {
        Self(AsyncComputeTaskPool::get().spawn(Self::mesh_task(section)))
    }

    pub(super) fn poll_tasks(
        mut query: Query<(Entity, &mut SectionTask)>,
        mut materials: Assets<StandardMaterial>,
        mut meshes: Assets<Mesh>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(mesh) = block_on(poll_once(&mut **task)) {
                let pbr = PbrBundle {
                    mesh: meshes.add(mesh),
                    material: materials.add(Color::GRAY.into()),
                    ..Default::default()
                };

                commands.entity(entity).insert(pbr).remove::<SectionTask>();
            }
        }
    }

    async fn mesh_task(_section: Section) -> Mesh { todo!() }
}
