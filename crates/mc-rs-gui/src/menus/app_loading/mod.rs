use bevy::prelude::*;

use crate::menus::traits::MenuComponent;

use super::{resources::MenuResources, states::assets::AssetLoadingState};

pub mod background;
pub mod logo;
pub mod progress;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AppLoadingNodeComponent;

impl AppLoadingNodeComponent {
    pub(super) fn setup(app: &mut App) {
        app.add_systems(Startup, Self::build);
        app.add_systems(OnEnter(AssetLoadingState::Unloaded), Self::show);

        app.add_systems(
            Update,
            Self::hide.run_if(
                in_state(AssetLoadingState::Finished)
                    .and_then(Self::is_shown.and_then(MenuResources::loaded)),
            ),
        );

        background::BackgroundNodeComponent::setup(app);
        progress::ProgressNodeComponent::setup(app);
        logo::LogoNodeComponent::setup(app);
    }

    fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building AppLoadingNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: Visibility::Visible,
            z_index: ZIndex::Global(i32::MAX - 32),
            ..Default::default()
        };

        let entity = world.spawn((AppLoadingNodeComponent, node)).id();
        background::BackgroundNodeComponent::build(entity, world);
        progress::ProgressNodeComponent::build(entity, world);
        logo::LogoNodeComponent::build(entity, world);
    }

    fn is_shown(query: Query<&Visibility, With<Self>>) -> bool {
        query.iter().all(|vis| *vis == Visibility::Visible)
    }

    fn show(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing {Self:?}");

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Visible;
        });
    }

    fn hide(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding {Self:?}");

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Hidden;
        });
    }
}
