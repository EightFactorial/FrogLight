use bevy::prelude::*;
use mc_rs_resourcepack::assets::textureatlases::{atlases::WidgetAtlas, AtlasFromWorld};

use crate::{
    menus::{
        app_menus::{
            main_menu::buttons::{MainMenuButtonAtlasComponent, MainMenuButtonTrigger},
            states::MainMenuState,
        },
        states::menus::MenuComponentMenusSet,
        traits::{AddMenuResource, MenuComponent},
    },
    resources::{
        font::{shadows::TextShadow, DefaultTextStyle},
        scale::GuiScaleComponent,
    },
};

use super::ButtonsNodeComponent;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct OptionsButtonComponent;

impl MenuComponent for OptionsButtonComponent {
    fn setup(app: &mut App) {
        app.add_systems(
            PreUpdate,
            Self::pressed.in_set(MenuComponentMenusSet).run_if(
                in_state(MainMenuState::MainMenu).and_then(ButtonsNodeComponent::any_interactions),
            ),
        );
    }

    fn build(parent: Entity, world: &mut World) {
        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building ButtonsNodeComponent");

        let (handle, index) = world
            .get_atlas_and_index(WidgetAtlas, WidgetAtlas::BUTTON_MENU)
            .expect("texture atlas and index");
        let handle = handle.clone();

        // Add the texture atlas to the menu resources.
        world.add_menu_resource(handle.clone().untyped());

        let button = world
            .spawn((
                OptionsButtonComponent,
                MainMenuButtonTrigger,
                ButtonBundle {
                    style: Style {
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        let background = world
            .spawn((
                GuiScaleComponent::new(200, 20),
                MainMenuButtonAtlasComponent,
                AtlasImageBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    texture_atlas: handle,
                    texture_atlas_image: index,
                    ..Default::default()
                },
            ))
            .set_parent(button)
            .id();

        let font_style: TextStyle = world.resource::<DefaultTextStyle>().clone().into();
        let mut shadow_style = font_style.clone();
        shadow_style.color = Color::BLACK;

        TextShadow::create_text_with_shadow("Options", background, world);
    }
}

impl OptionsButtonComponent {
    fn pressed(
        query: Query<&Interaction, (Changed<Interaction>, With<Self>)>,
        mut state: ResMut<NextState<MainMenuState>>,
    ) {
        if query.iter().any(|int| matches!(int, Interaction::Pressed)) {
            #[cfg(any(debug_assertions, feature = "debug"))]
            debug!("OptionsButtonComponent pressed");

            state.set(MainMenuState::Options);
        }
    }
}
