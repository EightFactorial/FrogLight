use bevy::prelude::*;

use crate::systems::states::application::{ApplicationState, MenuSet};

#[derive(Debug, Default, Clone, Copy)]
pub struct SplashPlugin;

/// A system set that runs when the [ApplicationState] is
/// [SplashScreen](ApplicationState::SplashScreen)
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct SplashSet;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            SplashSet
                .run_if(in_state(ApplicationState::SplashScreen))
                .ambiguous_with(MenuSet),
        );

        // TODO: Add splash screen
    }
}
