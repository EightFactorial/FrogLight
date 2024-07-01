#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub(crate) mod manifest;

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

/// A macro for generating `State<Version>` implementations.
#[cfg(feature = "froglight-protocol")]
#[proc_macro]
pub fn frog_state(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    protocol::frog_state(input)
}

// --- Registry Macros ---

#[cfg(feature = "froglight-registry")]
mod registry;

/// A macro for generating `ConvertKey` implementations.
#[cfg(feature = "froglight-registry")]
#[proc_macro_derive(FrogRegistry, attributes(frog))]
pub fn frog_registry(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    registry::frog_registry_convertkey(input)
}

/// A macro for generating block attributes.
#[cfg(feature = "froglight-registry")]
#[proc_macro]
pub fn frog_create_attributes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    registry::frog_create_attributes(input)
}

/// A macro for generating block structs.
#[cfg(feature = "froglight-registry")]
#[proc_macro]
pub fn frog_create_blocks(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    registry::frog_create_blocks(input)
}

/// A macro for generating block trait impls.
#[cfg(feature = "froglight-registry")]
#[proc_macro]
pub fn frog_create_block_impls(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    registry::frog_create_block_impls(input)
}

/// A macro for generating registry trait impls.
#[cfg(feature = "froglight-registry")]
#[proc_macro]
pub fn frog_create_registry_impls(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    registry::frog_create_registry_impls(input)
}
