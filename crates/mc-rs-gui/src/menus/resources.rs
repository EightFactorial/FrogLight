use bevy::{asset::RecursiveDependencyLoadState, prelude::*};

pub(super) fn setup(app: &mut App) { app.init_resource::<MenuResources>(); }

#[derive(Debug, Default, Clone, PartialEq, Eq, Deref, DerefMut, Resource)]
pub struct MenuResources {
    pub handles: Vec<UntypedHandle>,
}

impl MenuResources {
    /// Returns `true` if all of the [`UntypedHandle`]s in [`MenuResources`] are loaded.
    pub fn loaded(res: Res<MenuResources>, assets: Res<AssetServer>) -> bool {
        for handle in &res.handles {
            let state = assets.get_recursive_dependency_load_state(handle.id());

            if !matches!(state, None | Some(RecursiveDependencyLoadState::Loaded)) {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Waiting for {handle:?}");

                return false;
            }
        }

        true
    }
}
