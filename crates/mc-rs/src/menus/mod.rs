use belly::prelude::*;
use bevy::prelude::*;

use crate::systems::app_state::InMenuSet;

use self::server_menu::ServerMenu;

pub mod credits_menu;
pub mod inventory_menus;
pub mod main_menu;
pub mod pause_menu;
pub mod server_menu;
pub mod settings_menu;

/// Add menu systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_systems(Startup, (MenuRoot::create_camera, MenuRoot::create));
    app.add_systems(Update, MenuRoot::handle_escape.in_set(InMenuSet));

    // TODO: Add menu systems
    main_menu::setup_menu(app);
    server_menu::setup_menu(app);
    inventory_menus::setup_menus(app);
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
            <body class="root">
            </body>
        });
    }

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
            _ => {
                warn!("Escape pressed, but no menu found to close!");
                // warn!("Showing main menu");
                // MainMenu::show(elements);
            }
        }
    }
}
