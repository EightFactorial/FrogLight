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
    // Embedd the portal shader.
    embedded_asset!(app, "portal_effect.wgsl");

    // Register the portal shader.
    app.register_type::<PortalEffect>()
        // Add the portal shader plugin.
        .add_plugins(ExtractComponentPlugin::<PortalEffect>::default())
        .add_plugins(UniformComponentPlugin::<PortalEffect>::default());
}

/// A shader that renders a screen effect when the player
/// is standing inside a portal.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Reflect, Component, ExtractComponent, ShaderType,
)]
#[reflect(Component)]
pub struct PortalEffect {
    /// Whether the portal effect is enabled.
    pub enabled: u32,

    /// The intensity of the portal effect.
    pub intensity: f32,

    /// The color of the portal effect.
    ///
    /// This color is multiplied with the portal effect.
    pub color: Color,
}

#[allow(dead_code)]
impl PortalEffect {
    /// Returns a [`BindGroupEntry`] for this shader.
    pub(crate) fn bind_entry(&self) -> BindGroupEntry { todo!() }

    /// Returns a [`BindGroupLayoutEntry`] for this shader.
    pub(crate) fn layout_entry(&self) -> BindGroupLayoutEntry { todo!() }
}
