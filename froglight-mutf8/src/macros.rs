//! TODO

/// A `const` macro for creating [`MStr`] literals.
///
/// This should only be used for `const` and `static` items,
/// as the methods on [`MStr`] and [`MString`] are generally faster.
///
/// # Panics
///
/// Panics if the string literal is not valid MUTF-8.
///
/// ```rust
/// use froglight_mutf8::prelude::*;
///
/// const CONST_HELLO: &MStr = mutf8!("Hello, world!");
/// static STATIC_HELLO: &MStr = mutf8!("Hello, world!");
/// ```
#[macro_export]
macro_rules! mutf8 {
    ($str:literal) => {{
        match $crate::prelude::MStr::const_from_utf8($str) {
            Some(mstr) => mstr,
            None => panic!(concat!("Invalid MUTF-8 string literal: `", $str, "`")),
        }
    }};
}
