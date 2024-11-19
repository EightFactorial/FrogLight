use proc_macro::TokenStream;

mod attribute;
#[allow(clippy::module_inception)]
mod block;

/// Generate the block attributes.
pub(super) fn impl_generated_attributes(tokens: TokenStream) -> TokenStream {
    attribute::impl_generated_attributes(tokens.into()).into()
}

/// Generate the block structs.
pub(super) fn impl_generated_blocks(tokens: TokenStream) -> TokenStream {
    block::impl_generated_blocks(tokens.into()).into()
}
