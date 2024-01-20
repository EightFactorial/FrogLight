/// The loading icon of the loading screen
use bevy::prelude::*;

use crate::plugin::LoadingPluginIconPath;

#[doc(hidden)]
pub(super) fn setup(_app: &mut App) {}

/// The loading icon of the loading screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct LoadingIcon;

impl LoadingIcon {
    /// Create the loading icon
    // TODO: Use an ImageBundle with an embedded asset
    pub(super) fn build_loading_icon(world: &mut World, parent: Entity) {
        // Get the path to the icon asset
        let icon_asset_path = world.resource::<LoadingPluginIconPath>().0.clone();

        // Load the icon asset
        let asset_server = world.resource::<AssetServer>();
        let icon_asset_handle: Handle<Image> = asset_server.load(icon_asset_path);

        world
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(75.0),

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
                    LoadingIcon,
                    ImageBundle {
                        style: Style {
                            width: Val::VMin(50.0),
                            height: Val::Auto,
                            ..Default::default()
                        },
                        image: UiImage::new(icon_asset_handle),
                        ..Default::default()
                    },
                ));
            });
    }
}
