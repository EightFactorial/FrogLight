//! Events that are received by the connection plugin.

use bevy_app::App;

mod packet;
pub use packet::RecvPacketEvent;

mod server_status;
pub use server_status::ServerStatusEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { server_status::build(app); }
