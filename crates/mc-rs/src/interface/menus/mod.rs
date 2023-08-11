use belly::prelude::*;
use bevy::prelude::*;

use crate::{
    interface::menus::main_menu::MainMenu,
    systems::app_state::{ApplicationState, InGameSet, InMenuSet},
};

use self::server::ServerMenu;

pub mod credits;
pub mod main_menu;
pub mod server;
pub mod settings;

/// Add menu systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_event::<MenuPopupEvent>();

    app.add_systems(Startup, (MenuRoot::create_camera, MenuRoot::create));
    app.add_systems(
        Update,
        (
            MenuRoot::show_popups,
            MenuRoot::handle_escape.in_set(InMenuSet),
        ),
    );
    app.add_systems(
        OnEnter(ApplicationState::InGame),
        MenuRoot::hide_all.in_set(InGameSet),
    );

    // TODO: Add menu systems
    main_menu::setup_menu(app);
    server::setup_menu(app);
}

/// The menu root entity id
///
/// All components for the menu should be attached to this entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct MenuRoot(pub Entity);

impl MenuRoot {
    /// Create a new camera bundle
    fn create_camera(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

    /// Load the global stylesheet and create the menu root node
    fn create(mut commands: Commands) {
        commands.add(StyleSheet::load("style/global.ess"));
        commands.add(StyleSheet::load("style/menu.ess"));

        let entity = commands.spawn_empty().id();
        commands.insert_resource(MenuRoot(entity));

        commands.add(eml! {
            <body c:root>
            </body>
        });
    }

    /// Create a popup in the lower right corner
    fn show_popups(mut _events: EventReader<MenuPopupEvent>, mut _elements: Elements) {}

    // A list of possible menus
    const MENUS: [&'static str; 3] = ["div.server-menu", "div.options-menu", "div.main-menu"];

    /// Handle the escape button
    fn handle_escape(mut elements: Elements, input: Res<Input<KeyCode>>) {
        if !input.just_pressed(KeyCode::Escape) {
            return;
        }

        let menus = Self::MENUS.map(|c| {
            let ent = elements.select(c).entities();

            !elements
                .select(".hidden")
                .entities()
                .iter()
                .any(|e| ent.contains(e))
        });

        match menus {
            [true, _, false] => {
                ServerMenu::handle_escape(elements);
            }
            [false, _, true] => {
                // On main menu, do nothing
            }
            [false, false, false] => {
                warn!("Escape pressed and no menus are open!");
                MainMenu::show(elements);
            }
            _ => {
                warn!("Escape pressed, but no menu found to close!");
            }
        }
    }

    /// Hide all menus when the game starts
    fn hide_all(mut elements: Elements) { elements.select("body.root").add_class("hidden"); }
}

/// An event that causes a popup to appear in the lower right corner
#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub struct MenuPopupEvent {
    pub icon: PopupIcon,
    pub message: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PopupIcon {
    Info,
    Warning,
    Error,
}

#[allow(dead_code)]
impl PopupIcon {
    pub fn create_handle(&self, asset_server: &AssetServer) -> Handle<Image> {
        match self {
            Self::Info => asset_server.load("textures/gui/icon/info.png"),
            Self::Warning => asset_server.load("textures/gui/icon/warning.png"),
            Self::Error => asset_server.load("textures/gui/icon/error.png"),
        }
    }
}
