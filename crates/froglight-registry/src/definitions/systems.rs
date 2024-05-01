use std::marker::PhantomData;

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    event::EventReader,
    schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
    system::{Res, ResMut},
};
use froglight_protocol::traits::Version;

use super::{ConvertKey, DefaultRegistry, InitializeRegistry, SimpleRegistry};
use crate::{systemsets::RegistryPostUpdateSet, RegistryUpdateEvent};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct RegistrySystems<V: Version>(PhantomData<V>);

#[allow(dead_code)]
impl<V: Version> RegistrySystems<V> {
    /// Registers the [`SystemSet`] in the given [`App`].
    ///
    /// This should be called once for each [`Version`].
    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn register_systemset(app: &mut App)
    where
        [(); { V::ID } as usize]:,
    {
        app.configure_sets(
            PostUpdate,
            Self::default().run_if(Self::systemset_conditions).in_set(RegistryPostUpdateSet),
        );
    }

    /// Returns `true` if the [`RegistryUpdateEvent`] has the
    /// same [`Version::ID`] as [`V`].
    #[allow(clippy::cast_sign_loss)]
    fn systemset_conditions(mut events: EventReader<RegistryUpdateEvent>) -> bool
    where
        [(); { V::ID } as usize]:,
    {
        events.read().any(RegistryUpdateEvent::is_equal::<{ V::ID }>)
    }

    /// Initializes a [`DefaultRegistry`] and adds a
    /// listener to update the [`SimpleRegistry`].
    ///
    /// This should be called once for each registry in each [`Version`].
    pub(crate) fn create_systems<R>(app: &mut App)
    where
        R: 'static + Clone + ConvertKey + InitializeRegistry<V>,
    {
        app.init_resource::<DefaultRegistry<V, R>>()
            .add_systems(PostUpdate, Self::update_registry::<R>.in_set(Self::default()));
    }

    /// Overwrites the [`SimpleRegistry`] with the
    /// content inside the [`DefaultRegistry`].
    fn update_registry<R>(def: Res<DefaultRegistry<V, R>>, mut reg: ResMut<SimpleRegistry<R>>)
    where
        R: 'static + Clone + ConvertKey + InitializeRegistry<V>,
    {
        reg.overwrite_with(&def);
    }
}
