use bevy::math::Rect;
use mc_rs_core::ResourceLocation;

mod icons;
pub use icons::IconAtlas;

mod sliders;
pub use sliders::SliderAtlas;

mod widgets;
pub use widgets::WidgetAtlas;

pub trait TextureAtlasData: 'static {
    fn size() -> (u32, u32);
    fn path() -> ResourceLocation;
    fn coords() -> Vec<Rect>;
}
