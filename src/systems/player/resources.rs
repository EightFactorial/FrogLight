use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use mc_rs_core::schedule::{set::GameSet, state::ApplicationState};

pub(super) fn add_systems(app: &mut App) {
    app.insert_resource(Paused(true));

    app.add_systems(
        OnEnter(ApplicationState::Game),
        Paused::on_join.in_set(GameSet),
    );

    app.add_systems(
        Update,
        (
            Paused::on_escape,
            Paused::on_change.run_if(resource_changed::<Paused>()),
        )
            .chain()
            .in_set(GameSet),
    );

    app.add_systems(
        OnExit(ApplicationState::Game),
        Paused::on_leave.in_set(GameSet),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct Paused(pub bool);

impl Paused {
    fn on_join(mut paused: ResMut<Paused>) { **paused = false; }

    fn on_leave(mut paused: ResMut<Paused>) { **paused = true; }

    fn on_escape(input: Res<Input<KeyCode>>, mut paused: ResMut<Paused>) {
        if input.just_pressed(KeyCode::Escape) {
            **paused = !**paused;
        }
    }

    fn on_change(mut window: Query<&mut Window, With<PrimaryWindow>>, paused: Res<Paused>) {
        let mut window = window.single_mut();

        if **paused {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        } else {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
    }
}
