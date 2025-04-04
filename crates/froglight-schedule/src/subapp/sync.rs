use bevy_app::AppLabel;
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use derive_more::{Deref, DerefMut};

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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource, Reflect, Deref, DerefMut)]
#[reflect(Debug, Resource)]
pub struct SyncStorage<SubApp: AppLabel>(Vec<ReflectSubAppSync<SubApp>>);

impl<SubApp: AppLabel> FromWorld for SyncStorage<SubApp> {
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
