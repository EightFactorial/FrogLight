use bevy::prelude::*;
use mc_rs_core::schedule::{set::LoadingSet, state::ApplicationState};

use crate::resourcepacks::ResourcePacksFinishReloadEvent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct LoadingInterface;

pub(super) fn setup(app: &mut App) {
    app.add_systems(OnEnter(ApplicationState::Loading), LoadingInterface::show);
    app.add_systems(OnExit(ApplicationState::Loading), LoadingInterface::hide);

    app.add_systems(
        Update,
        (LoadingInterface::transition)
            .run_if(LoadingInterface::finish_loading)
            .in_set(LoadingSet),
    );
}

impl LoadingInterface {
    pub(super) fn spawn(
        root: Entity,
        state: &State<ApplicationState>,
        _assets: &AssetServer,
        commands: &mut Commands,
    ) {
        let loading = NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            background_color: Color::BLUE.into(),
            visibility: match **state {
                ApplicationState::Loading => Visibility::Visible,
                _ => Visibility::Hidden,
            },
            ..Default::default()
        };

        commands.entity(root).with_children(|root| {
            root.spawn((loading, Self)).with_children(|load| {
                load.spawn(TextBundle::from_section("Loading...", TextStyle::default()));
            });
        });
    }

    /// Check if the resourcepacks have finished reloading.
    fn finish_loading(
        state: Res<State<ApplicationState>>,
        events: EventReader<ResourcePacksFinishReloadEvent>,
    ) -> bool {
        matches!(**state, ApplicationState::Loading) && !events.is_empty()
    }

    /// Transition to the main menu.
    fn transition(mut next_state: ResMut<NextState<ApplicationState>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Entering ApplicationState::MainMenu state!");

        next_state.set(ApplicationState::MainMenu);
    }

    fn show(mut query: Query<&mut Visibility, With<Self>>) {
        query.for_each_mut(|mut vis| *vis = Visibility::Visible);
    }

    fn hide(mut query: Query<&mut Visibility, With<Self>>) {
        query.for_each_mut(|mut vis| *vis = Visibility::Hidden);
    }
}
