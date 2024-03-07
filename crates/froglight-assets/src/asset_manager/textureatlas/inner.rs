use bevy_app::Update;
use bevy_ecs::{
    schedule::{common_conditions::on_event, IntoSystemConfigs, NextState},
    system::ResMut,
};
use bevy_log::debug;
use bevy_reflect::Reflect;

use crate::{asset_manager::tracker::ResourcePackQueueFinished, ResourcePackState};

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    app.register_type::<AtlasManagerInner>();

    app.add_systems(
        Update,
        AtlasManagerInner::skip_building_atlases
            .run_if(on_event::<ResourcePackQueueFinished>())
            .in_set(ResourcePackState::Loading),
    );
}

#[derive(Debug, Default, Reflect)]
pub struct AtlasManagerInner {}

impl AtlasManagerInner {
    fn skip_building_atlases(mut state: ResMut<NextState<ResourcePackState>>) {
        debug!("Skipping building TextureAtlases");
        debug!("Entering ResourcePackState::Processing");
        state.set(ResourcePackState::Processing);
    }
}
