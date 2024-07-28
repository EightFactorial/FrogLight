use bevy::{
    core::Name,
    ecs::world::DeferredWorld,
    prelude::{BuildChildren, Bundle, Component, Entity, NodeBundle, ReflectComponent},
    reflect::Reflect,
    ui::{Display, PositionType, Style, Val},
};

/// The child entity that has the loading screen.
#[derive(Clone, Copy, Reflect, Component)]
#[reflect(Component)]
pub(super) struct LoadingScreenChild(pub(super) Entity);

impl LoadingScreenChild {
    const NAME: &'static str = "Loading Screen Child";
    pub(super) fn construct(world: &mut DeferredWorld, parent: Entity) -> Entity {
        world.commands().spawn(ChildBundle::default()).set_parent(parent).id()
    }
}

#[derive(Bundle)]
struct ChildBundle {
    node_bundle: NodeBundle,
    name: Name,
}

impl Default for ChildBundle {
    fn default() -> Self {
        Self {
            node_bundle: NodeBundle {
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Relative,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            name: Name::new(LoadingScreenChild::NAME),
        }
    }
}
