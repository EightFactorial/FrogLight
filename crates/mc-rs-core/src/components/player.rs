use bevy::prelude::*;

use crate::schedule::set::GameSet;

pub(super) fn setup(app: &mut App) {
    app.add_event::<CreateControlledPlayerEvent>();

    app.add_systems(
        Update,
        CreateControlledPlayerEvent::listener.in_set(GameSet),
    );
}

/// An event that is fired to create the local player
#[derive(Debug, Clone, PartialEq, Eq, Deref, DerefMut, Event)]
pub struct CreateControlledPlayerEvent(pub Entity);

impl CreateControlledPlayerEvent {
    fn listener(mut events: EventReader<Self>, mut commands: Commands) {
        events.iter().for_each(|Self(entity)| {
            let Some(mut commands) = commands.get_entity(*entity) else {
                error!("Failed to get entity for controlled player!");
                return;
            };

            // Add the player components
            commands
                // Add the player body components
                .insert((ControlledPlayer, TransformBundle::default()))
                .with_children(|parent| {
                    // Create the player head
                    parent.spawn((ControlledPlayerHead, TransformBundle::default()));
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
