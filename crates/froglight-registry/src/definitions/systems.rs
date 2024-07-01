use std::marker::PhantomData;

use bevy_app::{App, PostUpdate};
use bevy_ecs::{
    event::EventReader,
    schedule::{
        common_conditions::on_event, Condition, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet,
    },
    system::{Commands, Res},
};
use froglight_protocol::traits::Version;

use super::{ConvertKey, DefaultRegistry, InitializeRegistry};
use crate::{systemsets::RegistryPostUpdateSet, RegistryUpdateEvent};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub(crate) struct RegistrySystems<V: Version>(PhantomData<V>);

#[allow(clippy::cast_sign_loss, dead_code)]
impl<V: Version> RegistrySystems<V> {
    /// Registers the [`SystemSet`] in the given [`App`].
    ///
    /// This should be called once for each [`Version`].
    pub(crate) fn register(app: &mut App)
    where
        [(); { V::ID } as usize]:,
    {
        app.configure_sets(
            PostUpdate,
            Self::default()
                .run_if(on_event::<RegistryUpdateEvent>().and_then(Self::condition))
                .in_set(RegistryPostUpdateSet),
        );
    }

    /// Returns `true` if the [`RegistryUpdateEvent`] has the
    /// same [`Version::ID`] as [`V`].
    fn condition(mut events: EventReader<RegistryUpdateEvent>) -> bool
    where
        [(); { V::ID } as usize]:,
    {
        events.read().any(RegistryUpdateEvent::is_equal::<{ V::ID }>)
    }

    /// Initializes a [`DefaultRegistry`] and adds a
    /// listener system to update the [`SimpleRegistry`](super::SimpleRegistry).
    ///
    /// This should be called once for each registry in each [`Version`].
    pub(crate) fn init<R>(app: &mut App)
    where
        R: 'static + Clone + ConvertKey + InitializeRegistry<V>,
    {
        app.init_resource::<DefaultRegistry<V, R>>()
            .add_systems(PostUpdate, Self::refresh_registry::<R>.in_set(Self::default()));
    }

    /// Overwrite the [`SimpleRegistry`] with the [`DefaultRegistry`].
    fn refresh_registry<R>(def: Res<DefaultRegistry<V, R>>, mut commands: Commands)
    where
        R: 'static + Clone + ConvertKey + InitializeRegistry<V>,
    {
        commands.insert_resource(def.create_simple());
    }
}
