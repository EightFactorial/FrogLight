use froglight_protocol::versions::v1_20_0::V1_20_0;

mod misc;
mod states;

mod traits;
use traits::handler::HandleConnection;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) { <V1_20_0 as HandleConnection>::build(app); }
