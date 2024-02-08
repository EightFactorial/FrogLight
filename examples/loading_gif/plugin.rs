use std::time::Duration;

use bevy::prelude::*;
use froglight_client::loading::{
    systemsets::LoadingScreenToggleSet, FadeAnimationMarker, LoadingScreenCenter,
};

/// A custom loading plugin that displays a GIF instead of a static image.
///
/// This requires [`froglight_client`]'s `default-loading` feature to be enabled
/// **and** the [`LoadingPlugin`](froglight_client::loading::LoadingPlugin) to
/// be set to [`LoadingPlugin::None`](froglight_client::loading::LoadingPlugin::None).
///
///
/// # Note
/// This plugin is designed to be used with GIFs that have been split into
/// individual frames. Use an online tool to split the GIF into one massive
/// image, and then use the `embedded_asset!` macro to embed the image into the
/// application.
#[derive(Debug, Clone, PartialEq)]
pub struct GifLoadingPlugin {
    /// The path to the GIF's tiled frames
    pub path: String,
    /// The duration of each frame
    /// 
    /// For example, 30 FPS would be
    /// ```rust
    /// Duration::from_secs_f32(1.0 / 30.0)
    /// ```
    pub duration: Duration,
    /// The dimensions of each frame
    /// 
    /// In the provided example, each frame is 360x241 pixels
    pub frame_dimensions: Vec2,
    /// The tiling of the frames
    /// 
    /// In the provided example, the GIF is 2x94 frames
    pub frame_tiling: UVec2,
    /// The total number of frames
    /// 
    /// In the provided example, the GIF has 187 frames
    /// 
    /// This is required to prevent any issues with blank frames
    /// if the GIF doesn't fill the entire atlas
    pub frame_count: usize,
}

impl Plugin for GifLoadingPlugin {
    fn build(&self, app: &mut App) {
        // Create the loading art
        app.add_systems(
            Update,
            GifLoadingPlugin::build_loading_art
                .run_if(GifAsset::is_loaded)
                .run_if(not(any_with_component::<GifLoadingArt>()))
                .in_set(LoadingScreenToggleSet),
        );

        // Advance the art's frame
        app.add_systems(
            Update,
            GifLoadingArt::advance_frame
                .run_if(any_with_component::<GifLoadingArt>())
                .after(GifLoadingPlugin::build_loading_art)
                .in_set(LoadingScreenToggleSet),
        );
    }

    fn finish(&self, app: &mut App) {
        // Load the GIF asset into a resource
        let image_handle = app.world.resource::<AssetServer>().load(&self.path);
        app.world.insert_resource(GifAsset {
            timer: Timer::new(self.duration, TimerMode::Repeating),
            handle: image_handle,
            frame_dimensions: self.frame_dimensions,
            frame_tiling: self.frame_tiling,
            frame_count: self.frame_count,
        });
    }
}

impl GifLoadingPlugin {
    /// Create the loading art UI element
    fn build_loading_art(
        query: Query<Entity, With<LoadingScreenCenter>>,
        asset: Res<GifAsset>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
        mut commands: Commands,
    ) {
        // Create the texture atlas
        let atlas = TextureAtlas::from_grid(
            asset.handle.clone(),
            asset.frame_dimensions,
            asset.frame_tiling.x as usize,
            asset.frame_tiling.y as usize,
            None,
            None,
        );
        let atlas_handle = atlases.add(atlas);

        // Create the GIF loading art inside the loading screen
        match query.get_single() {
            Ok(parent) => {
                // Create a node bundle to center the art
                let child = commands
                    .spawn((
                        FadeAnimationMarker,
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
                    ))
                    .with_children(|node| {
                        // Create the GIF loading art
                        node.spawn((
                            FadeAnimationMarker,
                            GifLoadingArt,
                            AtlasImageBundle {
                                style: Style {
                                    width: Val::VMin(50.0),
                                    height: Val::Auto,
                                    ..Default::default()
                                },
                                texture_atlas: atlas_handle,
                                texture_atlas_image: UiTextureAtlasImage {
                                    index: 1,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ));
                    })
                    .id();

                // Forcefully insert the child at the front of the parent's children
                // Fixes the art being below the progress bar
                commands.entity(parent).insert_children(0, &[child]);
            }
            Err(err) => {
                error!("Failed to create GifLoadingArt: {err}");
            }
        }
    }
}

/// A marker component for the GIF loading art
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct GifLoadingArt;

impl GifLoadingArt {
    /// Advance the currently shown frame
    fn advance_frame(
        mut query: Query<&mut UiTextureAtlasImage, With<GifLoadingArt>>,
        mut asset: ResMut<GifAsset>,
        time: Res<Time<Real>>,
    ) {
        // Every time the timer ticks
        if asset.timer.tick(time.delta()).just_finished() {
            // Advance the frame, looping back to the start once the end is reached
            for mut image in query.iter_mut() {
                image.index = (image.index + 1) % asset.frame_count;
            }
        }
    }
}

/// A resource an asset handle, timer, and frame information
#[derive(Debug, Clone, PartialEq, Resource)]
struct GifAsset {
    timer: Timer,
    handle: Handle<Image>,
    frame_dimensions: Vec2,
    frame_tiling: UVec2,
    frame_count: usize,
}

impl GifAsset {
    /// Check if the asset is loaded
    fn is_loaded(asset: Res<GifAsset>, assets: Res<AssetServer>) -> bool {
        assets.is_loaded_with_dependencies(&asset.handle)
    }
}
