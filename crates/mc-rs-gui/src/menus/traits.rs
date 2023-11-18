use std::fmt::Debug;

use bevy::prelude::*;

pub(super) trait MenuComponent: Debug + Component {
    /// Setup the [`MenuComponent`] and all of its children's systems.
    fn setup(app: &mut App);
    /// Build the [`MenuComponent`] and all of its children.
    fn build(parent: Entity, world: &mut World);
}

pub(super) trait VisibilityFromWorld {
    /// Returns true if the world is in the given state.
    fn in_state<S: States + Eq>(&self, state: S) -> bool;
    /// Returns true if the world is not in the given state.
    fn not_in_state<S: States + Eq>(&self, state: S) -> bool { !self.in_state(state) }
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
