use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat},
};
use mc_rs_core::schedule::state::ApplicationState;
use mc_rs_resourcepack::assets::resourcepacks::{ResourcePacks, TextureFromWorld};

use crate::{
    menus::{
        main_menu::MainMenuState,
        traits::{AddMenuResource, MenuComponent, VisibilityFromWorld},
    },
    resources::camera::DefaultCamera,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct CubemapBackground;

impl MenuComponent for CubemapBackground {
    fn setup(app: &mut App) {
        app.add_systems(
            OnEnter(MainMenuState::Main),
            (
                CubemapBackground::show,
                (
                    DefaultCamera::enable_camera3d,
                    CubemapBackground::background_camera_fov,
                ),
            ),
        );
        app.add_systems(
            OnExit(MainMenuState::Main),
            (CubemapBackground::hide, DefaultCamera::disable_camera3d),
        );

        app.add_systems(
            Update,
            CubemapBackground::rotate_cube.run_if(
                in_state(MainMenuState::Main).and_then(any_with_component::<CubemapBackground>()),
            ),
        );

        app.add_systems(
            OnExit(ApplicationState::MainMenu),
            CubemapBackground::destroy,
        );
    }

    fn build(_parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building CubemapBackground");

        // Create cube mesh
        let mut mesh = Mesh::from(shape::Cube::new(1.0));
        Self::correct_uvs(&mut mesh);
        let mesh = world.resource_mut::<Assets<Mesh>>().add(mesh);

        // Create material
        let material_texture = Self::create_image(world);
        let material = StandardMaterial {
            base_color_texture: Some(material_texture.clone()),
            cull_mode: Some(Face::Front),
            unlit: true,
            ..Default::default()
        };
        let material = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(material);

        // Add handles to MenuResources
        world.add_menu_resource(material_texture.untyped());
        world.add_menu_resource(material.clone().untyped());
        world.add_menu_resource(mesh.clone().untyped());

        // Spawn CubemapBackground
        world.spawn((
            CubemapBackground,
            PbrBundle {
                mesh,
                material,
                visibility: world.get_visibility(MainMenuState::Main),
                // Flip the cube so that the inside is right-side-up
                transform: Transform::from_rotation(Quat::from_rotation_x(180f32.to_radians())),
                ..Default::default()
            },
        ));
    }
}

impl CubemapBackground {
    /// Set the background [camera](Camera)'s [FOV](Projection) to 90 degrees.
    fn background_camera_fov(mut query: Query<&mut Projection, With<Camera3d>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Setting background camera FOV");

        query.for_each_mut(|mut projection| {
            if let Projection::Perspective(ref mut perspective) = *projection {
                perspective.fov = 90f32.to_radians();
            }
        });
    }

    /// Rotate the [`CubemapBackground`].
    fn rotate_cube(
        time: Res<Time<Real>>,
        mut query: Query<&mut Transform, With<CubemapBackground>>,
    ) {
        let rotation = Quat::from_rotation_y(time.delta_seconds() / 40.);
        query.iter_mut().for_each(|mut transform| {
            transform.rotate(rotation);
        });
    }

    /// Despawn the [`CubemapBackground`] entity.
    fn destroy(query: Query<Entity, With<CubemapBackground>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Despawning CubemapBackground");

        query.for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        })
    }

    /// Create the background image from the panorama images.
    fn create_image(world: &mut World) -> Handle<Image> {
        // Get the panorama images
        let images = [
            world.get_texture("minecraft:gui/title/background/panorama_0"),
            world.get_texture("minecraft:gui/title/background/panorama_1"),
            world.get_texture("minecraft:gui/title/background/panorama_2"),
            world.get_texture("minecraft:gui/title/background/panorama_3"),
            world.get_texture("minecraft:gui/title/background/panorama_4"),
            world.get_texture("minecraft:gui/title/background/panorama_5"),
        ];

        // If any of the images are missing or have different sizes, create a fallback
        if images.iter().any(|handle| handle.is_none())
            || images.windows(2).any(|images| {
                // Get the image handles
                let (Some(a), Some(b)) = (images[0], images[1]) else {
                    // Missing image, use fallback
                    return true;
                };

                // Get the images
                if let (Some(a), Some(b)) = (
                    world.resource::<Assets<Image>>().get(a),
                    world.resource::<Assets<Image>>().get(b),
                ) {
                    // Compare the sizes, return fallback if they are different
                    a.size() != b.size()
                } else {
                    // Missing image, use fallback
                    true
                }
            })
        {
            return world.resource::<ResourcePacks>().fallback.clone();
        }

        // Unwrap all of the handles and get the images
        let images = images
            .into_iter()
            .map(Option::unwrap)
            .map(|handle| world.resource::<Assets<Image>>().get(handle).unwrap())
            .collect::<Vec<_>>();

        // Get the width and height of the final image
        // Images are stacked horizontally, so the width is the sum of the widths
        let (width, height) = images.iter().fold((0, 0), |(width, _), &image| {
            (width + image.width(), image.height())
        });

        // Create the image data buffer
        let mut image_data: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        // Loop through the images, copying each image's rows into the final image data buffer
        for y in 0..height {
            for &image in images.iter() {
                // Copy the row
                image_data.extend_from_slice(
                    &image.data
                        [(y * image.width() * 4) as usize..((y + 1) * image.width() * 4) as usize],
                );
            }
        }

        // Create the image
        let image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            image_data,
            TextureFormat::Rgba8UnormSrgb,
        );

        // Add the image as an asset
        world.resource_mut::<Assets<Image>>().add(image)
    }

    /// Correct the UVs of the [`cube`](shape::Cube) [`mesh`](Mesh).
    fn correct_uvs(mesh: &mut Mesh) {
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
    }
}
