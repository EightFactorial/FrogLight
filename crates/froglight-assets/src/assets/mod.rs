//! Different kinds of assets

mod blockstate;
pub use blockstate::*;

mod font;
pub use font::*;

mod language;
pub use language::*;

mod model;
pub use model::*;

mod particle;
pub use particle::*;

mod resourcepack;
pub use resourcepack::*;

mod sound;
pub use sound::*;

mod textsource;
pub use textsource::*;

#[doc(hidden)]
pub(super) fn build(app: &mut bevy_app::App) {
    blockstate::build(app);
    font::build(app);
    language::build(app);
    model::build(app);
    particle::build(app);
    resourcepack::build(app);
    sound::build(app);
    textsource::build(app);
}
