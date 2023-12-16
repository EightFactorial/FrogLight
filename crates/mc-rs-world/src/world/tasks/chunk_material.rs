#![allow(dead_code)]

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use compact_str::CompactString;
use derive_more::{From, Into};
use futures_lite::future;
use mc_rs_core::{
    position::{BlockPos, ChunkBlockPos, ChunkPos},
    ResourceLocation,
};
use mc_rs_resourcepack::{assets::resourcepacks::ResourcePacks, pack::ResourcePackAsset};

use crate::{
    blocks::{structs::BlockAir, traits::BlocksTrait, Blocks},
    resources::{CurrentWorld, Worlds},
    world::{
        chunk::{Chunk, ChunkSections},
        section::Section,
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
    pub(super) fn chunk_update(
        query: Query<(Entity, &ChunkPos, Ref<Chunk>)>,

        current: Res<CurrentWorld>,
        worlds: Res<Worlds>,

        mut commands: Commands,
    ) {
        let Some(world) = worlds.get_world(&current) else {
            return;
        };

        for (entity, chunk_pos, chunk_ref) in query.iter() {
            // Create/recreate the chunk mesh if the chunk is added/changed
            if chunk_ref.is_added() || chunk_ref.is_changed() {
                let mut neighbor_chunks = [None, None, None, None];
                for (i, n) in chunk_pos.sides().into_iter().enumerate() {
                    if let Some(Ok((_, _, chunk))) = world.get_entity(&n).map(|&e| query.get(e)) {
                        neighbor_chunks[i] = Some(chunk.sections.clone());
                    }
                }

                let task = Self::create(chunk_ref.sections.clone(), neighbor_chunks);
                commands.entity(entity).insert(task);
            }

            // Update the neighbor's chunk meshes if this chunk is changed
            if chunk_ref.is_changed() {
                for neighbor in chunk_pos.sides() {
                    let Some(Ok((entity, _, chunk_ref))) =
                        world.get_entity(&neighbor).map(|&e| query.get(e))
                    else {
                        continue;
                    };

                    let mut neighbor_chunks = [None, None, None, None];
                    for (i, n) in neighbor.sides().into_iter().enumerate() {
                        if let Some(Ok((_, _, chunk))) = world.get_entity(&n).map(|&e| query.get(e))
                        {
                            neighbor_chunks[i] = Some(chunk.sections.clone());
                        }
                    }

                    let task = Self::create(chunk_ref.sections.clone(), neighbor_chunks);
                    commands.entity(entity).insert(task);
                }
            }
        }
    }

    fn create(chunk: ChunkSections, neighbors: [Option<ChunkSections>; 4]) -> Self {
        let task = AsyncComputeTaskPool::get().spawn(Self::material_task(chunk, neighbors));
        Self(task)
    }

    pub(super) fn poll_tasks(
        mut query: Query<(Entity, &mut ChunkMaterialTask)>,
        packs: Res<ResourcePacks>,
        pack_assets: Res<Assets<ResourcePackAsset>>,

        // mut materials: ResMut<Assets<TerrainMaterial>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut commands: Commands,
    ) {
        for (entity, mut task) in query.iter_mut() {
            if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
                let mut commands = commands.entity(entity);

                for (index, (mesh, material_ids)) in result.into_iter().enumerate() {
                    let _material_textures: Vec<Handle<Image>> = material_ids
                        .into_iter()
                        .filter_map(|m| {
                            let block = Blocks::from_u32(m.id);
                            if block.is_air() {
                                return None;
                            }

                            let mut iter = block.resource_location().split(':');
                            let namespace = iter.next()?;
                            let path = iter.next()?;

                            Some(packs.get_model_texture_or_fallback(
                                &ResourceLocation::new(format!("{namespace}:block/{path}")),
                                &m.side,
                                &pack_assets,
                            ))
                        })
                        .cloned()
                        .collect();

                    let bundle = MaterialMeshBundle::<StandardMaterial> {
                        material: materials.add(Color::GRAY.into()),
                        transform: Transform::from_xyz(
                            0.0,
                            (index * ChunkMaterialSection::MESH_HEIGHT) as f32
                                + Chunk::VERTICAL_SHIFT as f32,
                            0.0,
                        ),
                        mesh: meshes.add(mesh),
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

    fn section_task(sections: &[Section], neighbors: [Option<&[Section]>; 4]) -> MaterialResult {
        // Store all blocks in a 3D array, with neighbors' blocks on the edges
        let mut section_mesh_data = [[[Blocks::Air(BlockAir); Section::SECTION_WIDTH + 2];
            Section::SECTION_DEPTH + 2]; Self::MESH_HEIGHT];

        for (y, layer) in section_mesh_data.iter_mut().enumerate() {
            let mut section = &sections[y / Section::SECTION_HEIGHT];

            for (z, row) in layer.iter_mut().enumerate() {
                // Get the neighbor section if the block is on the edge
                if z <= 1 {
                    if let Some(neighbor) = neighbors[0] {
                        section = &neighbor[y / Section::SECTION_HEIGHT];
                    }
                } else if z >= Section::SECTION_DEPTH {
                    if let Some(neighbor) = neighbors[1] {
                        section = &neighbor[y / Section::SECTION_HEIGHT];
                    }
                }

                for (x, block) in row.iter_mut().enumerate() {
                    // Get the neighbor section if the block is on the edge
                    if x <= 1 {
                        if let Some(neighbor) = neighbors[2] {
                            section = &neighbor[y / Section::SECTION_HEIGHT];
                        }
                    } else if x >= Section::SECTION_WIDTH {
                        if let Some(neighbor) = neighbors[3] {
                            section = &neighbor[y / Section::SECTION_HEIGHT];
                        }
                    }

                    let pos = BlockPos::new(x as i32 - 1, y as i32 - 1, z as i32 - 1);
                    let pos = ChunkBlockPos::from(pos);
                    *block = Blocks::from_u32(section.blocks.get(&pos));
                }
            }
        }

        let required_textures = Vec::new();
        let mesh = Mesh::from(shape::Box::new(
            Section::SECTION_WIDTH as f32,
            Self::MESH_HEIGHT as f32,
            Section::SECTION_DEPTH as f32,
        ));

        // TODO: Generate mesh and textures

        (mesh, required_textures)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockTexture {
    side: CompactString,
    id: u32,
}

impl BlockTexture {
    pub fn new(side: CompactString, id: u32) -> Self { Self { side, id } }
}
