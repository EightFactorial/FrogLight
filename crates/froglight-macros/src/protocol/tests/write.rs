use proc_macro2::TokenStream;
use syn::DeriveInput;

use super::Attributes;

pub(super) fn write_default(
    _input: &DeriveInput,
    _test_attrs: &Attributes,
    _output: &mut TokenStream,
) {
}

pub(super) fn write_example(
    _input: &DeriveInput,
    _test_attrs: &Attributes,
    _output: &mut TokenStream,
) {
}
