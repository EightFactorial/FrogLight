use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_resource::{BindGroupEntry, BindGroupLayoutEntry, ShaderType},
    },
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Embed the blur shader.
    embedded_asset!(app, "world_blur.wgsl");

    // Register the blur shader.
    app.register_type::<WorldBlurEffect>()
        // Add the blur shader plugin.
        .add_plugins(ExtractComponentPlugin::<WorldBlurEffect>::default())
        .add_plugins(UniformComponentPlugin::<WorldBlurEffect>::default());
}

/// A shader that renders a blur effect on the world.
///
/// The primary use for this shader is to
/// blur the world when the game is paused.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Reflect, Component, ExtractComponent, ShaderType,
)]
#[reflect(Component)]
pub struct WorldBlurEffect {
    /// Whether the blur effect is enabled.
    pub enabled: u32,

    /// The strength of the blur effect.
    pub strength: f32,
}

#[allow(dead_code)]
impl WorldBlurEffect {
    /// Returns a [`BindGroupEntry`] for this shader.
    pub(crate) fn bind_entry(&self) -> BindGroupEntry { todo!() }

    /// Returns a [`BindGroupLayoutEntry`] for this shader.
    pub(crate) fn layout_entry(self) -> BindGroupLayoutEntry { todo!() }
}
