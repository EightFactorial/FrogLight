#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "assets")]
mod assets;

#[cfg(feature = "protocol")]
mod protocol;

#[cfg(feature = "protocol")]
macro_rules! protocol_macro {
    ($name:ident, $fn:ident) => {
        #[allow(missing_docs)]
        #[cfg(feature = "protocol")]
        #[proc_macro_derive($name, attributes(frog))]
        pub fn $fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            protocol::$fn(input, true)
        }
    };
}

protocol_macro!(FrogRead, frog_read);
protocol_macro!(FrogWrite, frog_write);
protocol_macro!(FrogReadWrite, frog_read_write);

/// A macro for generating tests for `FrogRead` and `FrogWrite` implementations.
#[cfg(feature = "protocol")]
#[proc_macro_derive(FrogTest, attributes(frog))]
pub fn frog_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_test(input)
}

#[cfg(feature = "world")]
mod world;
