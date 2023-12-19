use bevy::prelude::*;
use mc_rs_resourcepack::assets::{resourcepacks::ResourcePacks, textureatlases::TextureAtlases};

pub(super) fn setup(app: &mut App) {
    app.add_state::<AssetLoadingState>();

    app.configure_sets(
        PreUpdate,
        (
            AssetUnloadedSet.run_if(in_state(AssetLoadingState::Unloaded)),
            AssetCreatingTextureAtlasesSet
                .run_if(in_state(AssetLoadingState::CreatingTextureAtlases)),
        )
            .chain()
            .run_if(not(in_state(AssetLoadingState::Finished))),
    );

    #[cfg(any(debug_assertions, feature = "debug"))]
    app.add_systems(
        OnEnter(AssetLoadingState::Unloaded),
        AssetUnloadedSet::enter.in_set(AssetUnloadedSet),
    );

    app.add_systems(
        OnEnter(AssetLoadingState::CreatingTextureAtlases),
        TextureAtlases::build.in_set(AssetCreatingTextureAtlasesSet),
    );

    app.add_systems(
        PreUpdate,
        (
            AssetUnloadedSet::next
                .run_if(in_state(AssetLoadingState::Unloaded).and_then(ResourcePacks::loaded))
                .in_set(AssetUnloadedSet),
            AssetCreatingTextureAtlasesSet::next
                .after(TextureAtlases::build)
                .run_if(
                    in_state(AssetLoadingState::CreatingTextureAtlases)
                        .and_then(TextureAtlases::loaded),
                )
                .in_set(AssetCreatingTextureAtlasesSet),
        )
            .chain()
            .run_if(not(in_state(AssetLoadingState::Finished))),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum AssetLoadingState {
    #[default]
    Unloaded,
    CreatingTextureAtlases,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetUnloadedSet;

impl AssetUnloadedSet {
    #[cfg(any(debug_assertions, feature = "debug"))]
    fn enter() {
        debug!("Entering AssetUnloaded");
    }

    fn next(mut state: ResMut<NextState<AssetLoadingState>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering AssetCreatingTextureAtlases");

        state.set(AssetLoadingState::CreatingTextureAtlases);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct AssetCreatingTextureAtlasesSet;

impl AssetCreatingTextureAtlasesSet {
    fn next(mut state: ResMut<NextState<AssetLoadingState>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering AssetFinished");

        state.set(AssetLoadingState::Finished);
    }
}
