use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum SettingsState {
    #[default]
    Overview,
    Video,
    Audio,
    Controls,
}
