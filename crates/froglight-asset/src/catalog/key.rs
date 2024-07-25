use std::marker::PhantomData;

use bevy_asset::{Asset, Assets, Handle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    component::{Component, ComponentHooks, ComponentId, StorageType},
    entity::Entity,
    reflect::ReflectComponent,
    world::DeferredWorld,
};
use bevy_log::{error, warn};
use bevy_reflect::Reflect;
use bevy_state::state::State;
use froglight_common::ResourceKey;

use crate::{AssetCatalog, AssetState};

/// A key to an [`Asset`] in the [`AssetCatalog`].
///
/// Automatically inserts a [`Handle`] to the asset when added to an entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct AssetKey<A: Asset> {
    #[deref]
    key: ResourceKey,
    #[reflect(ignore)]
    _a: PhantomData<A>,
}

impl<A: Asset> Component for AssetKey<A> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(Self::on_add).on_remove(Self::on_remove);
    }
}

impl<A: Asset> AssetKey<A> {
    /// Creates a new [`AssetKey`] from a [`ResourceKey`].
    #[must_use]
    pub const fn new(key: ResourceKey) -> Self { Self { key, _a: PhantomData } }

    /// Returns the [`ResourceKey`] of the asset.
    #[must_use]
    #[inline]
    pub fn key(&self) -> &ResourceKey { self.as_ref() }

    /// Looks up the [`AssetId`](bevy_asset::AssetId) in the [`AssetCatalog`]
    /// and inserts the [`Asset`]'s [`Handle`].
    fn on_add(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        // Get the AssetKey and AssetCatalog
        let asset_key = world.get::<AssetKey<A>>(entity).unwrap();
        let catalog = world.resource::<AssetCatalog>();

        // Get the AssetId from the AssetCatalog
        if let Some(asset_id) = catalog.get::<A>(asset_key) {
            let mut assets = world.resource_mut::<Assets<A>>();

            // Create a Handle to the asset and insert it into the entity
            if let Some(asset_handle) = assets.get_strong_handle(asset_id) {
                world.commands().entity(entity).insert(asset_handle);
            } else {
                let asset_key = world.get::<AssetKey<A>>(entity).unwrap();
                error!("AssetKey \"{}\" refers to an asset that does not exist!", asset_key.key);
            }
        } else if Some(&AssetState::Loaded)
            == world.get_resource::<State<AssetState>>().map(|s| &**s)
        {
            // If in the `AssetState::Loaded` state, warn about missing assets
            warn!("AssetKey \"{}\" does not refer to any known asset", asset_key.key);
        }
    }

    /// Removes the [`Handle`] from the entity, if it has one.
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        if world.get::<Handle<A>>(entity).is_some() {
            world.commands().entity(entity).remove::<Handle<A>>();
        }
    }
}

impl<A: Asset> From<ResourceKey> for AssetKey<A> {
    fn from(key: ResourceKey) -> Self { Self::new(key) }
}
impl<A: Asset> From<AssetKey<A>> for ResourceKey {
    fn from(key: AssetKey<A>) -> Self { key.key }
}

impl<A: Asset> AsRef<ResourceKey> for AssetKey<A> {
    fn as_ref(&self) -> &ResourceKey { &self.key }
}
