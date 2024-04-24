//! [`SystemSets`](bevy_ecs::schedule::SystemSet) for Froglight.

use bevy_app::App;

mod assets;
pub use assets::*;

mod client;
pub use client::*;

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

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    assets::build(app);
    client::build(app);
    interface::build(app);
    network::build(app);
    physics::build(app);
    registry::build(app);
    settings::build(app);
    utility::build(app);
}
