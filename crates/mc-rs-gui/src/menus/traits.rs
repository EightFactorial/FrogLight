use std::fmt::Debug;

use bevy::prelude::*;

pub(super) trait MenuComponent: Debug + Component {
    fn add_systems(app: &mut App);
    fn build(parent: Entity, world: &mut World);
}

pub(super) trait VisibilityFromWorld {
    fn in_state<S: States + Eq>(&self, state: S) -> bool;
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
