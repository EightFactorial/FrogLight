use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::systems::states::application::{ApplicationState, InMenuSet, MenuSet};

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
            MenuRoot::create_camera
                .run_if(not(any_with_component::<Camera2d>()))
                .in_set(MenuSet),
            // Create the menu root node
            MenuRoot::create
                .run_if(not(any_with_component::<MenuRoot>()))
                .in_set(MenuSet),
        ),
    );

    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        MenuRoot::show
            .run_if(any_with_component::<MenuRoot>())
            .in_set(InMenuSet),
    );
    app.add_systems(
        OnExit(ApplicationState::InMenu),
        MenuRoot::hide
            .run_if(any_with_component::<MenuRoot>())
            .in_set(InMenuSet),
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
    #[inline]
    pub fn camera_2d_bundle() -> Camera2dBundle {
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..Default::default()
        }
    }

    /// Create a new menu root node
    fn create(state: Res<State<ApplicationState>>, mut commands: Commands) {
        let visibility = if **state == ApplicationState::InMenu {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };

        commands.spawn((
            MenuRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                visibility,
                ..Default::default()
            },
        ));
    }

    /// Make the menu visible
    pub fn show(mut vis: Query<&mut Visibility, With<MenuRoot>>) {
        *vis.single_mut() = Visibility::Visible;
    }

    /// Make the menu visible
    pub fn hide(mut vis: Query<&mut Visibility, With<MenuRoot>>) {
        *vis.single_mut() = Visibility::Hidden;
    }
}
