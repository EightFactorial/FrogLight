//! The art displayed on the loading screen
use bevy::{prelude::*, render::view::RenderLayers};

use crate::{layout::fade_animation::FadeAnimationMarker, plugin::LoadingPluginArtPath};

#[doc(hidden)]
pub(super) fn setup(_app: &mut App) {}

/// The art displayed on the loading screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub(crate) struct LoadingArt;

impl LoadingArt {
    /// Create the loading art
    pub(super) fn build_loading_icon(world: &mut World, parent: Entity) {
        // Get the path to the art asset
        let art_asset_path = world.resource::<LoadingPluginArtPath>().0.clone();

        // Load the icon asset
        let asset_server = world.resource::<AssetServer>();
        let art_asset_handle: Handle<Image> = asset_server.load(art_asset_path);

        world
            .spawn((
                FadeAnimationMarker,
                RenderLayers::layer(1),
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
                    LoadingArt,
                    FadeAnimationMarker,
                    RenderLayers::layer(1),
                    ImageBundle {
                        style: Style {
                            width: Val::VMin(50.0),
                            height: Val::Auto,
                            ..Default::default()
                        },
                        image: UiImage::new(art_asset_handle),
                        ..Default::default()
                    },
                ));
            });
    }
}