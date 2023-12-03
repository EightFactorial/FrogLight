use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat},
};
use mc_rs_core::schedule::state::ApplicationState;
use mc_rs_resourcepack::assets::resourcepacks::{AssetFromWorld, ResourcePacks};

use crate::{
    menus::{
        app_menus::states::MainMenuState,
        states::menus::MenuComponentMenusSet,
        traits::{AddMenuResource, InState},
    },
    resources::camera::DefaultCamera,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct BackgroundCubeComponent;

impl BackgroundCubeComponent {
    pub(super) fn setup(app: &mut App) {
        app.add_systems(
            OnEnter(MainMenuState::MainMenu),
            (
                Self::show,
                (DefaultCamera::enable_camera3d, Self::background_camera),
            )
                .in_set(MenuComponentMenusSet),
        );

        app.add_systems(
            Update,
            Self::rotate_cube
                .in_set(MenuComponentMenusSet)
                .run_if(in_state(MainMenuState::MainMenu).and_then(any_with_component::<Self>())),
        );

        app.add_systems(
            OnExit(MainMenuState::MainMenu),
            (Self::hide, DefaultCamera::disable_camera3d).in_set(MenuComponentMenusSet),
        );
        app.add_systems(
            OnExit(ApplicationState::MainMenu),
            Self::destroy.in_set(MenuComponentMenusSet),
        );
    }

    pub(super) fn build(world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Building BackgroundCubeComponent");

        // Create cube mesh
        let mesh = Self::correct_uvs(Mesh::from(shape::Cube::new(1.0)));
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

        // Spawn BackgroundCubeComponent
        world.spawn((
            BackgroundCubeComponent,
            PbrBundle {
                mesh,
                material,
                visibility: world.get_visibility(MainMenuState::MainMenu),
                // Flip the cube so that the inside is right-side-up
                transform: Transform::from_rotation(Quat::from_rotation_x(180f32.to_radians())),
                ..Default::default()
            },
        ));
    }

    /// Rotate the [`BackgroundCubeComponent`].
    fn rotate_cube(
        time: Res<Time<Real>>,
        mut query: Query<&mut Transform, With<BackgroundCubeComponent>>,
    ) {
        let rads = (time.delta_seconds() / 30.0).clamp(0.0, 0.01);
        let rotation = Quat::from_rotation_y(rads);

        query.iter_mut().for_each(|mut transform| {
            transform.rotate(rotation);
        });
    }

    /// Despawn the [`BackgroundCubeComponent`] entity.
    fn destroy(query: Query<Entity, With<BackgroundCubeComponent>>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Despawning BackgroundCubeComponent");

        query.for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        })
    }

    /// Set the background [camera](Camera)'s [`Transform`] and [`FOV`](Projection).
    fn background_camera(mut query: Query<(&mut Transform, &mut Projection), With<Camera3d>>) {
        query.for_each_mut(|(mut transform, mut projection)| {
            // Reset camera's transform
            *transform = Transform::default();

            // Set camera's FOV
            if let Projection::Perspective(ref mut perspective) = *projection {
                if perspective.fov != 80f32.to_radians() {
                    #[cfg(any(debug_assertions, feature = "debug"))]
                    debug!("Setting Camera3d FOV");

                    perspective.fov = 80f32.to_radians();
                }
            }
        });
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
                    // Return fallback if the images are missing
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
                    // Return fallback if the images are missing
                    true
                }
            })
        {
            #[cfg(any(debug_assertions, feature = "debug"))]
            warn!("Using fallback texture for BackgroundCubeComponent");

            return world.resource::<ResourcePacks>().fallback.clone();
        }

        // Unwrap all of the handles and get the images
        let images = images
            .into_iter()
            .map(Option::unwrap)
            .map(|handle| world.resource::<Assets<Image>>().get(handle).unwrap())
            .collect::<Vec<_>>();

        // Get the width and height of the final image
        let (width, mut height) = images[0].size().into();
        height *= 6;

        // Create the image
        let image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            // Combine the images into one, vertically
            images.iter().fold(
                Vec::with_capacity((width * height * 4) as usize),
                |mut image_data, image| {
                    // Add the image data to the final image data
                    image_data.extend_from_slice(&image.data);

                    // Return the final image data
                    image_data
                },
            ),
            TextureFormat::Rgba8UnormSrgb,
        );

        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Created BackgroundCubeComponent Texture");

        // Add the image as an asset
        world.resource_mut::<Assets<Image>>().add(image)
    }

    /// Correct the UVs of a [`Cube`](shape::Cube) [`Mesh`].
    fn correct_uvs(mut mesh: Mesh) -> Mesh {
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                // Front
                [0., 0.],
                [1., 0.],
                [1., 1. / 6.],
                [0., 1. / 6.],
                // Back
                [1., 3. / 6.],
                [0., 3. / 6.],
                [0., 2. / 6.],
                [1., 2. / 6.],
                // Right
                [1., 1. / 6.],
                [1., 2. / 6.],
                [0., 2. / 6.],
                [0., 1. / 6.],
                // Left
                [1., 3. / 6.],
                [1., 4. / 6.],
                [0., 4. / 6.],
                [0., 3. / 6.],
                // Bottom
                [1., 1.],
                [0., 1.],
                [0., 5. / 6.],
                [1., 5. / 6.],
                // Top
                [1., 5. / 6.],
                [0., 5. / 6.],
                [0., 4. / 6.],
                [1., 4. / 6.],
            ],
        );

        mesh
    }

    fn show(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Showing BackgroundCubeComponent");

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Visible;
        });
    }

    fn hide(mut query: Query<&mut Visibility, With<Self>>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Hiding BackgroundCubeComponent");

        query.iter_mut().for_each(|mut vis| {
            *vis = Visibility::Hidden;
        });
    }
}
