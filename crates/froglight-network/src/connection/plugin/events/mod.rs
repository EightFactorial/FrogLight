//! Events that are sent and received by the connection plugin.

pub(super) mod recv;
pub use recv::*;

pub(super) mod send;
pub use send::*;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    recv::build(app);
    send::build(app);
}
