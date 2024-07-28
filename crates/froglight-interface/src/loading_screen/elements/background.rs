use bevy::{
    app::App,
    color::Color,
    core::Name,
    ecs::world::DeferredWorld,
    prelude::{
        BuildChildren, Bundle, Component, Entity, NodeBundle, ReflectComponent, ReflectDefault,
    },
    reflect::Reflect,
    ui::{Display, PositionType, Style, Val},
};

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<LoadingScreenBackground>(); }

/// The background of the [`LoadingScreen`](super::super::LoadingScreen).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
#[reflect(Default, Component)]
pub struct LoadingScreenBackground;

impl LoadingScreenBackground {
    const NAME: &'static str = "Loading Screen Background";
    pub(crate) fn construct(world: &mut DeferredWorld, parent: Entity) -> Entity {
        world.commands().spawn(BackgroundBundle::default()).set_parent(parent).id()
    }
}

#[derive(Bundle)]
struct BackgroundBundle {
    background: LoadingScreenBackground,
    node_bundle: NodeBundle,
    name: Name,
}

impl Default for BackgroundBundle {
    fn default() -> Self {
        Self {
            background: LoadingScreenBackground,
            node_bundle: NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..Default::default()
            },
            name: Name::new(LoadingScreenBackground::NAME),
        }
    }
}
