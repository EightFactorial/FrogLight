//! [`BlockModel`] and [`ItemModel`] assets.

use bevy_app::App;

mod block;
pub use block::*;

mod item;
// pub use item::*;

#[doc(hidden)]
pub(super) fn build(app: &mut App) {
    block::build(app);
    item::build(app);
}
