mod loaded;
pub use loaded::AssetsLoaded;

mod reload;
pub use reload::ReloadAssets;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    loaded::build(app);
    reload::build(app);
}
