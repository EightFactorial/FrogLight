#![allow(dead_code)]

use bevy::prelude::*;

/// The current state of the application
///
/// This is used to determine which systems should be run
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}
