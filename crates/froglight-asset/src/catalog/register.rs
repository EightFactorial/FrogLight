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

use super::{systemset::AssetKeyRefreshSet, AssetCatalog, AssetKey};

/// Add systems for refreshing [`AssetKey`]s of type `A`.
pub(super) fn add_systems<A: Asset>(app: &mut App) {
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
    let Some(untyped_map) = catalog.storage.get(&TypeId::of::<A>()) else { return };
    for (entity, key) in &query {
        if let Some(untyped_id) = untyped_map.get(key.as_ref()) {
            if let Some(asset_handle) =
                assets.get_strong_handle(untyped_id.typed_debug_checked::<A>())
            {
                commands.entity(entity).insert(asset_handle);
            } else {
                error!("AssetKey \"{}\" refers to an asset that does not exist!", key.as_ref());
            }
        } else {
            error!("AssetKey \"{}\" does not refer to any known asset!", key.as_ref());
        }
    }
}
