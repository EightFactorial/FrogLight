use bevy::{
    asset::{embedded_asset, ReflectAsset, ReflectHandle},
    prelude::*,
    render::{
        extract_component::ExtractComponentPlugin,
        render_phase::AddRenderCommand,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedRenderPipelines,
        },
        RenderApp,
    },
    ui::{
        extract_ui_material_nodes, extract_ui_materials, DrawUiMaterial, ExtractedUiMaterialNodes,
        ExtractedUiMaterials, RenderUiMaterials, RenderUiSystem, TransparentUi, UiMaterialMeta,
    },
};

mod pipeline;
use pipeline::GaussianNodePipeline;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    embedded_asset!(app, "gaussian_node.wgsl");

    app.register_type::<GaussianNode>()
        .init_asset::<GaussianNode>()
        .register_type_data::<Handle<GaussianNode>, ReflectHandle>()
        .add_plugins(ExtractComponentPlugin::<Handle<GaussianNode>>::extract_visible());

    if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app
            .add_render_command::<TransparentUi, DrawUiMaterial<GaussianNode>>()
            .init_resource::<ExtractedUiMaterials<GaussianNode>>()
            .init_resource::<ExtractedUiMaterialNodes<GaussianNode>>()
            .init_resource::<RenderUiMaterials<GaussianNode>>()
            .init_resource::<UiMaterialMeta<GaussianNode>>()
            .init_resource::<SpecializedRenderPipelines<GaussianNodePipeline>>()
            .add_systems(
                ExtractSchedule,
                (
                    extract_ui_materials::<GaussianNode>,
                    extract_ui_material_nodes::<GaussianNode>.in_set(RenderUiSystem::ExtractNode),
                ),
            );
        // TODO: Manual implementations
        // .add_systems(
        //     Render,
        //     (
        //         prepare_ui_materials::<GaussianNode>.
        // in_set(RenderSet::PrepareAssets),
        //         queue_ui_material_nodes::<GaussianNode>.
        // in_set(RenderSet::Queue),
        //         prepare_uimaterial_nodes::<GaussianNode>.
        // in_set(RenderSet::PrepareBindGroups),     ),
        // );
    }
}

#[doc(hidden)]
pub(super) fn finish(app: &mut App) {
    if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.init_resource::<GaussianNodePipeline>();
    }
}

/// A [`UiMaterial`] that applies a gaussian blur effect under the node.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect, Asset, AsBindGroup)]
#[reflect(Asset)]
pub struct GaussianNode {
    /// Whether the blur effect is enabled.
    pub enabled: u32,

    /// The strength of the blur effect.
    pub strength: f32,
}

impl GaussianNode {
    /// The path to the shader file.
    pub(crate) const SHADER_PATH: &'static str =
        "embedded://froglight_interface/materials/gaussian_node/gaussian_node.wgsl";

    /// Creates a new [`GaussianNode`] with the blur effect enabled and a
    /// strength of `1.0`.
    #[must_use]
    pub const fn new(enabled: bool, strength: f32) -> Self {
        Self { enabled: enabled as u32, strength }
    }
}

impl UiMaterial for GaussianNode {
    fn fragment_shader() -> ShaderRef { Self::SHADER_PATH.into() }

    fn specialize(_descriptor: &mut RenderPipelineDescriptor, _key: UiMaterialKey<Self>) {}
}
