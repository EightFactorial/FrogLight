use mc_rs_macros::impl_atlasdata;

use crate::assets::textureatlases::TextureAtlasType;

// TODO: Add the rest of the widgets
impl_atlasdata! {
    WidgetAtlas,
    (256, 256),
    "minecraft:gui/widgets",
    TextureAtlasType::Widget,
    HOTBAR = [0, 0, 182, 22],
    HOTBAR_SELECTED = [0, 22, 24, 46],

    OFFHAND_SLOT_0 = [24, 23, 46, 45],
    OFFHAND_SLOT_1 = [60, 23, 82, 45],

    BUTTON_MENU_SELECTED = [0, 46, 200, 66],
    BUTTON_MENU = [0, 66, 200, 86],
    BUTTON_MENU_HIGHLIGHTED = [0, 86, 200, 106],

    BUTTON_LANGUAGE = [0, 106, 20, 126],
    BUTTON_LANGUAGE_HIGHLIGHTED = [20, 106, 40, 126],

    BUTTON_LOCK = [0, 146, 20, 166],
    BUTTON_LOCK_HIGHLIGHTED = [0, 166, 20, 186],
    BUTTON_LOCK_DISABLED = [0, 186, 20, 206],

    BUTTON_UNLOCKED = [20, 146, 40, 166],
    BUTTON_UNLOCKED_HIGHLIGHTED = [20, 166, 40, 186],
    BUTTON_UNLOCKED_DISABLED = [20, 186, 40, 206],
}
