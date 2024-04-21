//! [`Events`](bevy_ecs::prelude::Event) sent and received by the connection
//! plugin.

use bevy_app::App;

mod connection;
pub use connection::*;

mod packets;
pub use packets::*;

mod status;
pub use status::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Register connection events
    app.add_event::<ConnectionRequest>().add_event::<ConnectionDisconnect>();

    // Register status events
    app.add_event::<StatusRequest>().add_event::<StatusResponse>();
}
