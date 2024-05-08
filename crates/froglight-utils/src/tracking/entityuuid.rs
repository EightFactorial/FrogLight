use bevy_app::{App, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::Entity,
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Query, ResMut, Resource},
};
use froglight_components::entity::EntityUuid;
use hashbrown::HashMap;

use crate::systemsets::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityUuidMap resource
    app.init_resource::<EntityUuidMap>();

    // Update the EntityUuidMap every frame
    app.add_systems(
        PreUpdate,
        EntityUuidMap::update_entityuuids
            .run_if(any_with_component::<EntityUuid>)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`EntityUuid`]s to [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityUuidMap(HashMap<EntityUuid, Entity>);

impl EntityUuidMap {
    fn update_entityuuids(mut map: ResMut<Self>, query: Query<(Entity, &EntityUuid)>) {
        // Clear the map
        map.clear();

        // Insert all uuids and entities into the map
        for (entity, uuid) in &query {
            map.insert(*uuid, entity);
        }
    }
}
