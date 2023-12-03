use bevy::prelude::*;

use crate::menus::states::menus::MenuComponentState;

use super::DefaultTextStyle;

pub(super) fn setup(app: &mut App) {
    app.add_systems(
        Update,
        TextShadow::update_shadows.run_if(
            in_state(MenuComponentState::Menus).and_then(resource_changed::<DefaultTextStyle>()),
        ),
    );
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TextShadow;

impl TextShadow {
    fn get_offset(text: &Text) -> f32 { text.sections[0].style.font_size / 8.0 }

    #[allow(clippy::type_complexity)]
    fn update_shadows(mut query: Query<(&mut Style, &Text), (Changed<Text>, With<TextShadow>)>) {
        for (mut style, text) in query.iter_mut() {
            let offset = Self::get_offset(text);

            style.top = Val::Px(offset);
            style.left = Val::Px(offset);
        }
    }

    pub fn create_text_with_shadow(text: &str, parent: Entity, world: &mut World) -> Entity {
        let default = world.resource::<DefaultTextStyle>();

        let font_style: TextStyle = default.clone().into();
        let text = Text::from_section(text, font_style);

        world
            .spawn((
                TextShadow,
                TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        align_self: AlignSelf::Center,
                        justify_self: JustifySelf::Center,
                        ..Default::default()
                    },
                    text: text.clone(),
                    z_index: ZIndex::Global(i32::MAX - 128),
                    ..Default::default()
                },
            ))
            .with_children(|node| {
                node.spawn(Self::create_shadow_text_bundle(text));
            })
            .set_parent(parent)
            .id()
    }

    pub fn create_shadow_text_bundle(mut text: Text) -> impl Bundle {
        for section in &mut text.sections {
            section.style.color = Self::create_shadow_color(section.style.color);
        }

        (
            TextShadow,
            TextBundle {
                style: Style {
                    top: Val::Px(Self::get_offset(&text)),
                    left: Val::Px(Self::get_offset(&text)),
                    ..Default::default()
                },
                text,
                z_index: ZIndex::Global(i32::MAX - 256),
                ..Default::default()
            },
        )
    }

    const COLOR_RATIO: f32 = 1.0;
    const BLACK_RATIO: f32 = 4.0;

    /// Returns the shadow color for the given color.
    pub fn create_shadow_color(color: Color) -> Color {
        let black = Color::BLACK;

        let r = (color.r() * Self::COLOR_RATIO + black.r() * Self::BLACK_RATIO)
            / (Self::COLOR_RATIO + Self::BLACK_RATIO);
        let g = (color.g() * Self::COLOR_RATIO + black.g() * Self::BLACK_RATIO)
            / (Self::COLOR_RATIO + Self::BLACK_RATIO);
        let b = (color.b() * Self::COLOR_RATIO + black.b() * Self::BLACK_RATIO)
            / (Self::COLOR_RATIO + Self::BLACK_RATIO);

        Color::rgba(r, g, b, 0.75)
    }
}
