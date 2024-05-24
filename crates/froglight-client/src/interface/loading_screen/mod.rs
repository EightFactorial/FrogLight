//! A basic [`LoadingScreen`]

use bevy::{prelude::*, ui::FocusPolicy};

mod enable;
pub use enable::*;

mod logo;
pub use logo::*;

mod progress;
pub use progress::*;

mod systemset;
pub use systemset::LoadingScreenSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.register_type::<LoadingScreen>();

    // Spawn a new LoadingScreen on startup if one doesn't exist
    app.add_systems(
        PostStartup,
        LoadingScreen::spawn_loading_screen.run_if(not(any_with_component::<LoadingScreen>)),
    );

    systemset::build(app);
    enable::build(app);
    logo::build(app);
    progress::build(app);
}

/// A marker [`Component`] for a loading screen.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component, Default)]
pub struct LoadingScreen;

impl LoadingScreen {
    /// The default [`ZIndex`] for the [`LoadingScreen`].
    ///
    /// Ensures the [`LoadingScreen`] is always on top.
    pub const Z_INDEX: ZIndex = ZIndex::Global(i32::MAX / 1024);

    /// A [`System`](bevy::prelude::System) that spawns a
    /// [`LoadingScreen`].
    ///
    /// This is a helper function to give the system a more descriptive name.
    #[inline]
    fn spawn_loading_screen(world: &mut World) { Self::spawn(world); }

    /// Spawns a new [`LoadingScreen`] in the [`World`].
    pub fn spawn(world: &mut World) {
        // Get the visibility of the loading screen
        let visibility = world.resource::<LoadingScreenVisibility>().get_visibility();

        // Spawn a new loading screen
        let entity = world.spawn_empty().id();
        Self::spawn_at(entity, visibility, world);

        // Set a global `ZIndex` to ensure the loading screen is always on top
        world.entity_mut(entity).insert(Self::Z_INDEX);
    }

    /// Creates a new [`LoadingScreen`] at the given [`Entity`].
    pub fn spawn_at(entity: Entity, visibility: Visibility, world: &mut World) {
        debug!("Entity {entity:?} - Spawning a new `LoadingScreen`");
        let Some(mut entity_commands) = world.get_entity_mut(entity) else {
            error!("Failed to spawn `LoadingScreen`, Entity not found!");
            return;
        };

        // Create a new NodeBundle
        //
        // Fills the screen, blocks input below it,
        // and centers its children in a column.
        let node = NodeBundle {
            style: Style {
                position_type: PositionType::Relative,

                display: Display::Flex,
                flex_direction: FlexDirection::Column,

                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,

                left: Val::Percent(0.0),
                top: Val::Percent(0.0),

                height: Val::Percent(100.0),
                width: Val::Percent(100.0),

                ..Default::default()
            },
            background_color: Color::BLACK.into(),
            focus_policy: FocusPolicy::Block,
            visibility,
            ..Default::default()
        };

        // Insert the marker and `NodeBundle`
        entity_commands.insert((LoadingScreen, Name::new("LoadingScreen"), node));

        // Spawn a logo and progress bar and add them as children
        let children: [Entity; 2] = entity_commands.world_scope(|world| {
            [LoadingScreenLogo::spawn(world), LoadingScreenProgressBar::spawn(world)]
        });
        entity_commands.push_children(&children);
    }
}
