use std::fmt::Debug;

use bevy::prelude::*;

use super::MenuResources;

pub(super) trait MenuComponent: Debug + Sized + Default + Component {
    /// Setup the [`MenuComponent`] and all of its children's systems.
    fn setup(app: &mut App);
    /// Build the [`MenuComponent`] and all of its children.
    fn build(parent: Entity, world: &mut World);

    /// Show this component.
    fn show(mut query: Query<&mut Visibility, With<Self>>) {
        query.iter_mut().for_each(|mut vis| {
            if !matches!(*vis, Visibility::Visible) {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Showing {:?}", Self::default());

                *vis = Visibility::Visible;
            }
        })
    }

    /// Hide this component.
    fn hide(mut query: Query<&mut Visibility, With<Self>>) {
        query.iter_mut().for_each(|mut vis| {
            if !matches!(*vis, Visibility::Hidden) {
                #[cfg(any(debug_assertions, feature = "debug"))]
                debug!("Hiding {:?}", Self::default());

                *vis = Visibility::Hidden;
            }
        })
    }
}

pub(super) trait VisibilityFromWorld {
    /// Returns true if the world is in the given state.
    fn in_state<S: States + Eq>(&self, state: S) -> bool;
    /// Returns either [Visibility::Visible] or [Visibility::Hidden]
    /// depending on whether the world is in the given state.
    fn get_visibility<S: States + Eq>(&self, state: S) -> Visibility;
}

impl VisibilityFromWorld for World {
    fn in_state<S: States + Eq>(&self, state: S) -> bool { *self.resource::<State<S>>() == state }
    fn get_visibility<S: States + Eq>(&self, state: S) -> Visibility {
        match self.in_state(state) {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        }
    }
}

pub(super) trait AddMenuResource {
    /// Add a resource to the [`MenuResources`] resource list.
    ///
    /// This is used to ensure that all resources are loaded before the menu is shown.
    fn add_menu_resource(&mut self, handle: UntypedHandle);
}

impl AddMenuResource for World {
    fn add_menu_resource(&mut self, handle: UntypedHandle) {
        self.resource_mut::<MenuResources>().push(handle)
    }
}
