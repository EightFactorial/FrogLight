use std::ops::Mul;

use bevy::{
    prelude::*,
    render::{mesh::VertexAttributeValues, view::RenderLayers},
};
use froglight_assets::{AssetManager, FallbackImage};

mod camera;
pub use camera::MainMenuBackgroundCamera;

mod cube;
use cube::MainMenuBackgroundShader;
use froglight_core::resources::{LoadingScreenState, MainMenuEnable};

use super::systemset::MainMenuUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    camera::build(app);
    cube::build(app);

    app.register_type::<MainMenuBackground>();
    app.add_systems(
        Update,
        MainMenuBackground::background_rotation
            .run_if(LoadingScreenState::is_hidden)
            .run_if(any_with_component::<MainMenuBackground>)
            .in_set(MainMenuUpdateSet),
    );
    app.add_systems(
        Update,
        MainMenuBackground::background_visibility
            .run_if(resource_exists_and_changed::<MainMenuEnable>)
            .run_if(any_with_component::<MainMenuBackground>)
            .in_set(MainMenuUpdateSet),
    );
}

/// A marker [`Component`] for the main menu background.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuBackground;

impl MainMenuBackground {
    const ROTATION_SPEED: f32 = 2.0;
    const RENDER_LAYER: RenderLayers = RenderLayers::layer(4);

    fn background_rotation(mut query: Query<&mut Transform, With<Self>>, time: Res<Time<Virtual>>) {
        let delta = time.delta_seconds().mul(Self::ROTATION_SPEED).min(0.2).to_radians();
        for mut transform in &mut query {
            transform.rotate_y(delta);
        }
    }

    fn background_visibility(
        mut query: Query<&mut Visibility, With<Self>>,
        res: Res<MainMenuEnable>,
    ) {
        let new = if **res { Visibility::Inherited } else { Visibility::Hidden };
        for mut visibility in &mut query {
            *visibility = new;
        }
    }

    pub(crate) fn build(world: &mut World) {
        debug!("Building MainMenuBackground");

        // Load the textures for the main menu background
        // TODO: Actually select the correct textures
        let shader: MainMenuBackgroundShader;
        {
            let assets = world.resource::<AssetManager>();
            let fallback = world.resource::<FallbackImage>();
            let textures = assets.textures.read();

            let front = textures.get("minecraft:gui/title/background/panorama_0");
            let front =
                if let Some(texture) = front { texture.clone() } else { fallback.as_ref().clone() };

            let right = textures.get("minecraft:gui/title/background/panorama_1");
            let right =
                if let Some(texture) = right { texture.clone() } else { fallback.as_ref().clone() };

            let back = textures.get("minecraft:gui/title/background/panorama_2");
            let back =
                if let Some(texture) = back { texture.clone() } else { fallback.as_ref().clone() };

            let left = textures.get("minecraft:gui/title/background/panorama_3");
            let left =
                if let Some(texture) = left { texture.clone() } else { fallback.as_ref().clone() };

            let top = textures.get("minecraft:gui/title/background/panorama_4");
            let top =
                if let Some(texture) = top { texture.clone() } else { fallback.as_ref().clone() };

            let bottom = textures.get("minecraft:gui/title/background/panorama_5");
            let bottom = if let Some(texture) = bottom {
                texture.clone()
            } else {
                fallback.as_ref().clone()
            };

            // Create the shader struct
            shader = MainMenuBackgroundShader { front, right, back, left, top, bottom };
        }

        // Add the shader to the asset manager
        let material: Handle<MainMenuBackgroundShader> = {
            let mut assets = world.resource_mut::<Assets<MainMenuBackgroundShader>>();
            assets.add(shader)
        };

        // Get a cube mesh
        let mesh: Handle<Mesh> = {
            // Create the mesh
            let mut mesh = Mesh::from(Cuboid::from_size(Vec3::splat(-1.0)));

            // Insert custom flipped/rotated UVs
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                VertexAttributeValues::Float32x2(vec![
                    // Front
                    [1.0, 0.0],
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    // Back
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    // Left
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    // Right
                    [0.0, 0.0],
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    // Top
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    // Bottom
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                ]),
            );

            // Add the index attributes to select the correct texture
            let mut indexes = Vec::with_capacity(24);
            for i in 0u32..6u32 {
                indexes.extend(vec![i; 4]);
            }
            mesh.insert_attribute(cube::ATTRIBUTE_INDEX, indexes);

            // Add the mesh to the asset manager
            world.resource_mut::<Assets<Mesh>>().add(mesh)
        };

        // Determine the visibility of the background
        let visibility = if let Some(MainMenuEnable(true)) = world.get_resource::<MainMenuEnable>()
        {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        // Create the background entity
        let bundle = MaterialMeshBundle { mesh, material, visibility, ..Default::default() };
        world.spawn((
            Name::new("MainMenuBackground"),
            MainMenuBackground,
            Self::RENDER_LAYER,
            bundle,
        ));
    }
}
