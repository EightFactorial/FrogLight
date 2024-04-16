use bevy_app::App;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::Entity,
    query::{With, Without},
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Query, ResMut, Resource},
};
use froglight_protocol::common::{ChunkPosition, EntityId};
use froglight_world::Chunk;
use hashbrown::{hash_set::Entry, HashMap, HashSet};

use crate::fixed_schedules::TwoTickSchedule;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityChunkMap resource
    app.init_resource::<EntityChunkMap>();

    // If there are any entities, update the EntityChunkMap every other tick
    app.add_systems(
        TwoTickSchedule,
        EntityChunkMap::update_entitychunkmap.run_if(any_with_component::<EntityId>),
    );
}

/// A map containing the positions of all [`Entities`](EntityId).
///
/// This is much faster than iterating over all entities to find a ones
/// in a specific chunk.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityChunkMap {
    #[deref]
    position_map: HashMap<ChunkPosition, Vec<Entity>>,
    updated_positions: HashSet<ChunkPosition>,
}

impl EntityChunkMap {
    /// Updates the [`EntityChunkMap`] with the latest entity
    /// [`ChunkPositions`](ChunkPosition).
    ///
    /// Added a filter for the `Chunk` component to avoid system conflicts.
    #[allow(clippy::type_complexity)]
    fn update_entitychunkmap(
        query: Query<(Entity, &ChunkPosition), (With<EntityId>, Without<Chunk>)>,
        mut map: ResMut<Self>,
    ) {
        // Get the position map and updated positions set
        let Self { position_map, updated_positions } = &mut *map;

        // Insert all positions and entities into the map
        for (entity, position) in &query {
            // Get the list of entities at the position
            let entities = position_map.entry(*position).or_default();

            // Clear the position's entity list if it hasn't been
            if let Entry::Vacant(entry) = updated_positions.entry(*position) {
                entry.insert();
                entities.clear();
            }

            // Add the entity to the entity list
            entities.push(entity);
        }

        // Clear all positions that haven't been updated
        position_map.retain(|position, _| updated_positions.remove(position));
        updated_positions.clear();
    }

    /// Returns the total number of [`Chunks`](ChunkPosition).
    #[must_use]
    pub fn total_chunks(&self) -> usize { self.position_map.len() }

    /// Returns the total number of [`Entities`](Entity).
    #[must_use]
    pub fn total_entities(&self) -> usize { self.position_map.values().map(Vec::len).sum() }
}
