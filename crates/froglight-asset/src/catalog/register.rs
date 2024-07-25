use std::any::TypeId;

use bevy_app::{App, Update};
use bevy_asset::{Asset, Assets};
use bevy_ecs::{
    entity::Entity,
    prelude::any_with_component,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, ResMut},
};
use bevy_log::error;
use bevy_state::state::OnEnter;

use super::{systemset::AssetKeyRefreshSet, AssetCatalog, AssetKey};
use crate::AssetState;

/// Add systems for refreshing [`AssetKey`]s of type `A`.
pub(super) fn add_systems<A: Asset>(app: &mut App) {
    app.register_type::<AssetKey<A>>();

    // Add the refresh system to the `AssetKeyRefreshSet`s
    app.add_systems(
        OnEnter(AssetState::Loaded),
        refresh_asset_keys::<A>
            .run_if(any_with_component::<AssetKey<A>>)
            .in_set(AssetKeyRefreshSet),
    );
    app.add_systems(
        Update,
        refresh_asset_keys::<A>
            .run_if(any_with_component::<AssetKey<A>>)
            .in_set(AssetKeyRefreshSet),
    );
}

/// Update the handles for all entities with an [`AssetKey`].
fn refresh_asset_keys<A: Asset>(
    query: Query<(Entity, &AssetKey<A>)>,
    catalog: Res<AssetCatalog>,
    mut assets: ResMut<Assets<A>>,
    mut commands: Commands,
) {
    let Some(untyped_map) = catalog.inner.get(&TypeId::of::<A>()) else {
        error!("No \"{}\" Assets exist in the AssetCatalog!", A::short_type_path());
        return;
    };

    for (entity, key) in &query {
        if let Some(untyped_id) = untyped_map.get(key.as_ref()) {
            if let Some(asset_handle) =
                assets.get_strong_handle(untyped_id.typed_debug_checked::<A>())
            {
                #[cfg(debug_assertions)]
                bevy_log::trace!(
                    "AssetKey::<{}>: \"{}\" -> \"{}\"",
                    A::short_type_path(),
                    key.as_ref(),
                    asset_handle.id()
                );

                commands.entity(entity).insert(asset_handle);
            } else {
                error!(
                    "AssetKey::<{}>: \"{}\" refers to an asset that does not exist!",
                    A::short_type_path(),
                    key.as_ref()
                );
            }
        } else {
            error!(
                "AssetKey::<{}>: \"{}\" does not refer to any known asset!",
                A::short_type_path(),
                key.as_ref()
            );
        }
    }
}
