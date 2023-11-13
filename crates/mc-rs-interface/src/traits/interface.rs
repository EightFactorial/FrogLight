use bevy::prelude::{App, Entity, World};

/// A trait for interface components
pub trait InterfaceComponent: 'static {
    /// Setup the interface component systems
    fn setup(app: &mut App);

    /// Build the interface component
    fn build(parent: Entity, world: &mut World);
}
