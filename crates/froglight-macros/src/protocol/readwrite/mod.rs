use proc_macro2::TokenStream;
use syn::DeriveInput;

pub(super) fn generate_read(_input: &DeriveInput) -> TokenStream { TokenStream::new() }

pub(super) fn generate_write(_input: &DeriveInput) -> TokenStream { TokenStream::new() }
