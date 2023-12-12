#![allow(dead_code)]

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use derive_more::{From, Into};
use futures_lite::future;

use crate::world::{
    chunk::{Chunk, ChunkSections},
    section::Section,
    shaders::terrain::TerrainMaterial,
};

/// A [Task] that creates a [`TerrainMaterial`] for a [`Chunk`].
///
/// This is used to decode chunks in parallel.
#[derive(Debug, From, Into, Deref, DerefMut, Component)]
pub struct ChunkMaterialTask(Task<MaterialResult>);

impl ChunkMaterialTask {
    pub(crate) const CHUNK_SECTION_MESHES: usize = 4;
    pub(crate) const SECTIONS_PER_MESH: usize = Chunk::SECTION_COUNT / Self::CHUNK_SECTION_MESHES;
}
pub(crate) type MaterialResult = Vec<(Mesh, TerrainMaterial)>;

impl ChunkMaterialTask {
    pub(super) fn create(chunk: &Chunk, neighbors: [Option<&Chunk>; 4]) -> Self {
        let task = AsyncComputeTaskPool::get().spawn(Self::material_task(
            chunk.sections.clone(),
            neighbors.map(|c| c.map(|c| c.sections.clone())),
        ));

        Self(task)
    }

    pub(crate) fn poll_tasks(
        mut query: Query<(Entity, &mut ChunkMaterialTask)>,
        mut materials: ResMut<Assets<TerrainMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
                let mut commands = commands.entity(entity);

                for (index, (mesh, material)) in result.into_iter().enumerate() {
                    let material = materials.add(material);
                    let mesh = meshes.add(mesh);

                    let bundle = MaterialMeshBundle::<TerrainMaterial> {
                        material,
                        mesh,
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            ((index * Self::SECTIONS_PER_MESH) as f32
                                * Section::SECTION_HEIGHT as f32)
                                + Chunk::VERTICAL_SHIFT as f32,
                            0.0,
                        )),
                        ..Default::default()
                    };

                    commands.with_children(|chunk| {
                        chunk.spawn((ChunkMaterialSection(index), bundle));
                    });
                }
            }
        }
    }

    async fn material_task(
        chunk: ChunkSections,
        neighbors: [Option<ChunkSections>; 4],
    ) -> MaterialResult {
        let mut materials = Vec::with_capacity(Self::CHUNK_SECTION_MESHES);

        let n1_guard = neighbors[0].as_ref().map(|n| n.read());
        let n2_guard = neighbors[1].as_ref().map(|n| n.read());
        let n3_guard = neighbors[2].as_ref().map(|n| n.read());
        let n4_guard = neighbors[3].as_ref().map(|n| n.read());

        for (i, sections) in chunk.read().chunks(Self::SECTIONS_PER_MESH).enumerate() {
            let neighbors = [
                n1_guard
                    .as_deref()
                    .map(|n| &n[i * Self::SECTIONS_PER_MESH..(i + 1) * Self::SECTIONS_PER_MESH]),
                n2_guard
                    .as_deref()
                    .map(|n| &n[i * Self::SECTIONS_PER_MESH..(i + 1) * Self::SECTIONS_PER_MESH]),
                n3_guard
                    .as_deref()
                    .map(|n| &n[i * Self::SECTIONS_PER_MESH..(i + 1) * Self::SECTIONS_PER_MESH]),
                n4_guard
                    .as_deref()
                    .map(|n| &n[i * Self::SECTIONS_PER_MESH..(i + 1) * Self::SECTIONS_PER_MESH]),
            ];

            materials.push(ChunkMaterialSection::section_task(sections, neighbors));
        }

        materials
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, From, Into, Deref, DerefMut, Component)]
pub struct ChunkMaterialSection(pub usize);

impl ChunkMaterialSection {
    fn section_task(
        _sections: &[Section],
        _neighbors: [Option<&[Section]>; 4],
    ) -> (Mesh, TerrainMaterial) {
        todo!()
    }
}
