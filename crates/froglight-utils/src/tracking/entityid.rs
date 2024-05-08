use bevy_app::{App, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    entity::Entity,
    schedule::{common_conditions::any_with_component, IntoSystemConfigs},
    system::{Query, ResMut, Resource},
};
use froglight_components::entity::EntityId;
use hashbrown::HashMap;

use crate::systemsets::UtilityPreUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Initialize the EntityIdMap resource
    app.init_resource::<EntityIdMap>();

    // Update the EntityIdMap every frame
    app.add_systems(
        PreUpdate,
        EntityIdMap::update_entityids
            .run_if(any_with_component::<EntityId>)
            .in_set(UtilityPreUpdateSet),
    );
}

/// A map of [`EntityId`]s to [`Entity`]s.
///
/// Much faster than using a query and iterating over all entities.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct EntityIdMap(HashMap<EntityId, Entity>);

impl EntityIdMap {
    fn update_entityids(query: Query<(Entity, &EntityId)>, mut map: ResMut<Self>) {
        // Clear the map
        map.clear();

        // Insert all ids and entities into the map
        for (entity, id) in &query {
            map.insert(*id, entity);
        }
    }
}
