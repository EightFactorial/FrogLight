use bevy_app::App;
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    reflect::ReflectResource,
    system::{ResMut, Resource},
};
use bevy_reflect::{std_traits::ReflectDefault, Reflect};

use super::OneTick;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    app.init_resource::<CurrentTick>().register_type::<CurrentTick>();
    app.add_systems(OneTick, CurrentTick::increment_tick_counter);
}

/// A [`Resource`] that keeps track of the current tick.
///
/// A tick is equal to `50ms`, or `1 / 20` of a
/// second, when using [`Real`](bevy_time::Real) time.
///
/// This will always be a value between `0` and `19`.
///
/// ---
///
/// This doesn't keep track of ticks from the server,
/// but rather the current [`Virtual`](bevy_time::Virtual) tick.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource, Reflect)]
#[reflect(Default, Resource)]
pub struct CurrentTick(pub u8);

impl CurrentTick {
    /// A [`System`](bevy_ecs::system::System) that
    /// increments the [`CurrentTick`].
    ///
    /// This system should be run every tick.
    pub fn increment_tick_counter(mut counter: ResMut<Self>) {
        let tick = counter.wrapping_add(1);
        if tick < 20 {
            **counter = tick;
        } else {
            **counter = 0;
        }
    }
}
