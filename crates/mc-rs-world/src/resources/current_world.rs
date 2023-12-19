use bevy::prelude::*;
use derive_more::{From, Into};
use mc_rs_core::{schedule::state::ApplicationState, ResourceLocation};

use super::WorldType;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        OnExit(ApplicationState::InGame),
        CurrentWorld::on_leave.run_if(resource_exists::<CurrentWorld>()),
    );
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, Resource)]
pub struct CurrentWorld(WorldType);

impl CurrentWorld {
    fn on_leave(mut commands: Commands) { commands.remove_resource::<Self>(); }
}

impl From<ResourceLocation> for CurrentWorld {
    fn from(value: ResourceLocation) -> Self { Self(WorldType::from(value)) }
}
