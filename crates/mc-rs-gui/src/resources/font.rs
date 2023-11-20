use bevy::prelude::*;

use super::scale::{GuiScale, GuiScaleEvent};

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Startup,
        DefaultTextStyle::initialize.run_if(not(resource_exists::<DefaultTextStyle>())),
    );

    app.add_systems(
        Update,
        (
            DefaultTextStyle::resize_font.run_if(on_event::<GuiScaleEvent>()),
            DefaultTextStyle::update_styles.run_if(
                resource_exists_and_changed::<DefaultTextStyle>()
                    .or_else(DefaultTextStyle::any_added_texts),
            ),
        )
            .chain(),
    );
}

/// TODO: Change font_size when [GuiScale](super::scale::GuiScale) changes.
#[derive(Debug, Default, Clone, Deref, DerefMut, Resource)]
pub struct DefaultTextStyle(pub TextStyle);

/// A component that can be added to a [Text] entity to ignore the [DefaultTextStyle].
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Component)]
pub struct IgnoreDefaultTextStyle;

impl DefaultTextStyle {
    // TODO: Get the actual formula for this.
    fn font_size(scale: &GuiScale) -> f32 { 10.0 + (scale.value() * 4) as f32 }

    /// Initialize the [`DefaultTextStyle`] resource.
    fn initialize(scale: Res<GuiScale>, mut commands: Commands) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Initializing DefaultTextStyle");

        commands.insert_resource(DefaultTextStyle(TextStyle {
            font_size: Self::font_size(&scale),
            color: Color::WHITE,
            ..Default::default()
        }));
    }

    /// Update the font size of the [`DefaultTextStyle`] when the [`GuiScale`] changes.
    fn resize_font(scale: Res<GuiScale>, mut style: ResMut<DefaultTextStyle>) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Resizing DefaultTextStyle");

        style.font_size = Self::font_size(&scale);
    }

    /// Returns true if a [`Text`] entity was added.
    fn any_added_texts(query: Query<(), (Added<Text>, Without<IgnoreDefaultTextStyle>)>) -> bool {
        !query.is_empty()
    }

    /// Updates all [`Text`] entities with the [`DefaultTextStyle`],
    /// ignoring those with the [`IgnoreDefaultTextStyle`] component.
    fn update_styles(
        mut query: Query<&mut Text, Without<IgnoreDefaultTextStyle>>,
        style: Res<DefaultTextStyle>,
    ) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        debug!("Updating TextStyles");

        query.iter_mut().for_each(|mut text| {
            style.set_text_style(&mut text);
        });
    }

    /// Updates the [`Text`] entity with the [`DefaultTextStyle`]'s font size
    pub fn set_text_style(&self, text: &mut Text) {
        text.sections
            .iter_mut()
            .for_each(|section| self.set_style(&mut section.style));
    }

    /// Updates the [`TextStyle`] with the [`DefaultTextStyle`]'s font size
    pub fn set_style(&self, style: &mut TextStyle) { style.font_size = self.font_size; }
}

impl From<TextStyle> for DefaultTextStyle {
    fn from(style: TextStyle) -> Self { Self(style) }
}

impl From<DefaultTextStyle> for TextStyle {
    fn from(style: DefaultTextStyle) -> Self { style.0 }
}
