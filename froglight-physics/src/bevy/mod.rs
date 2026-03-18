//! TODO

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use froglight_entity::{bevy::EntityBundleEvent, prelude::EntityBundle};

use crate::prelude::*;

/// A [`Plugin`] that...
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform>().register_type::<PreviousTransform>();
        app.register_type::<Velocity>().register_type::<PreviousVelocity>();
        app.register_type::<Acceleration>().register_type::<PreviousAcceleration>();
        app.register_type::<OnGround>().register_type::<PreviousOnGround>();
        app.register_type::<PhysicsState>().register_type::<PhysicsController>();

        app.add_observer(Self::entity_physics_observer);
    }
}

impl PhysicsPlugin {
    /// An [`Observer`] that listens for [`EntityBundleEvent`]s and attaches
    /// [`EntityAabb`]s and [`PhysicsState`]s to entities.
    pub fn entity_physics_observer(
        event: On<EntityBundleEvent>,
        query: Query<(&EntityBundle, Option<&PhysicsState>)>,
        mut commands: Commands,
    ) {
        if let Ok((bundle, state)) = query.get(event.entity()) {
            let mut commands = commands.entity(event.entity());

            commands.insert(*bundle.metadata().aabb());
            if state.is_none() {
                commands.insert(PhysicsState::default());
            }
        }
    }
}
