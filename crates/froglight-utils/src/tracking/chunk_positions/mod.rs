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

use crate::fixed_schedules::TwoTickSchedule;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the ChunkPositionMap resource
    app.init_resource::<ChunkPositionMap>();

    // If there are any chunks, update the ChunkPositionMap every other tick
    app.add_systems(
        TwoTickSchedule,
        ChunkPositionMap::update_chunkpositionmap.run_if(any_with_component::<Chunk>),
    );
}

/// A map containing the positions of all [`Chunks`](Chunk).
///
/// This is much faster than iterating over all entities to find a specific
/// chunk.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct ChunkPositionMap {
    position_map: HashMap<ChunkPosition, Entity>,
}

impl ChunkPositionMap {
    /// Updates the [`ChunkPositionMap`] with the latest chunk
    /// [`ChunkPositions`](ChunkPosition).
    ///
    /// Added a filter for the `EntityId` component to avoid system conflicts.
    #[allow(clippy::type_complexity)]
    fn update_chunkpositionmap(
        query: Query<(Entity, &ChunkPosition), (With<Chunk>, Without<EntityId>)>,
        mut map: ResMut<Self>,
    ) {
        // Clear the map
        map.clear();

        // Insert all positions and entities into the map
        for (entity, position) in &query {
            map.insert(*position, entity);
        }
    }
}
