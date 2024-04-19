//! Events that are sent by the connection plugin.

use bevy_app::App;

mod packet;
pub use packet::SendPacketEvent;

mod server_conn;
pub use server_conn::RequestConnectionEvent;

mod server_status;
pub use server_status::RequestStatusEvent;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    server_conn::build(app);
    server_status::build(app);
}
