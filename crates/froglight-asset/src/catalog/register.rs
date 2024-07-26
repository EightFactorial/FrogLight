use std::any::TypeId;

use bevy_app::{App, Update};
use bevy_asset::{Asset, Assets};
use bevy_ecs::{
    entity::Entity,
    prelude::any_with_component,
    query::Changed,
    schedule::IntoSystemConfigs,
    system::{Commands, Query, Res, ResMut},
};
use bevy_log::error;
use bevy_state::state::OnEnter;

use super::{catalog::UntypedAssetMap, systemset::AssetKeyRefreshSet, AssetCatalog, AssetKey};
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

    // Add the change detection system to the `AssetState::Loaded` set
    app.add_systems(
        Update,
        refresh_changed_asset_keys::<A>
            .run_if(any_with_component::<AssetKey<A>>)
            .run_if(any_keys_changed::<A>)
            .in_set(AssetState::Loaded),
    );
}

/// Returns `true` if any [`AssetKey`]s of type `A` have changed.
fn any_keys_changed<A: Asset>(query: Query<(), Changed<AssetKey<A>>>) -> bool { query.is_empty() }

/// Refresh the handles for entities with a changed [`AssetKey`].
fn refresh_changed_asset_keys<A: Asset>(
    query: Query<(Entity, &AssetKey<A>), Changed<AssetKey<A>>>,
    catalog: Res<AssetCatalog>,
    mut assets: ResMut<Assets<A>>,
    mut commands: Commands,
) {
    let Some(untyped_map) = catalog.inner.get(&TypeId::of::<A>()) else {
        error!("No \"{}\" Assets exist in the AssetCatalog!", A::short_type_path());
        return;
    };
    refresh_keys(query.iter(), untyped_map, &mut assets, &mut commands);
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
    refresh_keys(query.iter(), untyped_map, &mut assets, &mut commands);
}

fn refresh_keys<'a, A: Asset>(
    iterator: impl Iterator<Item = (Entity, &'a AssetKey<A>)>,
    untyped_map: &UntypedAssetMap,
    assets: &mut Assets<A>,
    commands: &mut Commands,
) {
    for (entity, key) in iterator {
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
