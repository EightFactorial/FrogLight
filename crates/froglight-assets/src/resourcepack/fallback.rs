use bevy::{asset::embedded_asset, prelude::*};
use froglight_core::systemsets::AssetStartupSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "fallback.png");

    app.add_systems(
        Startup,
        FallbackImage::create_fallback
            .run_if(not(resource_exists::<FallbackImage>))
            .in_set(AssetStartupSet),
    );
}

/// An [`Image`] that can be used as a fallback when another image or texture is
/// not available.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct FallbackImage(Handle<Image>);

impl FallbackImage {
    /// Inserts a new [`FallbackImage`] using the embedded asset `fallback.png`.
    fn create_fallback(asset_server: Res<AssetServer>, mut commands: Commands) {
        let handle = asset_server.load("embedded://froglight_assets/resourcepack/fallback.png");
        commands.insert_resource(Self(handle));
    }

    /// Creates a new [`FallbackImage`] with the given [`Handle<Image>`].
    #[must_use]
    pub fn new(handle: Handle<Image>) -> Self { Self(handle) }
}

impl AsRef<Handle<Image>> for FallbackImage {
    fn as_ref(&self) -> &Handle<Image> { &self.0 }
}
