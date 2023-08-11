#![allow(dead_code)]

use bevy::{prelude::*, utils::HashMap};
use mc_rs_proto::{
    buffer::DecodeError,
    types::{packets::chunk_data::ChunkDataPacket, position::ChunkPos, ResourceLocation},
};

use self::{
    chunk::{Chunk, ChunkEntity},
    global_palette::GlobalPalette,
    task::{SectionMarker, SectionTask},
};

use super::{
    app_state::{ApplicationState, GameSet},
    blocks::block_list::Blocks,
};

pub mod chunk;
pub mod global_palette;
pub mod palette;
pub mod resources;
pub mod section;
pub mod task;

/// Adds the `Worlds` resource and its systems.
pub(super) fn add_systems(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::Game),
        Worlds::create.in_set(GameSet),
    );

    app.add_systems(
        Update,
        SectionTask::poll_tasks
            // .run_if(any_with_component::<SectionTask>())
            .in_set(GameSet),
    );

    app.add_systems(
        OnExit(ApplicationState::Game),
        (
            Worlds::destroy,
            SectionTask::destory_tasks.run_if(any_with_component::<SectionTask>()),
        )
            .in_set(GameSet),
    );
}

pub const CHUNK_HEIGHT: usize = 384;
pub const CHUNK_VERT_DISPLACEMENT: isize = -64;
pub const CHUNK_SIZE: usize = 16;
pub const SECTION_HEIGHT: usize = 16;
pub const SECTION_COUNT: usize = CHUNK_HEIGHT / SECTION_HEIGHT;

/// The `Worlds` resource contains all the worlds of the server.
#[derive(Debug, Default, Clone, Deref, DerefMut, Resource)]
pub struct Worlds(pub HashMap<WorldType, World>);

impl Worlds {
    /// Creates a new `Worlds` resource when joining a server.
    pub fn create(mut commands: Commands) { commands.init_resource::<Worlds>(); }

    /// Destroys the `Worlds` resource when leaving a server.
    #[allow(clippy::type_complexity)]
    fn destroy(
        query: Query<Entity, Or<(With<Chunk>, With<SectionMarker>)>>,
        mut commands: Commands,
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        commands.remove_resource::<Worlds>();
    }

    /// Spawns a new chunk from the given chunk data and inserts it into the given world.
    ///
    /// This will create the world if it doesn't exist.
    pub fn insert_data<V: GlobalPalette>(
        &mut self,
        world_type: &WorldType,
        position: ChunkPos,
        chunk_data: ChunkDataPacket,
        blocks: &Blocks,
        world: &mut bevy::ecs::world::World,
    ) -> Result<ChunkEntity, DecodeError> {
        let (chunk, sections) = Chunk::decode::<V>(chunk_data, blocks, world)?;
        Ok(self.insert_chunk(world_type, position, chunk, Some(sections), world))
    }

    /// Spawns a new chunk and inserts it into the given world.
    ///
    /// This will create the world if it doesn't exist.
    pub fn insert_chunk(
        &mut self,
        world_type: &WorldType,
        position: ChunkPos,
        chunk: Chunk,
        sections: Option<Vec<Entity>>,
        world: &mut bevy::ecs::world::World,
    ) -> ChunkEntity {
        // Only make the chunk visible if it has any sections with entities.
        let visibility = if sections.is_some() {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        let mut chunk = world.spawn((
            chunk,
            TransformBundle {
                local: Transform::from_xyz(
                    (position.x * CHUNK_SIZE as i32) as f32,
                    0.,
                    (position.y * CHUNK_SIZE as i32) as f32,
                ),
                ..Default::default()
            },
            VisibilityBundle {
                visibility,
                ..Default::default()
            },
        ));

        if let Some(sections) = sections {
            chunk.insert_children(0, &sections);
        }

        let entity = ChunkEntity(chunk.id());

        self.insert_entity(world_type, position, entity);
        entity
    }

    /// Inserts a chunk entity into the given world.
    ///
    /// This will create the world if it doesn't exist.
    pub fn insert_entity(
        &mut self,
        world_type: &WorldType,
        position: ChunkPos,
        entity: ChunkEntity,
    ) {
        if let Some(world) = self.get_mut(world_type) {
            world.insert_chunk(position, entity);
        } else {
            let mut world = World::default();
            world.insert_chunk(position, entity);

            self.insert(world_type.clone(), world);
        }
    }

    /// Inserts an empty world.
    ///
    /// This does nothing if the world already exists.
    pub fn insert_empty(&mut self, world_type: &WorldType) {
        if !self.contains_key(world_type) {
            self.insert(world_type.clone(), World::default());
        }
    }

    /// Get the world.
    pub fn get_world(&self, world_type: &WorldType) -> Option<&World> { self.get(world_type) }

    /// Get a world mutably.
    pub fn get_world_mut(&mut self, world_type: &WorldType) -> Option<&mut World> {
        self.get_mut(world_type)
    }

    /// Get the entity id of a chunk in a world.
    pub fn get_chunk_id(&self, world_type: &WorldType, position: ChunkPos) -> Option<&ChunkEntity> {
        self.get_world(world_type)
            .and_then(|world| world.chunks.get(&position))
    }

    /// Get the chunk at the position in a world.
    pub fn get_chunk<'a>(
        &self,
        query: &'a Query<&Chunk>,
        world_type: &WorldType,
        position: ChunkPos,
    ) -> Option<&'a Chunk> {
        self.get_chunk_id(world_type, position)
            .and_then(|entity| query.get(**entity).ok())
    }

    /// Get the chunk at the position in a world mutably.
    pub fn get_chunk_mut<'a>(
        &'a mut self,
        query: &'a mut Query<&mut Chunk>,
        world_type: &WorldType,
        position: ChunkPos,
    ) -> Option<Mut<Chunk>> {
        self.get_chunk_id(world_type, position)
            .and_then(|entity| query.get_mut(**entity).ok())
    }
}

/// The `WorldType` enum represents the type of a world.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum WorldType {
    Nether,
    #[default]
    Overworld,
    End,
    Other(ResourceLocation),
}

impl From<ResourceLocation> for WorldType {
    fn from(value: ResourceLocation) -> Self {
        match value.as_str() {
            "minecraft:the_nether" => WorldType::Nether,
            "minecraft:overworld" => WorldType::Overworld,
            "minecraft:the_end" => WorldType::End,
            _ => WorldType::Other(value),
        }
    }
}

/// The `World` struct represents a world.
///
/// Currently, it only contains chunk entities.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct World {
    #[deref]
    pub chunks: HashMap<ChunkPos, ChunkEntity>,
}

impl World {
    /// Inserts a chunk entity into the world.
    pub fn insert_chunk(&mut self, position: ChunkPos, chunk: ChunkEntity) {
        self.chunks.insert(position, chunk);
    }

    /// Gets the chunk entity at the position.
    pub fn get_chunk(&self, position: ChunkPos) -> Option<&ChunkEntity> {
        self.chunks.get(&position)
    }
}
