//! [`SystemSets`](bevy_ecs::schedule::SystemSet) for Froglight.

use bevy_app::App;

mod assets;
pub use assets::*;

mod interface;
pub use interface::*;

mod networking;
pub use networking::*;

mod physics;
pub use physics::*;

mod utility;
pub use utility::*;

mod world;
pub use world::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    assets::build(app);
    interface::build(app);
    networking::build(app);
    physics::build(app);
    utility::build(app);
    world::build(app);
}
