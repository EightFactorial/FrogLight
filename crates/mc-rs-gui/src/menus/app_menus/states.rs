use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum MainMenuState {
    #[default]
    MainMenu,
    Multiplayer,
    Options,
}
