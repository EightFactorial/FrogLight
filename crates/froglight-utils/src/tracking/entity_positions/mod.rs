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

use crate::fixed_schedules::TenthSecondSchedule;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityChunkMap resource
    app.init_resource::<EntityChunkMap>();

    // Update the EntityChunkMap every 1/10 second if there are any entities
    app.add_systems(
        TenthSecondSchedule,
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

            // Clear the position's list if it hasn't been
            if let Entry::Vacant(entry) = updated_positions.entry(*position) {
                entry.insert();
                entities.clear();
            }

            // Add the entity to the list
            entities.push(entity);
        }

        // Clear all positions that haven't been updated
        position_map.iter_mut().for_each(|(position, entities)| {
            if !updated_positions.contains(position) {
                entities.clear();
            }
        });

        // Clear the updated positions set
        updated_positions.clear();
    }
}
