use bevy::{
    app::{App, Update},
    ecs::schedule::{common_conditions::any_with_component, IntoSystemSetConfigs, SystemSet},
};

use super::LoadingScreen;
use crate::systemsets::ClientUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add the LoadingScreenSet SystemSet
    app.configure_sets(
        Update,
        LoadingScreenSet.run_if(any_with_component::<LoadingScreen>).in_set(ClientUpdateSet),
    );
}

/// A [`SystemSet`] for [`LoadingScreen`] systems.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct LoadingScreenSet;