use froglight_protocol::versions::{v1_20_0::V1_20_0, v1_20_2::V1_20_2, v1_20_3::V1_20_3};

pub(crate) mod misc;

mod states;

pub(super) mod handler;
use handler::ConnectionHandler;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    <V1_20_0 as ConnectionHandler>::build(app);
    <V1_20_2 as ConnectionHandler>::build(app);
    <V1_20_3 as ConnectionHandler>::build(app);
}
