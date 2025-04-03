use std::ops::{Deref, DerefMut};

use bevy_app::AppLabel;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use super::ReflectSubAppSync;

/// A trait for syncing a main [`App`](bevy_app::App)
/// with a [`SubApp`](bevy_app::SubApp).
pub trait SubAppSync<SubApp: AppLabel> {
    /// The label of the [`SubApp`](bevy_app::SubApp).
    const LABEL: SubApp;

    /// Sync the main [`App`](bevy_app::App)
    /// with the [`SubApp`](bevy_app::SubApp).
    fn sync(app: &mut World, sub: &mut World);
}

// -------------------------------------------------------------------------------------------------

/// A [`Resource`] containing all related [`SubAppSync`] functions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource, Reflect)]
#[reflect(Debug, Resource)]
pub struct SyncStorage<SubApp: AppLabel>(Vec<ReflectSubAppSync<SubApp>>);

impl<SubApp: AppLabel> FromWorld for SyncStorage<SubApp> {
    fn from_world(world: &mut World) -> Self {
        let registry = world.resource::<AppTypeRegistry>().read();

        Self(
            registry
                .iter_with_data::<ReflectSubAppSync<SubApp>>()
                .map(|(_type, reflect)| *reflect)
                .collect(),
        )
    }
}

impl<SubApp: AppLabel> Deref for SyncStorage<SubApp> {
    type Target = Vec<ReflectSubAppSync<SubApp>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}
impl<SubApp: AppLabel> DerefMut for SyncStorage<SubApp> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
