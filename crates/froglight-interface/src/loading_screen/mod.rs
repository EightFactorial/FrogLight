//! A loading screen displayed while the client is loading.

use bevy::app::{App, Plugin};

mod marker;
pub use marker::LoadingScreen;

mod holder;
pub use holder::{LoadingScreenChild, LoadingScreenHolder};

/// A [`Plugin`] that creates a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LoadingScreen>()
            .register_type::<LoadingScreenChild>()
            .register_type::<LoadingScreenHolder>();
    }
}
