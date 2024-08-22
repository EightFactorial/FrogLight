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

use super::{AssetProcess, AssetProcessSet};
use crate::assets::ResourcePack;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<ResourcePackList>();
    app.init_resource::<ResourcePackList>();

    app.add_systems(
        Update,
        ResourcePackList::resource_warning
            .run_if(resource_changed::<ResourcePackList>.and_then(ResourcePackList::should_warn))
            .in_set(AssetProcessSet),
    );
}

/// A list of [`ResourcePack`]s.
///
/// [`ResourcePack`]s are loaded in the order they are listed.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct ResourcePackList(pub Vec<Handle<ResourcePack>>);

impl ResourcePackList {
    /// Create a new empty [`ResourcePackList`].
    #[must_use]
    pub const fn new() -> Self { Self(Vec::new()) }

    /// Return `true` if current and next states are not when assets are
    /// processed
    fn should_warn(current: Res<State<AssetProcess>>, next: Res<NextState<AssetProcess>>) -> bool {
        !matches!(
            (current.as_ref().get(), next.as_ref()),
            (AssetProcess::Waiting | AssetProcess::Loading, _)
                | (_, NextState::Pending(AssetProcess::Waiting | AssetProcess::Loading))
        )
    }

    /// Issue a warning that the [`ResourcePackList`] has been modified
    fn resource_warning() {
        warn!("ResourcePackList: Modified after assets are processed, this may cause issues!");
    }
}
