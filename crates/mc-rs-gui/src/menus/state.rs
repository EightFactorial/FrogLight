use bevy::prelude::*;

use crate::{
    assets::{resourcepacks::ResourcePacks, textureatlases::TextureAtlases},
    menus::{MenuResources, MenuRoot},
};

pub(super) fn setup(app: &mut App) {
    app.add_state::<GuiLoadState>();

    app.configure_sets(
        Update,
        (
            GuiLoadingTexturesSet.run_if(in_state(GuiLoadState::LoadingTextures)),
            GuiCreatingAtlasesSet.run_if(in_state(GuiLoadState::CreatingAtlases)),
            GuiBuildingSet.run_if(in_state(GuiLoadState::BuildingGui)),
        )
            .chain(),
    );

    app.add_systems(
        Update,
        (
            // Exit LoadingTextures state when all ResourcePacks and MenuResources are loaded
            (GuiLoadingTexturesSet::next
                .run_if(ResourcePacks::loaded.and_then(MenuResources::loaded)))
            .in_set(GuiLoadingTexturesSet),
            // Exit CreatingAtlases state when TextureAtlases are loaded
            (GuiCreatingAtlasesSet::next
                .after(TextureAtlases::build)
                .run_if(TextureAtlases::loaded))
            .in_set(GuiCreatingAtlasesSet),
            // Exit BuildingGui state when MenuRoot is built
            (GuiBuildingSet::next.run_if(any_with_component::<MenuRoot>())).in_set(GuiBuildingSet),
        ),
    );

    #[cfg(any(debug_assertions, feature = "debug"))]
    app.add_systems(
        OnEnter(GuiLoadState::LoadingTextures),
        // Log when entering the LoadingTextures state
        || debug!("Entering GuiLoadingTextures"),
    );

    // When entering the CreatingAtlases state
    app.add_systems(
        OnEnter(GuiLoadState::CreatingAtlases),
        // Build the TextureAtlases
        TextureAtlases::build,
    );
    // When entering the BuildingGui state
    app.add_systems(
        OnEnter(GuiLoadState::BuildingGui),
        // Build the MenuRoot
        MenuRoot::build,
    );
}

/// The state of the gui loading process
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GuiLoadState {
    #[default]
    LoadingTextures,
    CreatingAtlases,
    BuildingGui,
    Finished,
}

/// A set of systems that occurs when in the [GuiLoadState::LoadingTextures] state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GuiLoadingTexturesSet;

impl GuiLoadingTexturesSet {
    /// Advance to the [GuiLoadState::CreatingAtlases] state
    fn next(mut state: ResMut<NextState<GuiLoadState>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering GuiCreatingAtlases");

        state.set(GuiLoadState::CreatingAtlases)
    }
}

/// A set of systems that occurs when in the [GuiLoadState::CreatingAtlases] state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GuiCreatingAtlasesSet;

impl GuiCreatingAtlasesSet {
    /// Advance to the [GuiLoadState::BuildingGui] state
    fn next(mut state: ResMut<NextState<GuiLoadState>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering GuiBuilding");

        state.set(GuiLoadState::BuildingGui)
    }
}

/// A set of systems that occurs when in the [GuiLoadState::BuildingGui] state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct GuiBuildingSet;

impl GuiBuildingSet {
    /// Advance to the [GuiLoadState::Finished] state
    fn next(mut state: ResMut<NextState<GuiLoadState>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering GuiFinished");

        state.set(GuiLoadState::Finished)
    }
}
