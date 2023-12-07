use bevy::prelude::*;
use mc_rs_core::{enums::SoundType, sounds::SoundEvent, ResourceLocation};
use mc_rs_resourcepack::assets::{
    resourcepacks::AssetFromWorld, textureatlases::atlases::WidgetAtlas,
};

use crate::menus::{
    app_menus::states::MainMenuState,
    states::menus::MenuComponentMenusSet,
    traits::{AddMenuResource, MenuComponent},
};

pub mod multiplayer;
pub mod options;
pub mod quit;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct ButtonsNodeComponent;

impl MenuComponent for ButtonsNodeComponent {
    fn setup(app: &mut App) {
        app.add_systems(
            Update,
            (Self::button_state, Self::play_click)
                .in_set(MenuComponentMenusSet)
                .run_if(in_state(MainMenuState::MainMenu).and_then(Self::any_interactions)),
        );

        multiplayer::MultiplayerButtonComponent::setup(app);
        options::OptionsButtonComponent::setup(app);
        quit::QuitButtonComponent::setup(app);
    }

    fn build(parent: Entity, world: &mut World) {
        // Add click sound to MenuResources
        if let Some(sound) = world.get_sound("minecraft:random/click") {
            world.add_menu_resource(sound.clone().untyped());
        }

        #[cfg(any(debug_assertions, feature = "debug"))]
        trace!("Building ButtonsNodeComponent");
        let node = NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(5.0)),
                top: Val::Vw(5.0),
                bottom: Val::Px(50.0),
                ..Default::default()
            },
            ..Default::default()
        };

        let entity = world
            .spawn((
                ButtonsNodeComponent,
                node,
                #[cfg(any(debug_assertions, feature = "debug"))]
                Outline {
                    width: Val::Px(1.0),
                    color: Color::BLUE,
                    ..Default::default()
                },
            ))
            .set_parent(parent)
            .id();

        multiplayer::MultiplayerButtonComponent::build(entity, world);
        options::OptionsButtonComponent::build(entity, world);
        quit::QuitButtonComponent::build(entity, world);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuButtonTrigger;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct MainMenuButtonAtlasComponent;

impl ButtonsNodeComponent {
    fn any_interactions(
        query: Query<(), (Changed<Interaction>, With<MainMenuButtonTrigger>)>,
    ) -> bool {
        !query.is_empty()
    }

    #[allow(clippy::type_complexity)]
    fn button_state(
        query: Query<
            (&Children, &Interaction),
            (Changed<Interaction>, With<MainMenuButtonTrigger>),
        >,
        mut image: Query<&mut UiTextureAtlasImage, With<MainMenuButtonAtlasComponent>>,
    ) {
        for (children, interaction) in query.iter() {
            children.iter().for_each(|child| {
                if let Ok(mut image) = image.get_mut(*child) {
                    image.index = match interaction {
                        Interaction::None => WidgetAtlas::BUTTON_MENU,
                        Interaction::Hovered => WidgetAtlas::BUTTON_MENU_HIGHLIGHTED,
                        Interaction::Pressed => WidgetAtlas::BUTTON_MENU_SELECTED,
                    }
                }
            });
        }
    }

    fn play_click(
        query: Query<
            &Interaction,
            (
                Changed<Interaction>,
                With<multiplayer::MultiplayerButtonComponent>,
            ),
        >,
        mut events: EventWriter<SoundEvent>,
    ) {
        if query
            .iter()
            .any(|interaction| *interaction == Interaction::Pressed)
        {
            events.send(SoundEvent {
                asset: ResourceLocation::new("minecraft:random/click"),
                kind: SoundType::Global,
                position: None,
            });
        }
    }
}
