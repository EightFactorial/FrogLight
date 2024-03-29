//! Systems and components for the background panorama.

use std::ops::Mul;

use bevy::{
    prelude::*,
    render::{mesh::VertexAttributeValues, view::RenderLayers},
};
use froglight_assets::{AssetManager, FallbackImage, ResourcePackState};
use froglight_core::resources::LoadingScreenState;

pub(crate) mod camera;
pub use camera::MainMenuPanoramaCamera;

pub(crate) mod cube;
use cube::MainMenuBackgroundShader;

pub(crate) mod plugin;

use super::mainmenu::systemset::MainMenuUpdateSet;
use crate::menus::{
    loadingscreen::systemset::LoadingScreenUpdateSet,
    multiplayermenu::systemset::MultiplayerMenuUpdateSet,
    settingsmenu::systemset::SettingsMenuUpdateSet, InterfaceMenuState, InterfaceMenuUpdateSet,
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    camera::build(app);
    cube::build(app);

    app.init_resource::<MainMenuBackgroundEnable>().register_type::<MainMenuBackgroundEnable>();
    app.register_type::<MainMenuBackground>();

    app.configure_sets(
        Update,
        MainMenuPanoramaSet
            .ambiguous_with(LoadingScreenUpdateSet)
            .before(MainMenuUpdateSet)
            .before(MultiplayerMenuUpdateSet)
            .before(SettingsMenuUpdateSet)
            .in_set(InterfaceMenuUpdateSet),
    );

    // Build the panorama background.
    app.add_systems(
        OnEnter(ResourcePackState::Processing),
        MainMenuBackground::build_panorama
            .run_if(not(any_with_component::<MainMenuBackground>))
            .in_set(MainMenuPanoramaSet),
    );

    // Add the systems for the panorama background.
    app.add_systems(
        Update,
        MainMenuBackground::panorama_rotation
            .run_if(any_with_component::<MainMenuBackground>)
            .run_if(LoadingScreenState::is_hidden)
            .run_if(MainMenuBackgroundEnable::is_enabled)
            .in_set(MainMenuPanoramaSet),
    );
    app.add_systems(
        Update,
        MainMenuBackground::panorama_visibility
            .run_if(any_with_component::<MainMenuBackground>)
            .run_if(resource_exists_and_changed::<MainMenuBackgroundEnable>)
            .in_set(MainMenuPanoramaSet),
    );
}

/// A [`SystemSet`] for systems related to the main menu panorama.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct MainMenuPanoramaSet;

/// A marker [`Component`] for the main menu background.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Component)]
pub struct MainMenuBackground;

impl MainMenuBackground {
    const ROTATION_SPEED: f32 = 2.0;
    const RENDER_LAYER: RenderLayers = RenderLayers::layer(4);

    /// Rotates the panorama background.
    fn panorama_rotation(mut query: Query<&mut Transform, With<Self>>, time: Res<Time<Virtual>>) {
        let delta = time.delta_seconds().mul(Self::ROTATION_SPEED).min(0.2).to_radians();
        for mut transform in &mut query {
            transform.rotate_y(delta);
        }
    }

    /// Sets the visibility of the panorama background.
    fn panorama_visibility(
        mut query: Query<&mut Visibility, With<Self>>,

        state: Res<State<InterfaceMenuState>>,
        enable: Res<MainMenuBackgroundEnable>,
    ) {
        let new =
            if enable.is_enabled_in(**state) { Visibility::Inherited } else { Visibility::Hidden };

        for mut vis in &mut query {
            *vis = new;
        }
    }

    /// Builds the main menu panorama background.
    pub(crate) fn build_panorama(world: &mut World) {
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
        let Some(material): Option<Handle<MainMenuBackgroundShader>> = world
            .get_resource_mut::<Assets<MainMenuBackgroundShader>>()
            .map(|mut assets| assets.add(shader))
        else {
            error!(
                "Attempted to build MainMenuBackground without adding the InterfacePanoramaPlugin"
            );
            return;
        };

        // Get a cube mesh
        let mesh: Handle<Mesh> = {
            // Create the mesh
            let mut mesh = Mesh::from(Cuboid::from_size(Vec3::splat(-16.0)));

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
        let enable = world.get_resource::<MainMenuBackgroundEnable>();
        let state = world.get_resource::<State<InterfaceMenuState>>();

        let visibility = match (enable, state) {
            (Some(enable), Some(state)) => {
                if enable.is_enabled_in(**state) {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                }
            }
            _ => Visibility::Hidden,
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

/// Whether the [`MainMenuBackground`] is enabled for different
/// [`InterfaceMenuState`]s.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Resource)]
#[reflect(Resource)]
pub struct MainMenuBackgroundEnable {
    /// Enables the [`MainMenuBackground`] during
    /// [`InterfaceMenuState::MultiplayerMenu`].
    pub multiplayer_menu: bool,
    /// Enables the [`MainMenuBackground`] during
    /// [`InterfaceMenuState::SettingsMenu`].
    pub settings_menu: bool,
}

impl MainMenuBackgroundEnable {
    /// Returns [`true`] if the [`MainMenuBackground`] is enabled for the
    /// current [`InterfaceMenuState`].
    #[must_use]
    #[inline]
    pub fn is_enabled(res: Res<Self>, state: Res<State<InterfaceMenuState>>) -> bool {
        res.is_enabled_in(**state)
    }

    /// Returns [`true`] if the [`MainMenuBackground`] is enabled for the given
    /// [`InterfaceMenuState`].
    #[must_use]
    pub fn is_enabled_in(&self, state: InterfaceMenuState) -> bool {
        match state {
            InterfaceMenuState::MainMenu => true,
            InterfaceMenuState::MultiplayerMenu => self.multiplayer_menu,
            InterfaceMenuState::SettingsMenu => self.settings_menu,
        }
    }
}
