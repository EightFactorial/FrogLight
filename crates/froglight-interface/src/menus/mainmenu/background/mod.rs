use std::ops::Mul;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use froglight_assets::{AssetManager, FallbackImage, ResourcePackState};

mod camera;
pub use camera::MainMenuBackgroundCamera;

mod shader;
use shader::MainMenuBackgroundShader;

use super::systemset::MainMenuUpdateSet;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    camera::build(app);
    shader::build(app);

    app.add_systems(
        Update,
        MainMenuBackground::rotate_background
            .run_if(any_with_component::<MainMenuBackground>)
            .run_if(in_state(ResourcePackState::Ready))
            .in_set(MainMenuUpdateSet),
    );
}

/// A marker [`Component`] for the main menu background.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuBackground;

impl MainMenuBackground {
    const ROTATION_SPEED: f32 = 6.0;

    fn rotate_background(mut query: Query<&mut Transform, With<Self>>, time: Res<Time<Virtual>>) {
        let delta = time.delta_seconds().mul(Self::ROTATION_SPEED).min(0.2).to_radians();
        for mut transform in &mut query {
            transform.rotate_y(delta);
        }
    }

    pub(crate) fn build(world: &mut World) {
        debug!("Building MainMenuBackground");

        // Load the textures for the main menu background
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

            let bottom = textures.get("minecraft:gui/title/background/panorama_4");
            let bottom = if let Some(texture) = bottom {
                texture.clone()
            } else {
                fallback.as_ref().clone()
            };

            let top = textures.get("minecraft:gui/title/background/panorama_5");
            let top =
                if let Some(texture) = top { texture.clone() } else { fallback.as_ref().clone() };

            // Create the shader struct
            shader = MainMenuBackgroundShader { front, back, left, right, top, bottom };
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
                    // Bottom
                    [0.0, 1.0],
                    [1.0, 1.0],
                    [1.0, 0.0],
                    [0.0, 0.0],
                    // Top
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
            mesh.insert_attribute(shader::ATTRIBUTE_INDEX, indexes);

            // Add the mesh to the asset manager
            world.resource_mut::<Assets<Mesh>>().add(mesh)
        };

        // Create the background entity
        let bundle = MaterialMeshBundle { mesh, material, ..Default::default() };
        world.spawn((MainMenuBackground, Name::new("MainMenuBackground"), bundle));
    }
}
