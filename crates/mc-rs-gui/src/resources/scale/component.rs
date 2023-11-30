use bevy::prelude::*;

use super::GuiScale;

/// The target size of the GUI component.
///
/// This is used to scale the GUI to the correct size when the [`GuiScale`] changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Component)]
pub struct GuiScaleComponent(pub UVec2);

impl GuiScaleComponent {
    /// Returns `true` if a [`GuiScaleComponent`] was added.
    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn scale_added(query: Query<(), Added<GuiScaleComponent>>) -> bool {
        !query.is_empty()
    }

    /// Update the size [`Entity`]s who had a [`GuiScaleComponent`] added.
    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn added_update(
        mut query: Query<(&mut Style, &GuiScaleComponent), Added<GuiScaleComponent>>,
        scale: Res<GuiScale>,
    ) {
        query.iter_mut().for_each(|(mut style, scale_comp)| {
            Self::update_size(&mut style, scale_comp, &scale);
        });
    }

    /// Update the size of [`Entity`]s when [`GuiScale`] changes.
    #[allow(clippy::needless_pass_by_value)]
    pub(super) fn resize_update(
        mut query: Query<(&mut Style, &GuiScaleComponent)>,
        scale: Res<GuiScale>,
    ) {
        query.iter_mut().for_each(|(mut style, scale_comp)| {
            Self::update_size(&mut style, scale_comp, &scale);
        });
    }

    /// Set the [`Style`] of an [`Entity`] based on the [`GuiScale`].
    #[allow(clippy::cast_precision_loss, clippy::trivially_copy_pass_by_ref)]
    fn update_size(style: &mut Style, scale_comp: &GuiScaleComponent, scale: &GuiScale) {
        let new_size = scale.value() * **scale_comp;

        style.width = Val::Px(new_size.x as f32);
        style.height = Val::Px(new_size.y as f32);
    }

    /// Create a new [`GuiScaleComponent`] with the given width and height.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self { Self(UVec2::new(width, height)) }
}

impl From<UVec2> for GuiScaleComponent {
    fn from(scale: UVec2) -> Self { Self(scale) }
}

impl From<(u32, u32)> for GuiScaleComponent {
    fn from(scale: (u32, u32)) -> Self { Self(UVec2::new(scale.0, scale.1)) }
}

impl From<[u32; 2]> for GuiScaleComponent {
    fn from(scale: [u32; 2]) -> Self { Self(UVec2::new(scale[0], scale[1])) }
}

impl From<GuiScaleComponent> for UVec2 {
    fn from(scale: GuiScaleComponent) -> Self { scale.0 }
}

impl From<GuiScaleComponent> for (u32, u32) {
    fn from(scale: GuiScaleComponent) -> Self { (scale.0.x, scale.0.y) }
}

impl From<GuiScaleComponent> for [u32; 2] {
    fn from(scale: GuiScaleComponent) -> Self { [scale.0.x, scale.0.y] }
}
