use std::{any::TypeId, marker::PhantomData};

use bevy_asset::{Asset, Handle};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    component::{Component, ComponentHooks, ComponentId, StorageType},
    entity::Entity,
    reflect::ReflectComponent,
    world::DeferredWorld,
};
use bevy_log::warn;
use bevy_reflect::Reflect;
use froglight_common::ResourceKey;

use super::AssetCatalog;

/// A key to an asset in an [`AssetCatalog`].
///
/// Automatically inserts the correct [`Handle`] when added to an entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct AssetKey<A: Asset> {
    #[deref]
    asset_key: ResourceKey,
    #[reflect(ignore)]
    _marker: PhantomData<A>,
}

impl<A: Asset> AssetKey<A> {
    /// Creates a new [`AssetKey`] from a [`ResourceKey`].
    #[must_use]
    pub const fn new(asset_key: ResourceKey) -> Self { Self { asset_key, _marker: PhantomData } }

    /// Get a reference to the inner [`ResourceKey`].
    #[must_use]
    pub const fn as_key(&self) -> &ResourceKey { &self.asset_key }

    /// Returns the [`TypeId`] of the asset.
    #[must_use]
    pub const fn asset_id() -> TypeId { TypeId::of::<A>() }
}

impl<A: Asset> Component for AssetKey<A> {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(Self::on_add).on_remove(Self::on_remove);
    }
}

impl<A: Asset> AssetKey<A> {
    /// The `on_add` [`ComponentHooks`] function.
    fn on_add(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        let asset = world.get::<Self>(entity).expect("Hook called without Component?");
        // If the asset exists in the catalog
        if let Some(handle) = world.resource::<AssetCatalog>().get::<A>(asset) {
            // Queue a command to insert the handle
            world.commands().entity(entity).insert(handle);
        } else {
            // TODO: Do not warn if state hasn't loaded all assets yet
            warn!(
                "AssetKey<{}>: Requested unknown asset: \"{}\"",
                A::short_type_path(),
                asset.as_key()
            );
        }
    }

    /// The `on_remove` [`ComponentHooks`] function.
    fn on_remove(mut world: DeferredWorld, entity: Entity, _: ComponentId) {
        // If the entity has a handle
        if world.get::<Handle<A>>(entity).is_some() {
            // Queue a command to remove the handle
            world.commands().entity(entity).remove::<Handle<A>>();
        }
    }
}

impl<A: Asset> From<ResourceKey> for AssetKey<A> {
    fn from(key: ResourceKey) -> Self { Self::new(key) }
}
impl<A: Asset> From<AssetKey<A>> for ResourceKey {
    fn from(key: AssetKey<A>) -> Self { key.asset_key }
}

impl<A: Asset> AsRef<ResourceKey> for AssetKey<A> {
    fn as_ref(&self) -> &ResourceKey { &self.asset_key }
}
