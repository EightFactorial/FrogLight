#![doc = include_str!("../README.md")]

#[cfg(feature = "bevy")]
pub mod bevy;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    /// TODO: Remove
    pub struct NetworkPlaceholder;
}
