use belly::prelude::*;
use bevy::{app::AppExit, prelude::*};

use crate::systems::states::application::{ApplicationState, MenuSet};

use super::MenuRoot;

pub mod backgrounds;

/// Set up the main menu
pub(super) fn setup_menu(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        MainMenu::create
            .run_if(not(any_with_component::<MainMenu>()))
            .in_set(MenuSet),
    );

    backgrounds::setup_backgrounds(app);
}

/// A marker component for the main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenu;

impl MainMenu {
    /// Create the main menu
    fn create(root: Res<MenuRoot>, mut elements: Elements, mut commands: Commands) {
        commands.entity(**root).insert(MainMenu);

        elements.select(".root").add_child(eml! {
            <div class="main-menu">
                <div class="title">
                    "MC-RS"
                </div>
                <div class="buttons">
                    <button class="singleplayer">
                        "Singleplayer"
                    </button>
                    <button class="multiplayer">
                        "Multiplayer"
                    </button>
                    <button class="settings">
                        "Settings"
                    </button>
                    <button class="quit" on:press=|ctx| { ctx.send_event(AppExit) }>
                        "Quit"
                    </button>
                </div>
            </div>
        });
    }
}
