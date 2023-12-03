use bevy::{
    asset::embedded_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    window::{PrimaryWindow, WindowResized},
};

use crate::{
    menus::states::menus::{MenuComponentMenusSet, MenuComponentState},
    resources::scale::GuiScale,
};

pub(super) fn setup(app: &mut App) {
    embedded_asset!(app, "embedded/block_background.wgsl");
    app.add_plugins(UiMaterialPlugin::<BlockBackgroundMaterial>::default());

    app.add_systems(
        Update,
        BlockBackgroundMaterial::scale_background
            .in_set(MenuComponentMenusSet)
            .run_if(in_state(MenuComponentState::Menus).and_then(on_event::<WindowResized>())),
    );
}

#[derive(Debug, Clone, Asset, TypePath, AsBindGroup)]
pub struct BlockBackgroundMaterial {
    #[uniform(0)]
    pub scale_x: f32,
    #[uniform(1)]
    pub scale_y: f32,
    #[texture(2)]
    #[sampler(3)]
    pub texture: Handle<Image>,
}

impl Default for BlockBackgroundMaterial {
    fn default() -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            texture: Default::default(),
        }
    }
}

impl BlockBackgroundMaterial {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            scale_x: 1.0,
            scale_y: 1.0,
            texture,
        }
    }

    pub fn with_scale(texture: Handle<Image>, scale_x: f32, scale_y: f32) -> Self {
        Self {
            scale_x,
            scale_y,
            texture,
        }
    }

    /// Scales the background to fit the window, accounting for the GUI scale.
    fn scale_background(
        query: Query<&Window, With<PrimaryWindow>>,
        gui_scale: Res<GuiScale>,

        mut materials: ResMut<Assets<BlockBackgroundMaterial>>,
    ) {
        let Ok(window) = query.get_single() else {
            error!("Failed to get PrimaryWindow");
            return;
        };

        // Calculate the scaling factor.
        let scaler = Self::get_scale(gui_scale.value());
        let scale_width = window.width() / scaler;
        let scale_height = window.height() / scaler;

        // Update the materials.
        for (_, mat) in materials.iter_mut() {
            mat.scale_x = scale_width;
            mat.scale_y = scale_height;
        }
    }

    /// Calculates the scale of the background based on the GUI scale.
    pub fn get_scale(gui_scale: u32) -> f32 { 32.0 + (gui_scale as f32 - 1.0) * 16.0 }
}

impl UiMaterial for BlockBackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "embedded://mc_rs_gui/menus/shaders/embedded/block_background.wgsl".into()
    }
}
