use bevy::prelude::*;
use froglight_core::resources::LoadingScreenEnable;

use super::LoadingScreenRootNode;
use crate::menus::{
    mainmenu::systemset::MainMenuUpdateSet, multiplayermenu::systemset::MultiplayerMenuUpdateSet,
    panorama::MainMenuPanoramaSet, settingsmenu::systemset::SettingsMenuUpdateSet,
    InterfaceMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add basic `SystemSet`s
    app.configure_sets(
        PostStartup,
        LoadingScreenPostStartupSet.run_if(LoadingScreenEnable::is_enabled),
    );
    app.configure_sets(
        Update,
        LoadingScreenUpdateSet
            .run_if(LoadingScreenEnable::is_enabled)
            .ambiguous_with(MainMenuPanoramaSet)
            .before(MainMenuUpdateSet)
            .before(MultiplayerMenuUpdateSet)
            .before(SettingsMenuUpdateSet)
            .in_set(InterfaceMenuUpdateSet),
    );

    // Add states
    app.init_state::<LoadingScreenStateSet>()
        .register_type::<LoadingScreenStateSet>()
        .register_type::<State<LoadingScreenStateSet>>()
        .register_type_data::<State<LoadingScreenStateSet>, ReflectResource>();

    // Add state-specific `SystemSet`s
    app.configure_sets(
        Update,
        (
            LoadingScreenStateSet::Hidden
                .run_if(in_state(LoadingScreenStateSet::Hidden))
                .run_if(any_with_component::<LoadingScreenRootNode>),
            LoadingScreenStateSet::Shown
                .run_if(in_state(LoadingScreenStateSet::Shown))
                .run_if(any_with_component::<LoadingScreenRootNode>),
        )
            .in_set(LoadingScreenUpdateSet),
    );
}

/// A [`SystemSet`] for loading screen systems that should run during
/// [`PostStartup`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct LoadingScreenPostStartupSet;

/// A [`SystemSet`] for loading screen systems that should run during
/// [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct LoadingScreenUpdateSet;

/// State-specific [`SystemSet`]s for loading screen systems that should run
/// during [`Update`].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States, SystemSet)]
pub(crate) enum LoadingScreenStateSet {
    Hidden,
    #[default]
    Shown,
}
