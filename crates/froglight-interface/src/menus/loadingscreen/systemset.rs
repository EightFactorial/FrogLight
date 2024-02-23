use bevy::prelude::*;

use crate::menus::InterfaceMenuUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add basic `SystemSet`s
    app.configure_sets(PostStartup, LoadingScreenPostStartupSet);
    app.configure_sets(Update, LoadingScreenUpdateSet.in_set(InterfaceMenuUpdateSet));

    // Add states
    app.init_state::<LoadingScreenStateSet>()
        .register_type::<LoadingScreenStateSet>()
        .register_type::<State<LoadingScreenStateSet>>()
        .register_type_data::<State<LoadingScreenStateSet>, ReflectResource>();

    // Add state-specific `SystemSet`s
    app.configure_sets(
        Update,
        LoadingScreenStateSet::Hidden
            .run_if(in_state(LoadingScreenStateSet::Hidden))
            .in_set(LoadingScreenUpdateSet),
    );
    app.configure_sets(
        Update,
        LoadingScreenStateSet::Shown
            .run_if(in_state(LoadingScreenStateSet::Shown))
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
