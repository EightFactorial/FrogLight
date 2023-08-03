use bevy::prelude::*;

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
            MenuRoot::create_camera.run_if(not(any_with_component::<Camera2d>())),
            // Create the menu root node
            MenuRoot::create.run_if(not(any_with_component::<MenuRoot>())),
        )
            .in_set(MenuSet),
    );

    #[cfg(feature = "splash")]
    {
        app.add_systems(
            OnEnter(ApplicationState::InMenu),
            MenuRoot::delete_splash
                .run_if(any_with_component::<crate::plugins::splash::SplashRoot>()),
        );
    }

    app.add_systems(
        Update,
        MenuRoot::show.run_if(MenuRoot::is_hidden).in_set(InMenuSet),
    );
    app.add_systems(
        OnExit(ApplicationState::InMenu),
        MenuRoot::hide.in_set(InMenuSet),
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
    fn create_camera(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

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

    /// Is the menu currently visible
    fn is_hidden(vis: Query<&Visibility, With<MenuRoot>>) -> bool {
        vis.single() == Visibility::Hidden
    }

    /// Make the menu visible
    fn show(mut vis: Query<&mut Visibility, With<MenuRoot>>) {
        *vis.single_mut() = Visibility::Visible;
    }

    /// Make the menu visible
    fn hide(mut vis: Query<&mut Visibility, With<MenuRoot>>) {
        *vis.single_mut() = Visibility::Hidden;
    }

    #[cfg(feature = "splash")]
    /// Delete the splash screen
    fn delete_splash(
        mut commands: Commands,
        mut elements: belly::prelude::Elements,
        query: Query<Entity, With<crate::plugins::splash::SplashRoot>>,
    ) {
        // Remove the elements
        for entity in elements.select(".splash").entities() {
            commands.entity(entity).despawn_recursive();
        }

        // Remove the splash screen root
        commands.entity(query.single()).despawn_recursive();
    }
}
