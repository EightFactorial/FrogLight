use bevy::prelude::*;

use crate::menus::{
    state::GuiLoadState,
    traits::{MenuComponent, VisibilityFromWorld},
};

use super::{
    state::{GuiBuildingSet, GuiLoadingResourcePacksSet},
    MenuRoot,
};

mod bar;
mod logo;

mod menu;
use menu::LoadingMenu;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct LoadingMenuRoot;

impl LoadingMenuRoot {
    /// Setup [LoadingMenuRoot] and it's components.
    pub(crate) fn setup(app: &mut App) {
        app.add_systems(Startup, LoadingMenuRoot::build.before(MenuRoot::build));

        app.add_systems(
            OnEnter(GuiLoadState::LoadingResourcePacks),
            LoadingMenuRoot::show.in_set(GuiLoadingResourcePacksSet),
        );
        app.add_systems(
            OnExit(GuiLoadState::BuildingGui),
            LoadingMenuRoot::hide.in_set(GuiBuildingSet),
        );

        LoadingMenu::setup(app);
    }

    /// Build the [LoadingMenuRoot] and its components.
    fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building LoadingMenuRoot");

        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            visibility: match !world.in_state(GuiLoadState::Finished) {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            },
            z_index: ZIndex::Global(i32::MAX - 32),
            ..Default::default()
        };

        // Spawn LoadingMenuRoot
        let entity = world.spawn((LoadingMenuRoot, node)).id();
        LoadingMenu::build(entity, world);
    }

    /// Show the [`LoadingMenuRoot`].
    fn show(mut query: Query<&mut Visibility, With<LoadingMenuRoot>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing LoadingMenuRoot");

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Visible;
        })
    }

    /// Hide the [`LoadingMenuRoot`].
    fn hide(mut query: Query<&mut Visibility, With<LoadingMenuRoot>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding LoadingMenuRoot");

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Hidden;
        })
    }
}
