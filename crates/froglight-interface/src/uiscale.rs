use std::num::NonZeroU8;

use bevy_app::{App, PreUpdate};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::{
    event::{Event, EventReader, EventWriter},
    query::With,
    schedule::{common_conditions::on_event, Condition, IntoSystemConfigs},
    system::{Query, Res, ResMut, Resource},
};
use bevy_log::{debug, warn};
use bevy_ui::UiScale;
use bevy_window::{PrimaryWindow, WindowResized};
use froglight_core::systemsets::InterfacePreUpdateSet;

/// The virtual window width.
///
/// All Ui should be designed for this width,
/// the scale will be adjusted to fit the actual window size.
pub const WINDOW_VIRTUAL_WIDTH: u32 = 380;

/// The virtual window height.
///
/// All Ui should be designed for this height,
/// the scale will be adjusted to fit the actual window size.
pub const WINDOW_VIRTUAL_HEIGHT: u32 = 240;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    // Add the UIScale update event
    app.add_event::<UiScaleUpdate>();

    // Add the UiScale Mode and Limit resources
    app.init_resource::<UiScaleMode>().init_resource::<UiScaleLimit>();

    // Add the UiScale update system
    app.add_systems(
        PreUpdate,
        UiScaleLimit::update_uiscale
            .run_if(on_event::<WindowResized>().or_else(on_event::<UiScaleUpdate>()))
            .run_if(UiScaleMode::enabled)
            .in_set(InterfacePreUpdateSet),
    );
}

/// The UI scale mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct UiScaleMode(pub bool);

impl Default for UiScaleMode {
    fn default() -> Self { UiScaleMode(true) }
}

impl UiScaleMode {
    /// Creates a new UI scale mode.
    #[must_use]
    pub const fn new(enabled: bool) -> Self { UiScaleMode(enabled) }

    /// Returns `true` if the UI scale is enabled.
    #[must_use]
    pub const fn is_enabled(&self) -> bool { self.0 }

    /// A [`Condition`](bevy_ecs::schedule::Condition) that checks if the
    /// [`UiScaleMode`] is enabled.
    #[must_use]
    pub fn enabled(mode: Res<Self>) -> bool { mode.is_enabled() }
}

/// The UI scale limit.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Resource)]
pub struct UiScaleLimit(pub Option<NonZeroU8>);

impl UiScaleLimit {
    /// Creates a new UI scale limit.
    #[must_use]
    pub const fn new(limit: u8) -> Self { UiScaleLimit(NonZeroU8::new(limit)) }

    /// Returns the UI scale limit.
    #[must_use]
    pub const fn get(&self) -> Option<u8> {
        if let Some(limit) = self.0 {
            Some(limit.get())
        } else {
            None
        }
    }

    /// A [`System`](bevy_ecs::system::System) that updates the UI scale when
    /// the [`PrimaryWindow`](bevy_window::PrimaryWindow) is resized.
    fn update_uiscale(
        query: Query<(), With<PrimaryWindow>>,
        limit: Res<Self>,
        mut scale: ResMut<UiScale>,
        mut resize_events: EventReader<WindowResized>,
        mut update_events: EventWriter<UiScaleUpdate>,
    ) {
        if let Some(event) = resize_events.read().filter(|&e| query.contains(e.window)).last() {
            #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
            if let Some(new_scale) =
                Self::set(&mut scale, limit.get(), event.width as u32, event.height as u32)
            {
                update_events.send(UiScaleUpdate(new_scale));
            }
        }
    }

    /// Updates the UI scale.
    ///
    /// Returns `Some(scale)` if the [`UiScale`] was set.
    fn set(uiscale: &mut UiScale, limit: Option<u8>, width: u32, height: u32) -> Option<u8> {
        let mut scale = Self::calculate(width, height);

        // Cap the scale to the limit.
        if let Some(limit) = limit {
            scale = scale.min(limit);
        }

        // Update the UI scale if it changed.
        let fscale = f32::from(scale);
        if (uiscale.0 - fscale).abs() < f32::EPSILON {
            None
        } else {
            debug!("UiScale: {scale}");
            uiscale.0 = fscale;
            Some(scale)
        }
    }

    /// Calculates a new scale based on the window size.
    fn calculate(width: u32, height: u32) -> u8 {
        let w_scale = width / WINDOW_VIRTUAL_WIDTH;
        let h_scale = height / WINDOW_VIRTUAL_HEIGHT;

        let output = w_scale.min(h_scale).max(1);
        u8::try_from(output).unwrap_or_else(|_| {
            warn!("UiScale calculation overflowed!");
            1u8
        })
    }
}

/// An event that is sent when the [`UiScale`] is updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut, Event)]
pub struct UiScaleUpdate(pub u8);

impl UiScaleUpdate {
    /// Creates a new UI scale update event.
    #[must_use]
    pub const fn new(scale: u8) -> Self { UiScaleUpdate(scale) }
}
