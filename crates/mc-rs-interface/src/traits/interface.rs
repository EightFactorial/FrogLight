use bevy::prelude::{App, Entity, World};

/// A trait for interface components
pub trait InterfaceComponent: 'static {
    fn setup(app: &mut App);

    fn build(parent: Entity, world: &mut World);
}
