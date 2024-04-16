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
use hashbrown::{HashMap, HashSet};

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
    position_map: HashMap<ChunkPosition, Vec<Entity>>,
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
        // Create a set to track which positions have been updated
        let mut updated_positions = HashSet::new();

        // Insert all positions and entities into the map
        for (entity, position) in &query {
            // Get the list of entities at the position
            let entities = map.position_map.entry(*position).or_default();

            // Clear the position's list if it hasn't been
            if !updated_positions.contains(position) {
                updated_positions.insert(*position);
                entities.clear();
            }

            // Add the entity to the list
            entities.push(entity);

            // Track that the position has been updated
            updated_positions.insert(*position);
        }

        // Clear all positions that haven't been updated
        map.position_map.iter_mut().for_each(|(position, entities)| {
            if !updated_positions.contains(position) {
                entities.clear();
            }
        });
    }
}
