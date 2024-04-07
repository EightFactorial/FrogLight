use proc_macro::TokenStream;
use syn::DeriveInput;

use super::Attributes;

/// Generate tests for `FrogRead` and `FrogWrite` implementations.
pub(super) fn generate_tests(_input: &DeriveInput, _attrs: &Attributes) -> TokenStream {
    TokenStream::new()
}
