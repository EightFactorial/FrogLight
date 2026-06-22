//! A small wrapper around the [`yansi`](::yansi) crate,
//! overriding the default colors to match Minecraft's color codes.

pub use yansi::{Attribute, Condition};

mod color;
pub use color::Color;

mod paint;
pub use paint::{Paint, Painted};

mod style;
pub use style::Style;
