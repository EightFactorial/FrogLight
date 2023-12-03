use bevy::prelude::*;

use super::resources::MenuResources;

/// A trait implemented for all menu components.
pub(super) trait MenuComponent: Sized + std::fmt::Debug + Default + Component {
    /// Setup the menu component.
    fn setup(app: &mut App);

    /// Build the menu component.
    fn build(parent: Entity, world: &mut World);

    fn show(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing {:?}", Self::default());

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Visible;
        });
    }

    fn hide(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding {:?}", Self::default());

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Hidden;
        });
    }
}

pub(super) trait AddMenuResource {
    fn add_menu_resource(&mut self, handle: UntypedHandle);
}

impl AddMenuResource for World {
    fn add_menu_resource(&mut self, handle: UntypedHandle) {
        let mut resources = self.resource_mut::<MenuResources>();
        resources.push(handle);
    }
}

/// A trait implemented on [`World`] that provides methods for checking the current state.
pub(super) trait InState {
    /// Returns true if the world is in the given state.
    fn in_state<S: States + Eq>(&self, state: S) -> bool;
    /// Returns either [Visibility::Visible] or [Visibility::Hidden]
    /// depending on whether the world is in the given state.
    fn get_visibility<S: States + Eq>(&self, state: S) -> Visibility;
}

impl InState for World {
    fn in_state<S: States + Eq>(&self, state: S) -> bool { *self.resource::<State<S>>() == state }
    fn get_visibility<S: States + Eq>(&self, state: S) -> Visibility {
        match self.in_state(state) {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        }
    }
}
