use bevy::prelude::*;
use leafwing_input_manager::{prelude::InputMap, InputManagerBundle};
use mc_rs_core::{
    components::player::CreateControlledPlayerEvent,
    schedule::{set::GameSet, state::ApplicationState},
};
use serde::{Deserialize, Serialize};

pub mod gameplay;
use gameplay::{GameplayActions, GameplayKeybinds};

pub mod inventory;
use inventory::{InventoryActions, InventoryKeybinds};

pub mod movement;
use movement::{MovementActions, MovementKeybinds};

use super::Settings;

pub(super) fn setup(app: &mut App) {
    // Setup submodules
    gameplay::setup(app);
    inventory::setup(app);
    movement::setup(app);

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

// TODO: Accept keyboard and mouse buttons as keybinds
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Keybinds {
    #[serde(default)]
    pub movement: MovementKeybinds,
    #[serde(default)]
    pub gameplay: GameplayKeybinds,
    #[serde(default)]
    pub inventory: InventoryKeybinds,
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
    pub fn new(keybinds: &Keybinds) -> Self {
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
        settings: Res<Settings>,
        mut commands: Commands,
    ) {
        events
            .iter()
            .for_each(|event| match commands.get_entity(**event) {
                None => error!("Could not attach controls to player entity!"),
                Some(mut commands) => {
                    commands.insert(Self::new(&settings.keybinds));
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
