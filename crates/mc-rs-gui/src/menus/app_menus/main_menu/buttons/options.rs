use bevy::prelude::*;
use mc_rs_resourcepack::assets::{
    resourcepacks::ResourcePacks,
    textureatlases::{atlases::WidgetAtlas, AtlasFromWorld},
};

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

        let background = if let Some((handle, index)) =
            world.get_atlas_and_index(WidgetAtlas, WidgetAtlas::BUTTON_MENU)
        {
            let bundle = Self::button_bundle(handle.clone(), index, world);
            world.spawn(bundle).set_parent(button).id()
        } else {
            let bundle = Self::fallback_bundle(world);
            world.spawn(bundle).set_parent(button).id()
        };

        world.entity_mut(background).insert((
            GuiScaleComponent::new(200, 20),
            MainMenuButtonAtlasComponent,
        ));

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

    fn button_bundle(
        handle: Handle<TextureAtlas>,
        index: UiTextureAtlasImage,
        world: &mut World,
    ) -> AtlasImageBundle {
        // Add the texture atlas to the menu resources.
        world.add_menu_resource(handle.clone().untyped());

        // Create the bundle.
        AtlasImageBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            texture_atlas: handle.clone(),
            texture_atlas_image: index,
            ..Default::default()
        }
    }

    fn fallback_bundle(world: &mut World) -> ImageBundle {
        // Get the fallback texture.
        let fallback = world.resource::<ResourcePacks>().fallback.clone();

        // Add the texture atlas to the menu resources.
        world.add_menu_resource(fallback.clone().untyped());

        // Create the bundle.
        ImageBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            image: fallback.into(),
            ..Default::default()
        }
    }
}
