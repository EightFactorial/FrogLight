#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use bevy_app::{App, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::Entity,
    query::With,
    schedule::{
        common_conditions::{any_with_component, resource_exists},
        IntoSystemConfigs,
    },
    system::{Query, ResMut, Resource},
};
use froglight_protocol::common::ChunkPosition;
use froglight_world::Chunk;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

use crate::systemset::{UtilityPreUpdateDeferredSet, UtilityPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the ChunkPositionMap resource
    app.init_resource::<ChunkPositionMap>();

    // Update the ChunkPositionMap every `frame`
    app.add_systems(
        PreUpdate,
        ChunkPositionMap::update_chunkposition_map
            .run_if(any_with_component::<Chunk>)
            .run_if(resource_exists::<ChunkPositionMap>)
            .after(UtilityPreUpdateDeferredSet)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`ChunkPosition`]s to [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct ChunkPositionMap(HashMap<ChunkPosition, Entity>);

impl ChunkPositionMap {
    fn update_chunkposition_map(
        query: Query<(Entity, &ChunkPosition), With<Chunk>>,
        mut map: ResMut<Self>,
    ) {
        // Clear the map
        map.clear();

        // Insert all positions and entities into the map
        for (entity, position) in &query {
            #[cfg(debug_assertions)]
            debug_assert!(
                map.insert(*position, entity).is_none(),
                "Two Chunks cannot occupy the same position!"
            );

            #[cfg(not(debug_assertions))]
            map.insert(*position, entity);
        }
    }
}

#[test]
fn chunkpos_map() -> bevy_app::AppExit {
    use bevy_app::{prelude::*, AppExit};
    use bevy_ecs::prelude::*;

    let mut app = App::new();
    app.add_plugins((
        bevy_app::ScheduleRunnerPlugin::default(),
        bevy_time::TimePlugin,
        crate::UtilityPlugin,
    ));

    app.add_systems(
        Update,
        |query: Query<(Entity, &ChunkPosition)>,
         entity_map: Res<ChunkPositionMap>,
         mut events: EventWriter<AppExit>,
         mut commands: Commands| {
            // Check that the ChunkPositionMap contains all entities
            for (entity, entity_id) in &query {
                assert_eq!(entity_map.get(entity_id), Some(&entity));
            }

            #[allow(clippy::cast_possible_wrap)]
            let count = query.iter().count() as i64;

            // Spawn new entities until there are 512, then exit
            if count >= 512 {
                events.send(AppExit::Success);
            } else {
                commands.spawn((ChunkPosition::splat(count), Chunk::new_empty(0, 0)));
            }
        },
    );

    app.run()
}
