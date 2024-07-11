use proc_macro::TokenStream;

mod create_attributes;
mod create_blocks;

/// Generate the block attributes.
pub(super) fn frog_create_attributes(tokens: TokenStream) -> TokenStream {
    create_attributes::generate_attributes(tokens).into()
}

/// Generate the block structs.
pub(super) fn frog_create_blocks(tokens: TokenStream) -> TokenStream {
    create_blocks::generate_blocks(tokens).into()
}

/// Generate the block trait impls.
pub(super) fn frog_create_block_impls(tokens: TokenStream) -> TokenStream {
    create_blocks::generate_block_impls(tokens).into()
}
