#[cfg(not(feature = "hashbrown"))]
use std::collections::HashMap;

use bevy_app::{App, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::Entity,
    schedule::{
        common_conditions::{any_with_component, resource_exists},
        IntoSystemConfigs,
    },
    system::{Query, ResMut, Resource},
};
use froglight_components::entity::EntityId;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

use crate::systemset::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityIdMap resource
    app.init_resource::<EntityIdMap>();

    // Update the EntityIdMap every `frame`
    app.add_systems(
        PreUpdate,
        EntityIdMap::update_entityid_map
            .run_if(any_with_component::<EntityId>)
            .run_if(resource_exists::<EntityIdMap>)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`EntityId`]s to [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityIdMap(HashMap<EntityId, Entity>);

impl EntityIdMap {
    fn update_entityid_map(query: Query<(Entity, &EntityId)>, mut map: ResMut<Self>) {
        // Clear the map
        map.clear();

        // Insert all ids and entities into the map
        for (entity, id) in &query {
            #[cfg(debug_assertions)]
            debug_assert!(
                map.insert(*id, entity).is_none(),
                "Two entities cannot have the same EntityId!"
            );

            #[cfg(not(debug_assertions))]
            map.insert(*id, entity);
        }
    }
}

#[test]
fn entityid_map() {
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
        |query: Query<(Entity, &EntityId)>,
         entity_map: Res<EntityIdMap>,
         mut events: EventWriter<AppExit>,
         mut commands: Commands| {
            // Check that the EntityIdMap contains all entities
            for (entity, entity_id) in &query {
                assert_eq!(entity_map.get(entity_id), Some(&entity));
            }

            #[allow(clippy::cast_possible_truncation)]
            let count = query.iter().count() as u32;

            // Spawn new entities until there are 512, then exit
            if count >= 512 {
                events.send(AppExit);
            } else {
                commands.spawn(EntityId::new(count));
            }
        },
    );

    app.run();
}
