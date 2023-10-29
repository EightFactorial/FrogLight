use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputMap, InputManagerBundle};
use mc_rs_core::{
    components::player::CreateControlledPlayerEvent,
    schedule::{set::GameSet, state::ApplicationState},
};

use super::{
    gameplay::GameplayActions, inventory::InventoryActions, movement::MovementActions, KeyBinds,
};

pub(super) fn setup(app: &mut App) {
    // Add systems to add and remove controls
    app.add_systems(
        Update,
        PlayerControllerBundle::add_to_player.in_set(GameSet),
    );

    app.add_systems(
        OnExit(ApplicationState::InGame),
        PlayerControllerBundle::remove_controls,
    );
}

/// Bundle of all player controls.
#[derive(Bundle)]
pub struct PlayerControllerBundle {
    movement: InputManagerBundle<MovementActions>,
    gameplay: InputManagerBundle<GameplayActions>,
    inventory: InputManagerBundle<InventoryActions>,
}

impl PlayerControllerBundle {
    /// Create a new bundle of player controls.
    pub fn new(keybinds: &KeyBinds) -> Self {
        Self {
            movement: InputManagerBundle {
                input_map: keybinds.movement.into(),
                ..Default::default()
            },
            gameplay: InputManagerBundle {
                input_map: keybinds.gameplay.into(),
                ..Default::default()
            },
            inventory: InputManagerBundle {
                input_map: keybinds.inventory.into(),
                ..Default::default()
            },
        }
    }

    /// Add controls to the player entity.
    fn add_to_player(
        mut events: EventReader<CreateControlledPlayerEvent>,
        keybinds: Res<KeyBinds>,
        mut commands: Commands,
    ) {
        events
            .iter()
            .for_each(|event| match commands.get_entity(**event) {
                None => error!("Could not attach controls to player entity!"),
                Some(mut commands) => {
                    commands.insert(Self::new(&keybinds));
                }
            });
    }

    /// Remove controls from the player entity.
    fn remove_controls(
        query: Query<Entity, With<InputMap<MovementActions>>>,
        mut commands: Commands,
    ) {
        query.for_each(|entity| {
            if let Some(mut commands) = commands.get_entity(entity) {
                commands.remove::<Self>();
            }
        });
    }
}
