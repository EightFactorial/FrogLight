#[cfg(not(feature = "hashbrown"))]
use std::collections::{HashMap, HashSet};

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
use froglight_protocol::common::{ChunkPosition, EntityId};
#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};

use crate::systemset::{UtilityPreUpdateDeferredSet, UtilityPreUpdateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityChunkMap resource
    app.init_resource::<EntityChunkMap>();

    // Update the EntityChunkMap every `frame`
    app.add_systems(
        PreUpdate,
        EntityChunkMap::update_entitychunk_map
            .run_if(any_with_component::<EntityId>)
            .run_if(resource_exists::<EntityChunkMap>)
            .after(UtilityPreUpdateDeferredSet)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`EntityId`] [`ChunkPosition`]s to a list of [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityChunkMap {
    #[deref]
    map: HashMap<ChunkPosition, Vec<Entity>>,
    updates: HashSet<ChunkPosition>,
}

impl EntityChunkMap {
    fn update_entitychunk_map(
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

#[test]
#[expect(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
fn entitychunk_map() -> bevy_app::AppExit {
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
         entity_map: Res<EntityChunkMap>,
         mut events: EventWriter<AppExit>,
         mut commands: Commands| {
            // Check that the EntityChunkMap contains all entities
            for (entity, entity_pos) in &query {
                assert_eq!(
                    entity_map.get(entity_pos).map(|list| list.contains(&entity)),
                    Some(true)
                );
            }

            let count = query.iter().count();
            let pos = ChunkPosition::splat((count % 13) as i64);
            let id = EntityId::new(count as u32);

            // Spawn new entities until there are 512, then exit
            if count >= 512 {
                // Check that the EntityChunkMap contains the correct number of entities
                assert_eq!(entity_map.get(&pos).unwrap().len(), 39);

                events.send(AppExit::Success);
            } else {
                commands.spawn((id, pos));
            }
        },
    );

    app.run()
}
