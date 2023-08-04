use belly::prelude::*;
use bevy::prelude::*;

pub mod credits_menu;
pub mod inventory_menus;
pub mod main_menu;
pub mod pause_menu;
pub mod server_menu;
pub mod settings_menu;

/// Add menu systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_systems(Startup, (MenuRoot::create_camera, MenuRoot::create));

    // TODO: Add menu systems
    main_menu::setup_menu(app);
    inventory_menus::setup_menus(app);
}

/// The menu root node
///
/// All menus should be children of this node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenuRoot;

impl MenuRoot {
    /// Create a new camera bundle
    fn create_camera(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

    /// Load the global stylesheet and create the menu root node
    fn create(mut commands: Commands) {
        commands.add(StyleSheet::load("style/global.ess"));
        commands.add(StyleSheet::load("style/menu.ess"));

        commands.spawn(MenuRoot);
        commands.add(eml! {
            <body class="root">
            </body>
        });
    }
}
