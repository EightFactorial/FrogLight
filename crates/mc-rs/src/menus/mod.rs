use bevy::prelude::*;

use crate::systems::states::application::{ApplicationState, MenuSet};

pub mod credits_menu;
pub mod inventory_menus;
pub mod main_menu;
pub mod pause_menu;
pub mod server_menu;
pub mod settings_menu;

/// Add menu systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_systems(
        OnEnter(ApplicationState::default()),
        (
            // Create the camera
            MenuRoot::create_camera.run_if(not(any_with_component::<Camera2d>())),
            // Create the menu root node
            MenuRoot::create.run_if(not(any_with_component::<MenuRoot>())),
        )
            .chain()
            .in_set(MenuSet),
    );

    // TODO: Add menu systems
    main_menu::setup_menu(app);
    inventory_menus::setup_menus(app);
}

/// The menu root node
///
/// All menus should be children of this node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MenuRoot;

impl MenuRoot {
    /// Create a new camera bundle
    fn create_camera(mut commands: Commands) { commands.spawn(Self::camera_2d_bundle()); }

    /// Get the default Camera2dBundle
    ///
    /// Right now this is just the default Camera2dBundle,
    /// but it's here in case we need to change it later
    #[inline]
    pub fn camera_2d_bundle() -> Camera2dBundle { Camera2dBundle::default() }

    /// Create a new menu root node
    fn create(mut commands: Commands) {
        commands.spawn((
            MenuRoot,
            NodeBundle {
                style: Style {
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
        ));
    }
}
