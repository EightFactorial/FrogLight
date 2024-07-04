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
use froglight_components::entity::EntityUuid;
#[cfg(feature = "hashbrown")]
use hashbrown::HashMap;

use crate::systemset::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityUuidMap resource
    app.init_resource::<EntityUuidMap>();

    // Update the EntityUuidMap every `frame`
    app.add_systems(
        PreUpdate,
        EntityUuidMap::update_entityuuid_map
            .run_if(any_with_component::<EntityUuid>)
            .run_if(resource_exists::<EntityUuidMap>)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`EntityUuid`]s to [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityUuidMap(HashMap<EntityUuid, Entity>);

impl EntityUuidMap {
    fn update_entityuuid_map(mut map: ResMut<Self>, query: Query<(Entity, &EntityUuid)>) {
        // Clear the map
        map.clear();

        // Insert all uuids and entities into the map
        for (entity, uuid) in &query {
            #[cfg(debug_assertions)]
            debug_assert!(
                map.insert(*uuid, entity).is_none(),
                "Two entities cannot have the same EntityUuid!"
            );

            #[cfg(not(debug_assertions))]
            map.insert(*uuid, entity);
        }
    }
}

#[test]
fn entityuuid_map() -> bevy_app::AppExit {
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
        |query: Query<(Entity, &EntityUuid)>,
         entity_map: Res<EntityUuidMap>,
         mut events: EventWriter<AppExit>,
         mut commands: Commands| {
            // Check that the EntityUuidMap contains all entities
            for (entity, entity_uuid) in &query {
                assert_eq!(entity_map.get(entity_uuid), Some(&entity));
            }

            // Spawn new entities until there are 512, then exit
            let count = query.iter().count() as u128;
            if count >= 512 {
                events.send(AppExit::Success);
            } else {
                commands.spawn(EntityUuid::from(count));
            }
        },
    );

    app.run()
}
