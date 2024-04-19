//! Events that are received by the connection plugin.

use bevy_app::App;

mod packet;
pub use packet::RecvPacketEvent;

pub(crate) mod server_conn;
pub use server_conn::ServerConnectionEvent;

pub(crate) mod server_status;
pub use server_status::ServerStatusEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    server_conn::build(app);
    server_status::build(app);
}
