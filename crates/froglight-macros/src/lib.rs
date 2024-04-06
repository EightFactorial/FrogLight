#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

// --- Asset Macros ---

#[cfg(feature = "asset")]
mod asset;

// --- Protocol Macros ---

#[cfg(feature = "froglight-protocol")]
mod protocol;

#[cfg(feature = "froglight-protocol")]
macro_rules! protocol_macro {
    ($name:ident, $fn:ident) => {
        #[allow(missing_docs)]
        #[cfg(feature = "froglight-protocol")]
        #[proc_macro_derive($name, attributes(frog))]
        pub fn $fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            protocol::$fn(input, true)
        }
    };
}

#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogRead, frog_read);
#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogWrite, frog_write);
#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogReadWrite, frog_read_write);

/// A macro for generating tests for `FrogRead` and `FrogWrite` implementations.
#[cfg(feature = "froglight-protocol")]
#[proc_macro_derive(FrogTest, attributes(frog))]
pub fn frog_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_test(input)
}

/// A macro for generating a version state implementation.
#[cfg(feature = "froglight-protocol")]
#[proc_macro]
pub fn frog_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_state(input)
}

// --- World Macros ---

#[cfg(feature = "world")]
mod world;
