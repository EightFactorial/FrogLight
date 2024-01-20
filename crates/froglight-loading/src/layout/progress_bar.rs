//! The progress bar of the loading screen
use bevy::prelude::*;

#[doc(hidden)]
pub(super) fn setup(_app: &mut App) {}

/// The progress bar of the loading screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct ProgressBar;

impl ProgressBar {
    /// Create the progress bar
    pub(super) fn build_loading_bar(world: &mut World, parent: Entity) {
        world
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(25.0),

                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                #[cfg(debug_assertions)]
                Outline::new(Val::Px(1.0), Val::Auto, Color::BLUE),
            ))
            .set_parent(parent)
            .with_children(|node| {
                node.spawn((
                    ProgressBar,
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(90.0),
                            height: Val::Vh(2.0),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default()
                    },
                ));
            });
    }
}
