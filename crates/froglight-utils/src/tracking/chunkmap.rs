use bevy_app::{App, PreUpdate};
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

use crate::systemsets::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the ChunkPositionMap resource
    app.init_resource::<ChunkPositionMap>();

    // Update the ChunkPositionMap every frame
    app.add_systems(
        PreUpdate,
        ChunkPositionMap::update_chunkpositions
            .run_if(any_with_component::<Chunk>)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`ChunkPosition`]s to [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct ChunkPositionMap(HashMap<ChunkPosition, Entity>);

impl ChunkPositionMap {
    #[allow(clippy::type_complexity)]
    fn update_chunkpositions(
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
