use bevy::prelude::*;

use crate::schedule::{set::GameSet, state::ApplicationState};

use self::{
    resources::WorldType,
    structure::{chunk::Chunk, section::SectionComponent},
    worlds::Worlds,
};

pub mod palette;
pub mod resources;
pub mod structure;
pub mod worlds;

pub const CHUNK_HEIGHT: usize = 384;
pub const CHUNK_VERT_DISPLACEMENT: isize = -64;
pub const CHUNK_SIZE: usize = 16;
pub const SECTION_HEIGHT: usize = 16;
pub const SECTION_COUNT: usize = CHUNK_HEIGHT / SECTION_HEIGHT;

/// Adds the `Worlds` resource and its systems.
pub(super) fn setup(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::InGame),
        Worlds::create.in_set(GameSet),
    );

    app.add_systems(
        Update,
        SectionComponent::despawn_orphans
            .run_if(any_component_removed::<Chunk>())
            .in_set(GameSet),
    );

    app.add_systems(
        OnExit(ApplicationState::InGame),
        Worlds::destroy.in_set(GameSet),
    );
}
