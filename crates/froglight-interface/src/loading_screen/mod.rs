//! A loading screen displayed while the client is loading.

use bevy::app::{App, Plugin};

pub mod elements;

mod child;


mod screen;
pub use screen::LoadingScreen;

/// A [`Plugin`] that creates a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        elements::build(app);
        screen::build(app);
    }
}
