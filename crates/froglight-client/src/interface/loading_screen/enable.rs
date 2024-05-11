use bevy::{prelude::*, render::view::VisibilitySystems};

use super::LoadingScreen;
use crate::systemsets::ClientPostUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add the LoadingScreenVisibility resource
    app.init_resource::<LoadingScreenVisibility>().register_type::<LoadingScreenVisibility>();

    // Add the LoadingScreenVisibility visibility system
    app.add_systems(
        PostUpdate,
        LoadingScreenVisibility::set_loadingsceen_visibility
            .run_if(any_with_component::<LoadingScreen>)
            .run_if(resource_exists_and_changed::<LoadingScreenVisibility>)
            .run_if(not(resource_added::<LoadingScreenVisibility>))
            .before(VisibilitySystems::VisibilityPropagate)
            .in_set(ClientPostUpdateSet),
    );
}

/// A [`Resource`] that enables or disables the [`LoadingScreen`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct LoadingScreenVisibility(pub bool);

impl Default for LoadingScreenVisibility {
    fn default() -> Self { LoadingScreenVisibility(true) }
}

impl LoadingScreenVisibility {
    /// Creates a new [`LoadingScreenVisibility`].
    #[must_use]
    pub const fn new(enabled: bool) -> Self { LoadingScreenVisibility(enabled) }

    /// Toggles the [`LoadingScreenVisibility`].
    pub fn toggle(&mut self) { self.0 = !self.0 }

    /// Returns `true` if the [`LoadingScreenVisibility`] is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool { self.0 }

    /// Returns the [`Visibility`] of the [`LoadingScreen`].
    #[must_use]
    pub const fn get_visibility(&self) -> Visibility {
        if self.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        }
    }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`LoadingScreenVisibility`] is enabled.
    #[must_use]
    pub fn enabled(enable: Res<Self>) -> bool { enable.is_enabled() }

    /// A [`System`](bevy_ecs::system::System) that sets the [`LoadingScreen`]
    /// [`Visibility`].
    fn set_loadingsceen_visibility(
        mut query: Query<&mut Visibility, With<LoadingScreen>>,
        state: Res<Self>,
    ) {
        let new_vis = state.get_visibility();
        debug!("Setting `LoadingScreen` visibility to `{new_vis:?}`");

        for mut vis in &mut query {
            *vis = new_vis;
        }
    }
}
