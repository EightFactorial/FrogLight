#![allow(clippy::module_inception)]

use bevy::prelude::App;

pub mod anvil;
pub mod chest;
pub mod crafting_table;
pub mod furnace;
pub mod inventory;
pub mod smithing_table;

pub(super) fn setup_menus(_app: &mut App) {
    // TODO: Add inventory systems
}
