use proc_macro::TokenStream;

mod attribute;
#[allow(clippy::module_inception)]
mod block;
mod traits;

/// Generate the block attributes.
pub(super) fn impl_generated_attributes(tokens: TokenStream) -> TokenStream {
    attribute::impl_generated_attributes(tokens.into()).into()
}

/// Generate the block structs.
pub(super) fn impl_generated_blocks(tokens: TokenStream) -> TokenStream {
    block::impl_generated_blocks(tokens.into()).into()
}

/// Implement block traits.
pub(super) fn impl_block_traits(tokens: TokenStream) -> TokenStream {
    traits::impl_block_traits(tokens.into()).into()
}
