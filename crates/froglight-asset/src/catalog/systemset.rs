use bevy_app::{App, Update};
use bevy_ecs::{
    prelude::resource_changed,
    schedule::{IntoSystemSetConfigs, SystemSet},
};
use bevy_state::state::OnEnter;

use super::AssetCatalog;
use crate::AssetState;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.configure_sets(OnEnter(AssetState::Loaded), AssetKeyRefreshSet);
    app.configure_sets(
        Update,
        AssetKeyRefreshSet.run_if(resource_changed::<AssetCatalog>).in_set(AssetState::Loaded),
    );
}

/// A [`SystemSet`] that refreshes all [`AssetKey`](super::AssetKey)s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(super) struct AssetKeyRefreshSet;
