#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "io")]
mod io;

pub mod prelude {
    //! Re-exports of common types, traits, and macros.

    /// Placeholder for the prelude.
    pub struct _Placeholder;
}
