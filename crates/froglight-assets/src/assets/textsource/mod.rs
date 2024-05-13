use bevy_app::App;
use bevy_reflect::Reflect;

#[doc(hidden)]
pub(super) fn build(app: &mut App) { app.register_type::<TextSource>(); }

/// A source of text loaded from a file
#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum TextSource {
    /// A JSON file
    Json(#[reflect(ignore)] serde_json::Value),
    /// A raw text file
    RawText(#[reflect(ignore)] String),
}
