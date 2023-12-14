#![allow(dead_code)]

use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology,
    tasks::{AsyncComputeTaskPool, Task},
};
use derive_more::{From, Into};
use futures_lite::future;
use mc_rs_core::{
    position::{BlockPos, ChunkBlockPos, ChunkPos},
    ResourceLocation,
};
use mc_rs_resourcepack::{assets::resourcepacks::ResourcePacks, pack::ResourcePackAsset};

use crate::{
    blocks::{traits::BlocksTrait, Blocks},
    resources::{CurrentWorld, Worlds},
    world::{
        chunk::{Chunk, ChunkSections},
        section::Section,
        shaders::terrain::TerrainMaterial,
    },
};

/// A [Task] that creates a [`TerrainMaterial`] for a [`Chunk`].
///
/// This is used to decode chunks in parallel.
#[derive(Debug, From, Into, Deref, DerefMut, Component)]
pub struct ChunkMaterialTask(Task<MaterialTaskResult>);

impl ChunkMaterialTask {
    pub(crate) const CHUNK_SECTION_MESHES: usize = 4;
    pub(crate) const SECTIONS_PER_MESH: usize = Chunk::SECTION_COUNT / Self::CHUNK_SECTION_MESHES;
}

pub(crate) type MaterialTaskResult = Vec<MaterialResult>;
type MaterialResult = (Mesh, Vec<BlockTexture>);

impl ChunkMaterialTask {
    pub(super) fn chunk_updater(
        query: Query<(Entity, &ChunkPos, Ref<Chunk>)>,

        current: Res<CurrentWorld>,
        worlds: Res<Worlds>,

        mut commands: Commands,
    ) {
        let Some(world) = worlds.get_world(&current) else {
            return;
        };

        for (entity, chunk_pos, chunk_ref) in query.iter() {
            if chunk_ref.is_added() {
                let neighbors = chunk_pos.sides();

                let mut neighbor_chunks = [None, None, None, None];
                for (i, n) in neighbors.into_iter().enumerate() {
                    if let Some(chunk) = world.get_entity(&n) {
                        if let Ok((_, _, chunk)) = query.get(*chunk) {
                            neighbor_chunks[i] = Some(chunk.sections.clone());
                        }
                    }
                }

                let task = Self::create(chunk_ref.sections.clone(), neighbor_chunks);
                commands.entity(entity).insert(task);
            } else if chunk_ref.is_changed() {
                // TODO: Update all four neighbors
            }
        }
    }

    pub(super) fn create(chunk: ChunkSections, neighbors: [Option<ChunkSections>; 4]) -> Self {
        let task = AsyncComputeTaskPool::get().spawn(Self::material_task(chunk, neighbors));
        Self(task)
    }

    pub(crate) fn poll_tasks(
        mut query: Query<(Entity, &mut ChunkMaterialTask)>,
        packs: Res<ResourcePacks>,
        pack_assets: Res<Assets<ResourcePackAsset>>,

        mut materials: ResMut<Assets<TerrainMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
                let mut commands = commands.entity(entity);

                for (index, (mesh, material_ids)) in result.into_iter().enumerate() {
                    let material_textures: Vec<Handle<Image>> = material_ids
                        .into_iter()
                        .filter_map(|m| {
                            let blocks = Blocks::from_u32(m.id);
                            if let Blocks::Air(_) = blocks {
                                return None;
                            }

                            let mut iter = blocks.resource_location().split(':');
                            let namespace = iter.next().unwrap();
                            let path = iter.next().unwrap();

                            // TODO: Get the actual texture
                            Some(packs.get_model_texture_or_fallback(
                                &ResourceLocation::from(format!("{namespace}:block/{path}")),
                                &m.side,
                                &pack_assets,
                            ))
                        })
                        .cloned()
                        .collect();

                    let bundle = MaterialMeshBundle::<TerrainMaterial> {
                        material: materials.add(TerrainMaterial::new(material_textures)),
                        mesh: meshes.add(mesh),
                        transform: Transform::from_translation(Vec3::new(
                            0.0,
                            (index * Self::SECTIONS_PER_MESH * Section::SECTION_HEIGHT) as f32
                                + Chunk::VERTICAL_SHIFT as f32,
                            0.0,
                        )),
                        ..Default::default()
                    };

                    commands.with_children(|chunk| {
                        chunk.spawn((ChunkMaterialSection(index), bundle));
                    });
                }

                commands.remove::<ChunkMaterialTask>();
            }
        }
    }

    async fn material_task(
        chunk: ChunkSections,
        neighbors: [Option<ChunkSections>; 4],
    ) -> MaterialTaskResult {
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
    pub(crate) const MESH_HEIGHT: usize =
        ChunkMaterialTask::SECTIONS_PER_MESH * Section::SECTION_HEIGHT;

    fn section_task(sections: &[Section], _neighbors: [Option<&[Section]>; 4]) -> MaterialResult {
        let mut required_textures = Vec::new();

        let mut section_mesh_data =
            [[[0u32; Section::SECTION_WIDTH + 2]; Section::SECTION_DEPTH + 2]; Self::MESH_HEIGHT];

        for (y, layer) in section_mesh_data.iter_mut().enumerate() {
            let section = &sections[y / Section::SECTION_HEIGHT];

            for (z, row) in layer.iter_mut().enumerate() {
                for (x, block) in row.iter_mut().enumerate() {
                    let pos = BlockPos::new(x as i32 - 1, y as i32 - 1, z as i32 - 1);
                    let pos = ChunkBlockPos::from(pos);

                    let state_id = section.blocks.get(&pos);
                    let texture = BlockTexture::new(String::from("up"), state_id);

                    if !required_textures.contains(&texture) {
                        required_textures.push(texture);
                    }

                    *block = state_id;
                }
            }
        }

        (
            Mesh::new(PrimitiveTopology::TriangleList),
            required_textures,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTexture {
    side: String,
    id: u32,
}

impl BlockTexture {
    fn new(side: String, id: u32) -> Self { Self { side, id } }
}
