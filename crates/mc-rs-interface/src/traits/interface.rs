use bevy::prelude::{App, Entity, World};

pub trait SubInterface {
    fn setup(app: &mut App);

    fn build(root: Entity, world: &mut World);
}
