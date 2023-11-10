use bevy::prelude::*;

pub(super) fn setup(app: &mut App) {
    app.add_state::<MainMenuState>()
        .add_state::<SettingsState>()
        .add_state::<GuiState>();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum MainMenuState {
    #[default]
    Main,
    Singleplayer,
    Multiplayer,
    Realms,
    Settings,
    Quit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum SettingsState {
    #[default]
    Overview,
    Video,
    Audio,
    Controls,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GuiState {
    #[default]
    None,
    Inventory,
    Chest,
    CraftingTable,
    Stonecutter,
    Loom,
    Furnace,
    BlastFurnace,
    Smoker,
    BrewingStand,
    EnchantmentTable,
    Anvil,
    Beacon,
    Villager,
    Horse,
    Hopper,
    Dispenser,
}
