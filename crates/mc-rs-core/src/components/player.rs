use bevy::prelude::*;

use crate::schedule::set::GameSet;

pub(super) fn setup(app: &mut App) {
    app.add_event::<CreateUserEvent>();
    app.add_systems(Update, CreateUserEvent::listener.in_set(GameSet));
}

/// An event that is fired to create the local player
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Event)]
pub struct CreateUserEvent(pub Entity);

impl CreateUserEvent {
    fn listener(
        query: Query<Entity, With<Camera3d>>,
        mut events: EventReader<Self>,
        mut commands: Commands,
    ) {
        events.read().for_each(|Self(entity)| {
            let Ok(camera_entity) = query.get_single() else {
                error!("Failed to get entity for camera!");
                return;
            };

            let Some(mut commands) = commands.get_entity(*entity) else {
                error!("Failed to get entity for user's player!");
                return;
            };

            // Add the player components
            commands
                // Add the player body components
                .insert((ControlledPlayer, SpatialBundle::default()))
                .with_children(|parent| {
                    // Create the player head
                    parent
                        .spawn((ControlledPlayerHead, SpatialBundle::default()))
                        .add_child(camera_entity);
                });
        });
    }
}

/// A marker component for the local player
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct ControlledPlayer;

/// A marker component for the local player's head
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
pub struct ControlledPlayerHead;
