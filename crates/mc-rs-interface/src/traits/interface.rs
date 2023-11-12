use bevy::prelude::{App, Entity, State, States, Visibility, World};

/// A trait for interface components
pub trait InterfaceComponent: 'static {
    /// Setup the interface component systems
    fn setup(app: &mut App);

    /// Build the interface component
    fn build(parent: Entity, world: &mut World);
}

/// A trait for interface components to determine their visibility
pub trait MenuVisibility {
    /// Returns true if the world is in the given state
    fn in_state<S: States + PartialEq>(&self, state: S) -> bool;

    /// Returns true if the world is in the given states
    fn in_states<S1: States + PartialEq, S2: States + PartialEq>(
        &self,
        state_1: S1,
        state_2: S2,
    ) -> bool;

    /// Returns the visibility of the menu based on the given states
    fn get_menu_visibility<S1: States + PartialEq, S2: States + PartialEq>(
        &self,
        state_1: S1,
        state_2: S2,
    ) -> Visibility
    where
        S1: PartialEq,
        S2: PartialEq;
}

impl MenuVisibility for World {
    fn in_state<S: States + PartialEq>(&self, state: S) -> bool {
        *self.resource::<State<S>>() == state
    }

    fn in_states<S1: States + PartialEq, S2: States + PartialEq>(
        &self,
        state_1: S1,
        state_2: S2,
    ) -> bool {
        self.in_state(state_1) && self.in_state(state_2)
    }

    fn get_menu_visibility<S1: States + PartialEq, S2: States + PartialEq>(
        &self,
        state_1: S1,
        state_2: S2,
    ) -> Visibility {
        match self.in_states(state_1, state_2) {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        }
    }
}
