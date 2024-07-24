use bevy_app::{App, Update};
use bevy_asset::Handle;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    prelude::resource_changed,
    reflect::ReflectResource,
    schedule::{Condition, IntoSystemConfigs},
    system::{Res, Resource},
};
use bevy_log::warn;
use bevy_reflect::{prelude::ReflectDefault, Reflect};
use bevy_state::state::{NextState, State};

use super::{AssetLoadState, AssetLoadSystemSet};
use crate::ResourcePack;

#[doc(hidden)]
pub(crate) fn build(app: &mut App) {
    app.register_type::<ResourcePackList>().init_resource::<ResourcePackList>();

    // Warn if the `ResourcePackList` is modified after assets are processed.
    app.add_systems(
        Update,
        ResourcePackList::list_modified_warning
            .run_if(resource_changed::<ResourcePackList>.and_then(ResourcePackList::should_warn))
            .in_set(AssetLoadSystemSet),
    );
}

/// A list of [`ResourcePack`]s to use when processing assets.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Reflect, Deref, DerefMut, Resource)]
#[reflect(Default, Resource)]
pub struct ResourcePackList {
    packs: Vec<Handle<ResourcePack>>,
}

impl ResourcePackList {
    /// Creates a empty [`ResourcePackList`].
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// Returns `true` if the current state is not
    /// [`AssetLoadState::Waiting`] or [`AssetLoadState::Loading`]
    /// and if the next state is not
    /// [`AssetLoadState::Waiting`] or [`AssetLoadState::Loading`].
    fn should_warn(
        current: Res<State<AssetLoadState>>,
        next: Res<NextState<AssetLoadState>>,
    ) -> bool {
        !matches!(
            (current.as_ref().get(), next.as_ref()),
            (AssetLoadState::Waiting | AssetLoadState::Loading, _)
                | (_, NextState::Pending(AssetLoadState::Waiting | AssetLoadState::Loading))
        )
    }

    /// Log a warning message that the [`ResourcePackList`] was modified.
    fn list_modified_warning() {
        warn!("ResourcePackList: Modified after assets are processed, this may cause issues!");
    }
}
