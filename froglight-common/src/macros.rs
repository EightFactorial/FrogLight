//! TODO

/// A `const` macro for creating [`Ident`](crate::prelude::Ident)s.
///
/// This should only be used for `const` and `static` items,
/// as the methods on [`Ident`](crate::prelude::Ident) and
/// [`Identifier`](crate::prelude::Identifier) are generally faster.
///
/// # Panics
///
/// Panics if the string literal is not a valid
/// [`Ident`](crate::prelude::Ident).
///
/// ```rust
/// use froglight_common::prelude::*;
///
/// const CONST_DIRT: &Ident = ident!("froglight:blocks/dirt");
/// static STATIC_STONE: &Ident = ident!("froglight:stone");
/// ```
#[macro_export]
macro_rules! ident {
    ($str:literal) => {{
        match $crate::prelude::Ident::try_new($str) {
            Ok(ident) => ident,
            Err(..) => panic!(concat!("Invalid Identifier: `", $str, "`")),
        }
    }};
}
