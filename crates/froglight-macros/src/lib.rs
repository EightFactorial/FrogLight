#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub(crate) mod manifest;

// --- Asset Macros ---

#[cfg(feature = "asset")]
mod asset;

// --- Protocol Macros ---

#[cfg(feature = "froglight-protocol")]
mod protocol;

#[cfg(feature = "froglight-protocol")]
macro_rules! protocol_macro {
    ($macro:ident, $name:ident, $kind:expr) => {
        #[allow(missing_docs)]
        #[cfg(feature = "froglight-protocol")]
        #[proc_macro_derive($macro, attributes(frog))]
        pub fn $name(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            protocol::frog_protocol(input, $kind)
        }
    };
}

#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogRead, frog_read, protocol::GenerateType::Read);
#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogWrite, frog_write, protocol::GenerateType::Write);
#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogReadWrite, frog_readwrite, protocol::GenerateType::ReadWrite);
#[cfg(feature = "froglight-protocol")]
protocol_macro!(FrogTest, frog_test, protocol::GenerateType::Test);

/// A macro for generating a version state implementation.
#[cfg(feature = "froglight-protocol")]
#[proc_macro]
pub fn frog_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_state(input)
}

// --- World Macros ---

#[cfg(feature = "world")]
mod world;
