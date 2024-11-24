use proc_macro::TokenStream;

mod component;
#[allow(clippy::module_inception)]
mod entity;

/// A macro for generating entity components.
pub(super) fn impl_generated_components(tokens: TokenStream) -> TokenStream {
    component::impl_generated_components(tokens.into()).into()
}

/// A macro for generating entities.
pub(super) fn impl_generated_entities(tokens: TokenStream) -> TokenStream {
    entity::impl_generated_entities(tokens.into()).into()
}
