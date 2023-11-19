use bevy::prelude::*;
use mc_rs_core::schedule::state::ApplicationState;

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
            GuiLoadingTexturesSet::next
                .in_set(GuiLoadingTexturesSet)
                .run_if(in_state(GuiLoadState::LoadingTextures).and_then(ResourcePacks::loaded)),
            // Exit CreatingAtlases state when TextureAtlases are loaded
            GuiCreatingAtlasesSet::next
                .after(TextureAtlases::build)
                .in_set(GuiCreatingAtlasesSet)
                .run_if(in_state(GuiLoadState::CreatingAtlases).and_then(TextureAtlases::loaded)),
            // Exit BuildingGui state when MenuRoot is built
            GuiBuildingSet::next.in_set(GuiBuildingSet).run_if(
                in_state(GuiLoadState::BuildingGui)
                    .and_then(MenuResources::loaded.and_then(any_with_component::<MenuRoot>())),
            ),
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
    fn next(
        mut gui_state: ResMut<NextState<GuiLoadState>>,
        mut app_state: ResMut<NextState<ApplicationState>>,
    ) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering GuiFinished");

        gui_state.set(GuiLoadState::Finished);

        if matches!(app_state.0, Some(ApplicationState::Loading)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("Entering MainMenu");

            app_state.set(ApplicationState::MainMenu);
        }
    }
}
