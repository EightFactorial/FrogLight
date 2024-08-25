use bevy_app::{App, Update};
use bevy_ecs::schedule::IntoSystemConfigs;

use crate::{AssetProcess, ResourcePackList};

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    // Check if all `ResourcePack`s are loaded.
    app.add_systems(
        Update,
        success::enter_processing_state
            .ambiguous_with(error::enter_waiting_state)
            .run_if(success::all_assets_loaded)
            .in_set(AssetProcess::Loading),
    );

    // Check if any `ResourcePack`s have errors.
    app.add_systems(
        Update,
        error::enter_waiting_state
            .ambiguous_with(success::all_assets_loaded)
            .run_if(error::any_asset_errors)
            .in_set(AssetProcess::Loading),
    );
}

mod success {
    use bevy_asset::{AssetServer, RecursiveDependencyLoadState};
    use bevy_ecs::system::{Res, ResMut};
    use bevy_log::debug;
    use bevy_state::state::NextState;

    use super::{AssetProcess, ResourcePackList};

    /// Check if all [`ResourcePack`](crate::ResourcePack)s are loaded.
    pub(super) fn all_assets_loaded(list: Res<ResourcePackList>, assets: Res<AssetServer>) -> bool {
        list.iter().all(|h| {
            assets.get_recursive_dependency_load_state(h.id())
                == Some(RecursiveDependencyLoadState::Loaded)
        })
    }

    /// Enter the [`AssetProcess::Processing`] state.
    pub(super) fn enter_processing_state(mut state: ResMut<NextState<AssetProcess>>) {
        debug!("AssetProcess: Starting processing...");
        state.set(AssetProcess::Processing);
    }
}

mod error {
    use bevy_asset::{AssetServer, RecursiveDependencyLoadState};
    use bevy_ecs::system::{Res, ResMut};
    use bevy_log::error;
    use bevy_state::state::NextState;

    use super::{AssetProcess, ResourcePackList};

    /// Check if any [`ResourcePack`](crate::ResourcePack)s have errors.
    pub(super) fn any_asset_errors(list: Res<ResourcePackList>, assets: Res<AssetServer>) -> bool {
        let mut value = false;

        for h in list.iter() {
            if let Some(state) = assets.get_recursive_dependency_load_state(h.id()) {
                if state == RecursiveDependencyLoadState::Failed {
                    // Log the `ResourcePack` that failed to load.
                    if let Some(path) = h.path() {
                        error!("ResourcePack `{}` failed to load", path);
                    } else {
                        error!("ResourcePack `{}` failed to load", h.id());
                    }

                    value = true;
                }
            } else {
                // Log the missing `ResourcePack`
                if let Some(path) = h.path() {
                    error!("ResourcePack `{}` failed to load, does not exist", path);
                } else {
                    error!("ResourcePack `{}` failed to load, does not exist", h.id());
                }

                value = true;
            }
        }

        value
    }

    /// Enter the [`AssetProcess::Waiting`] state.
    pub(super) fn enter_waiting_state(mut state: ResMut<NextState<AssetProcess>>) {
        error!("AssetProcess: Error, re-entering waiting...");
        state.set(AssetProcess::Waiting);
    }
}
