use std::marker::PhantomData;

use bevy_app::AppLabel;
use bevy_ecs::prelude::*;
use bevy_reflect::{FromType, prelude::*};
use derive_more::{Deref, DerefMut};

/// A [`Resource`] containing all related [`SubAppSync`] functions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource, Reflect, Deref, DerefMut)]
#[reflect(Debug, Resource)]
pub struct AppSyncStorage<SubApp: AppLabel>(Vec<ReflectSubAppSync<SubApp>>);

impl<SubApp: AppLabel> FromWorld for AppSyncStorage<SubApp> {
    fn from_world(world: &mut World) -> Self {
        Self(
            world
                .resource::<AppTypeRegistry>()
                .read()
                .iter_with_data::<ReflectSubAppSync<SubApp>>()
                .map(|(_, reflect)| *reflect)
                .collect(),
        )
    }
}

// -------------------------------------------------------------------------------------------------

/// A [`TypeData`](bevy_reflect::TypeData)-compatible
/// container for a [`SubAppSync`] implementation.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ReflectSubAppSync<SubApp: AppLabel> {
    sync_fn: fn(&mut World, &mut World),
    _phantom: PhantomData<SubApp>,
}

impl<T: SubAppSync<SubApp>, SubApp: AppLabel> FromType<T> for ReflectSubAppSync<SubApp> {
    fn from_type() -> Self { Self { sync_fn: T::sync, _phantom: PhantomData } }
}

impl<SubApp: AppLabel> ReflectSubAppSync<SubApp> {
    /// Sync the main [`App`](bevy_app::App)
    /// with the [`SubApp`](bevy_app::SubApp).
    #[inline]
    pub fn sync(&self, app: &mut World, sub: &mut World) { (self.sync_fn)(app, sub); }
}

// Manual implementations to avoid trait bounds.

impl<SubApp: AppLabel> Clone for ReflectSubAppSync<SubApp> {
    #[allow(clippy::non_canonical_clone_impl)]
    fn clone(&self) -> Self { Self { sync_fn: self.sync_fn, _phantom: PhantomData } }
}
impl<SubApp: AppLabel> Copy for ReflectSubAppSync<SubApp> {}

// -------------------------------------------------------------------------------------------------

/// A trait for syncing a main [`App`](bevy_app::App)
/// with a [`SubApp`](bevy_app::SubApp).
pub trait SubAppSync<SubApp: AppLabel> {
    /// The label of the [`SubApp`](bevy_app::SubApp).
    const LABEL: SubApp;

    /// Sync the main [`App`](bevy_app::App)
    /// with the [`SubApp`](bevy_app::SubApp).
    fn sync(app: &mut World, sub: &mut World);
}
