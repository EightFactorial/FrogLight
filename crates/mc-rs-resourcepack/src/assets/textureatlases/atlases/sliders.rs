use mc_rs_macros::impl_atlasdata;

use crate::assets::textureatlases::TextureAtlasType;

impl_atlasdata! {
    SliderAtlas,
    (256, 256),
    "minecraft:gui/slider",
    TextureAtlasType::Slider,
    SLIDER_EMPTY = [0, 0, 200, 20],
    SLIDER_EMPTY_HIGHLIGHT = [0, 20, 200, 40],
    SLIDER_FULL = [0, 40, 200, 60],
    SLIDER_FULL_HIGHLIGHT = [0, 60, 200, 80],
}
