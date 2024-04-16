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
use hashbrown::HashMap;

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
/// This is much faster than iterating over all entities to find a specific
/// entity.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityChunkMap {
    position_map: HashMap<ChunkPosition, Entity>,
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
        #[cfg(debug_assertions)]
        let preupdate_size = map.len();

        // Clear the map
        map.clear();

        // Insert all positions and entities into the map
        for (entity, position) in &query {
            map.insert(*position, entity);
        }

        #[cfg(debug_assertions)]
        bevy_log::trace!("EntityChunkMap size: {} (was: {preupdate_size})", map.len());
    }
}
