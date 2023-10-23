use belly::prelude::*;
use bevy::prelude::*;

use mc_rs_core::schedule::{
    set::{GameSet, InMenuSet},
    state::ApplicationState,
};

pub mod hotbar;
pub mod inventory;
pub mod pause;

/// Add interface systems to the app
pub(super) fn setup(app: &mut App) {
    app.add_systems(Startup, InterfaceRoot::load);

    app.add_systems(
        OnEnter(ApplicationState::Game),
        InterfaceRoot::create.in_set(GameSet),
    );

    app.add_systems(
        OnEnter(ApplicationState::InMenu),
        InterfaceRoot::destroy
            .run_if(resource_exists::<InterfaceRoot>())
            .in_set(InMenuSet),
    );

    inventory::setup_menus(app);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct InterfaceRoot(pub Entity);

impl InterfaceRoot {
    fn load(mut commands: Commands) { commands.add(StyleSheet::load("style/player.ess")); }

    fn create(mut commands: Commands) {
        let id = commands.spawn_empty().id();
        commands.insert_resource(InterfaceRoot(id));

        commands.add(eml! {
            <body c:player>
            </body>
        })
    }

    fn destroy(root: Res<InterfaceRoot>, mut elements: Elements, mut commands: Commands) {
        commands.entity(**root).despawn_recursive();
        elements.select("body.player").remove();
    }
}
