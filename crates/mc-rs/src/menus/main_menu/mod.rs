use belly::prelude::*;
use bevy::{app::AppExit, prelude::*};

use crate::systems::app_state::{ApplicationState, InMenuSet};

use super::MenuRoot;

pub mod backgrounds;

/// Set up the main menu
pub(super) fn setup_menu(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        (
            MainMenu::show.run_if(any_with_component::<MainMenu>()),
            MainMenu::create.run_if(not(any_with_component::<MainMenu>())),
        )
            .in_set(InMenuSet),
    );

    backgrounds::setup_backgrounds(app);
}

/// A marker component for the main menu
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenu;

impl MainMenu {
    /// Show the main menu
    pub fn show(mut elements: Elements) {
        elements
            .select(".root div.main-menu")
            .remove_class("hidden");

        elements
            .select(".root div.main-background")
            .remove_class("hidden");
    }

    /// Create the main menu
    fn create(root: Res<MenuRoot>, mut elements: Elements, mut commands: Commands) {
        commands.entity(**root).insert(MainMenu);

        elements.select(".root").add_child(eml! {
            <div class="main-menu">
                <div class="main-menu-menu">
                    <div class="main-menu-title">
                        <img class="main-menu-logo" src="textures/gui/title/logo.png" />
                    </div>
                    <div class="main-menu-buttons">
                        <button class="button" on:press=|ctx| { Self::click_button(ctx, ".servers-menu") }>
                            "Servers"
                        </button>
                        <button class="button" on:press=|ctx| { Self::click_button(ctx, ".options-menu") }>
                            "Options"
                        </button>
                        <button class="button" on:press=|ctx| { ctx.send_event(AppExit) }>
                            "Quit"
                        </button>
                    </div>
                </div>
                <div class="menu-version">
                    "MC-RS v"
                    { env!("CARGO_PKG_VERSION") }
                </div>
                <div class="menu-disclaimer">
                    "ALPHA SOFTWARE - USE AT YOUR OWN RISK"
                </div>
            </div>
        });
    }

    fn click_button(ctx: &mut EventContext<impl Event>, query: &str) {
        ctx.select(".root div.main-menu").add_class("hidden");
        ctx.select(".root div.main-background").add_class("hidden");

        ctx.select(&format!(".root div{query}"))
            .remove_class("hidden");
    }
}
