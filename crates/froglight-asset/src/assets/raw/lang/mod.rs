//! [`SingleLanguageMap`] and related types.

use bevy_app::App;

mod asset;
pub use asset::SingleLanguageMap;

mod current;
pub use current::CurrentLanguage;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    asset::build(app);
    current::build(app);
}
