use bevy::prelude::*;
use mc_rs_core::schedule::state::ApplicationState;

mod background;
use background::MainMenuBackground;

mod buttons;
use buttons::MainMenuButtons;

mod cube;

mod title;
use title::MainMenuTitle;

use crate::{
    interface::state::MainMenuState,
    traits::interface::{InterfaceComponent, MenuVisibility},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct MainMenuInterface;

impl InterfaceComponent for MainMenuInterface {
    fn setup(app: &mut App) {
        // Show the main menu when entering the ApplicationState::MainMenu state
        app.add_systems(
            OnEnter(ApplicationState::MainMenu),
            MainMenuInterface::show.run_if(
                any_with_component::<MainMenuInterface>().and_then(in_state(MainMenuState::Main)),
            ),
        );

        // Show the main menu when entering the MainMenuState::Main state
        app.add_systems(
            OnEnter(MainMenuState::Main),
            MainMenuInterface::show.run_if(
                in_state(ApplicationState::MainMenu)
                    .and_then(any_with_component::<MainMenuInterface>()),
            ),
        );
        // Hide the main menu when exiting the MainMenuState::Main state
        app.add_systems(
            OnExit(MainMenuState::Main),
            MainMenuInterface::hide.run_if(
                in_state(ApplicationState::MainMenu)
                    .and_then(any_with_component::<MainMenuInterface>()),
            ),
        );

        MainMenuBackground::setup(app);
        MainMenuTitle::setup(app);
        MainMenuButtons::setup(app);
    }

    fn build(root: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building MainMenuInterface");

        // Create the main menu node
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,

                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::NONE),
            visibility: world.get_menu_visibility(ApplicationState::MainMenu, MainMenuState::Main),
            ..Default::default()
        };

        // Spawn the main menu as a child of the root node
        let main_menu = world.spawn((MainMenuInterface, node)).id();
        world.entity_mut(root).add_child(main_menu);

        // Build interface components
        MainMenuBackground::build(main_menu, world);
        MainMenuTitle::build(main_menu, world);
        MainMenuButtons::build(main_menu, world);
    }
}

impl MainMenuInterface {
    fn show(mut query: Query<&mut Visibility, With<MainMenuInterface>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing MainMenuInterface");

        query.for_each_mut(|mut vis| {
            *vis = Visibility::Visible;
        });
    }

    fn hide(mut query: Query<&mut Visibility, With<MainMenuInterface>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding MainMenuInterface");

        query.for_each_mut(|mut vis| {
            *vis = Visibility::Hidden;
        });
    }
}
