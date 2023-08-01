use bevy::prelude::*;

pub mod credits_menu;
pub mod inventory_menus;
pub mod main_menu;
pub mod pause_menu;
pub mod server_menu;
pub mod settings_menu;

/// Add menu systems to the app
pub(super) fn setup(app: &mut App) {
    // TODO: Add menu systems

    // Add inventory systems
    inventory_menus::setup_inventories(app);
}
