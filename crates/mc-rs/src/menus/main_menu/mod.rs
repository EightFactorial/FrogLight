use belly::prelude::*;
use bevy::{app::AppExit, prelude::*};
use rand::seq::IteratorRandom;

use crate::systems::app_state::{ApplicationState, InMenuSet};

use self::backgrounds::MainMenuBackground;

use super::{server_menu::ServerMenuPing, MenuRoot};

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
        elements.select("div.main-menu").remove_class("hidden");
        MainMenuBackground::show(elements);
    }

    /// Create the main menu
    fn create(root: Res<MenuRoot>, mut elements: Elements, mut commands: Commands) {
        commands.entity(**root).insert(MainMenu);

        elements.select(".root").add_child(eml! {
            <div c:main-menu>
                <div c:main-menu-menu>
                    <div c:main-menu-title>
                        <img c:main-menu-logo src="textures/gui/title/logo.png"/>
                        <div c:main-menu-subtitle><span>{ Self::get_subtitle() }</span></div>
                    </div>
                    <div c:main-menu-buttons>
                        <button c:button on:press=|ctx| { Self::click_button(ctx, "div.server-menu"); ctx.send_event(ServerMenuPing) }>
                            "Servers"
                        </button>
                        <button c:button on:press=|ctx| { Self::click_button(ctx, "div.options-menu") }>
                            "Options"
                        </button>
                        <button c:button on:press=|ctx| { ctx.send_event(AppExit) }>
                            "Quit"
                        </button>
                    </div>
                </div>
                <div c:menu-version>
                    "MC-RS v"
                    { env!("CARGO_PKG_VERSION") }
                </div>
                <div c:menu-disclaimer>
                    "ALPHA SOFTWARE - USE AT YOUR OWN RISK"
                </div>
            </div>
        });
    }

    /// Function to handle button clicks
    fn click_button(ctx: &mut EventContext<impl Event>, query: &str) {
        ctx.select("div.main-menu").add_class("hidden");
        ctx.select("div.main-background").add_class("hidden");

        ctx.select(query).remove_class("hidden");
    }

    /// The list of possible subtitles
    const SUBTITLES: &str = include_str!("../../../assets/language/menu_subtitle.txt");

    /// Get a random subtitle from the list
    fn get_subtitle() -> &'static str {
        let mut rng = rand::thread_rng();

        Self::SUBTITLES
            .lines()
            .choose(&mut rng)
            .unwrap_or(Self::SUBTITLES.lines().next().expect("No subtitles found"))
    }
}
