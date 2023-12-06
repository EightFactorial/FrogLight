use bevy::prelude::*;

use crate::menus::{
    states::menus::MenuComponentState,
    traits::{InState, MenuComponent},
};

pub mod hud;
pub mod inventory;
pub mod pause;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct AppIngameNodeComponent;

impl MenuComponent for AppIngameNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(OnEnter(MenuComponentState::InGame), Self::show);
        app.add_systems(OnExit(MenuComponentState::InGame), Self::hide);

        inventory::InventoryNodeComponent::setup(app);
        hud::HudNodeComponent::setup(app);
        pause::PauseNodeComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building AppIngameNodeComponent");
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: world.get_visibility(MenuComponentState::InGame),
            ..Default::default()
        };
        let entity = world
            .spawn((AppIngameNodeComponent, node))
            .set_parent(parent)
            .id();
        inventory::InventoryNodeComponent::build(entity, world);
        hud::HudNodeComponent::build(entity, world);
        pause::PauseNodeComponent::build(entity, world);
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
