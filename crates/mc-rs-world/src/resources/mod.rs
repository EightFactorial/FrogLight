use bevy::app::App;

mod worlds;
pub use worlds::{World, Worlds};

mod world_type;
pub use world_type::WorldType;

mod current_world;
pub use current_world::CurrentWorld;

pub(super) fn setup(app: &mut App) { current_world::setup(app); }
