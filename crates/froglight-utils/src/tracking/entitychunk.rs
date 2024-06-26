#[cfg(not(feature = "hashbrown"))]
use std::collections::{HashMap, HashSet};

use bevy_app::{App, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::Entity,
    query::With,
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Query, ResMut, Resource},
};
use froglight_protocol::common::{ChunkPosition, EntityId};
#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};

use crate::systemset::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityChunkMap resource
    app.init_resource::<EntityChunkMap>();

    // Update the EntityChunkMap every frame
    app.add_systems(
        PreUpdate,
        EntityChunkMap::update_entitychunks
            .run_if(any_with_component::<EntityId>)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`ChunkPosition`]s to a list of [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityChunkMap {
    #[deref]
    map: HashMap<ChunkPosition, Vec<Entity>>,
    updates: HashSet<ChunkPosition>,
}

impl EntityChunkMap {
    #[allow(clippy::type_complexity)]
    fn update_entitychunks(
        query: Query<(Entity, &ChunkPosition), With<EntityId>>,
        mut map: ResMut<Self>,
    ) {
        let Self { map, updates } = &mut *map;

        // Loop through all entities with an EntityId
        for (entity, position) in &query {
            // Get the list of entities in the chunk
            let entity_list = map.entry(*position).or_default();

            // If the chunk has not been reset this frame, clear the list
            if updates.insert(*position) {
                entity_list.clear();
            }

            // Add the entity to the list
            entity_list.push(entity);
        }

        // Remove all chunks that have not been updated this frame
        map.retain(|pos, _| updates.contains(pos));
        updates.clear();
    }
}
