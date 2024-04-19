use froglight_protocol::versions::v1_20_0::V1_20_0;

pub(crate) mod misc;

mod states;

pub(super) mod traits;
use traits::handler::ConnectionHandler;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { <V1_20_0 as ConnectionHandler>::build(app); }
