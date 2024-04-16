//! [`SystemSets`](bevy_ecs::schedule::SystemSet) for Froglight.

use bevy_app::App;

mod assets;
pub use assets::*;

mod entities;
pub use entities::*;

mod interface;
pub use interface::*;

mod network;
pub use network::*;

mod physics;
pub use physics::*;

mod registry;
pub use registry::*;

mod settings;
pub use settings::*;

mod utility;
pub use utility::*;

mod world;
pub use world::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    //  --- Ordered ---
    network::build(app);
    registry::build(app);

    // --- Ambiguous ---
    entities::build(app);
    utility::build(app);

    // --- Ordered ---
    world::build(app);
    physics::build(app);

    // --- Ordered ---
    settings::build(app);
    assets::build(app);
    interface::build(app);
}
