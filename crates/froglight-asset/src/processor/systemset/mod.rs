use bevy_app::App;

mod asset_process;
pub use asset_process::{AssetProcess, AssetProcessSet};

mod asset_state;
pub use asset_state::{AssetState, AssetStateSet};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    asset_process::build(app);
    asset_state::build(app);
}
