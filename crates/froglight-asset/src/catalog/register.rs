use std::any::TypeId;

use bevy_app::{App, Update};
use bevy_asset::{Asset, Assets};
use bevy_ecs::{
    entity::Entity,
    prelude::any_with_component,
    query::{Changed, QueryFilter, QueryParIter},
    schedule::IntoSystemConfigs,
    system::{Local, ParallelCommands, Query, Res, ResMut},
};
use bevy_log::error;
use bevy_state::state::OnEnter;
use parking_lot::RwLock;

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
    commands: ParallelCommands,

    mut assets: ResMut<Assets<A>>,
    local: Local<RwLock<Option<Assets<A>>>>,
) {
    // Initialize `local`
    if local.read().is_none() {
        *local.write() = Some(Assets::default());
    }

    // Swap `assets` into `local`
    std::mem::swap(local.write().as_mut().unwrap(), &mut assets);

    // Refresh the keys
    if let Some(untyped_map) = catalog.inner.get(&TypeId::of::<A>()) {
        refresh_keys(query.par_iter(), untyped_map, &local, commands);
    } else {
        error!("No \"{}\" Assets exist in the AssetCatalog!", A::short_type_path());
    };

    // Swap `local` back into `assets`
    std::mem::swap(local.write().as_mut().unwrap(), &mut *assets);
}

/// Update the handles for all entities with an [`AssetKey`].
fn refresh_asset_keys<A: Asset>(
    query: Query<(Entity, &AssetKey<A>)>,
    catalog: Res<AssetCatalog>,
    commands: ParallelCommands,

    mut assets: ResMut<Assets<A>>,
    local: Local<RwLock<Option<Assets<A>>>>,
) {
    // Initialize `local`
    if local.read().is_none() {
        *local.write() = Some(Assets::default());
    }

    // Swap `assets` into `local`
    std::mem::swap(local.write().as_mut().unwrap(), &mut assets);

    // Refresh the keys
    if let Some(untyped_map) = catalog.inner.get(&TypeId::of::<A>()) {
        refresh_keys(query.par_iter(), untyped_map, &local, commands);
    } else {
        error!("No \"{}\" Assets exist in the AssetCatalog!", A::short_type_path());
    };

    // Swap `local` back into `assets`
    std::mem::swap(local.write().as_mut().unwrap(), &mut *assets);
}

/// Create [`Handle`](bevy_asset::Handle)s for all entities with an
/// [`AssetKey`].
///
/// Runs in parallel using a [`QueryParIter`].
fn refresh_keys<'a, A: Asset, F: QueryFilter>(
    iterator: QueryParIter<'a, 'a, (Entity, &AssetKey<A>), F>,
    untyped_map: &UntypedAssetMap,
    assets: &RwLock<Option<Assets<A>>>,
    commands: ParallelCommands,
) {
    iterator.for_each(|(entity, key)| {
        if let Some(untyped_id) = untyped_map.get(key.as_ref()) {
            if let Some(asset_handle) = assets
                .write()
                .as_mut()
                .and_then(|a| a.get_strong_handle(untyped_id.typed_debug_checked::<A>()))
            {
                #[cfg(debug_assertions)]
                bevy_log::trace!(
                    "AssetKey::<{}>: \"{}\" -> \"{}\"",
                    A::short_type_path(),
                    key.as_ref(),
                    asset_handle.id()
                );

                commands.command_scope(|mut commands| {
                    commands.entity(entity).insert(asset_handle);
                });
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
    });
}
