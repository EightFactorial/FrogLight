use bevy::prelude::*;

mod background;
use background::MainMenuBackground;
use mc_rs_core::schedule::state::ApplicationState;

use crate::{interface::state::MainMenuState, traits::interface::SubInterface};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuInterface;

impl SubInterface for MainMenuInterface {
    fn setup(app: &mut App) {
        // TODO: Add systems

        MainMenuBackground::setup(app);
    }

    fn build(root: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuInterface");

        // Set visibility based on current state
        let app_state = world.resource::<State<ApplicationState>>();
        let menu_state = world.resource::<State<MainMenuState>>();
        let visibility = match (**app_state, **menu_state) {
            (ApplicationState::MainMenu, MainMenuState::Main) => Visibility::Visible,
            _ => Visibility::Hidden,
        };

        // TODO: Build main menu interface
        let menu_node = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            visibility,
            ..Default::default()
        };

        // Spawn the main menu
        let main_menu = world.spawn((MainMenuInterface, menu_node)).id();
        world.entity_mut(root).add_child(main_menu);

        // Build sub-interfaces
        MainMenuBackground::build(main_menu, world);
    }
}
