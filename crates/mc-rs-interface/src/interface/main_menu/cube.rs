use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use mc_rs_core::ResourceLocation;

use crate::{resourcepacks::ResourcePacks, traits::textures::GetAssetFromWorld};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(super) struct BackgroundCube;

impl BackgroundCube {
    /// Generate a cube texture from the panorama textures.
    pub fn create_cube_texture(world: &mut World) -> Handle<Image> {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building BackgroundCube Texture");

        // Get the cubemap textures
        let mut cube_textures = [
            "panorama_0",
            "panorama_1",
            "panorama_2",
            "panorama_3",
            "panorama_4",
            "panorama_5",
        ]
        .into_iter()
        .map(|name| {
            world
                .get_texture(&ResourceLocation::new(format!(
                    "minecraft:gui/title/background/{name}"
                )))
                .clone()
        })
        .collect::<Vec<_>>();

        // Get the image assets
        let fallback = world.resource::<ResourcePacks>().fallback.clone();
        let mut images = world.resource_mut::<Assets<Image>>();

        // Get the texture size
        let texture = images.get(cube_textures[0].id()).unwrap();
        let mut width = texture.width();
        let mut height = texture.height();

        // Check that all textures are square and the same size,
        // otherwise replace all textures with the fallback texture
        if width != height
            || cube_textures.iter().any(|texture| {
                let texture = images.get(texture.id()).unwrap();
                texture.width() != width || texture.height() != height
            })
        {
            #[cfg(any(debug_assertions, feature = "debug"))]
            error!("Panorama textures are not the same size, using fallback texture");

            let image = images.get(fallback.id()).unwrap();
            width = image.width();
            height = image.height();

            cube_textures = vec![
                fallback.clone(),
                fallback.clone(),
                fallback.clone(),
                fallback.clone(),
                fallback.clone(),
                fallback,
            ]
        }

        // Combine the cubemap textures lengthwise into a single texture
        let mut image_data: Vec<u8> = Vec::with_capacity((width * width * 6 * 4) as usize);
        for y in 0..height {
            for texture in cube_textures.iter() {
                let texture_data = images.get(texture.id()).unwrap().data.as_slice();

                image_data.extend_from_slice(
                    &texture_data[(y * width * 4) as usize..((y + 1) * width * 4) as usize],
                );
            }
        }

        // Add the cubemap texture as an asset
        images.add(Image::new(
            Extent3d {
                width: width * 6,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            image_data,
            TextureFormat::Rgba8UnormSrgb,
        ))
    }

    /// Generate a cube mesh with modified UV coordinates.
    pub fn create_cube_mesh(world: &mut World) -> Handle<Mesh> {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building BackgroundCube Mesh");

        let mut mesh = Mesh::from(shape::Cube { size: -1.0 });

        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                // Front
                [0., 0.],
                [1. / 6., 0.],
                [1. / 6., 1.],
                [0., 1.],
                // Back
                [3. / 6., 1.],
                [2. / 6., 1.],
                [2. / 6., 0.],
                [3. / 6., 0.],
                // Left
                [2. / 6., 0.],
                [2. / 6., 1.],
                [1. / 6., 1.],
                [1. / 6., 0.],
                // Right
                [4. / 6., 0.],
                [4. / 6., 1.],
                [3. / 6., 1.],
                [3. / 6., 0.],
                // Bottom
                [1., 1.],
                [5. / 6., 1.],
                [5. / 6., 0.],
                [1., 0.],
                // Top
                [5. / 6., 1.],
                [4. / 6., 1.],
                [4. / 6., 0.],
                [5. / 6., 0.],
            ],
        );

        // Add the mesh as an asset
        world.resource_mut::<Assets<Mesh>>().add(mesh)
    }
}
