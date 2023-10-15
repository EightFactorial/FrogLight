use proc_macro2::TokenStream;
use quote::quote;

use crate::proto::{derive_decode, derive_encode, generate_tests};

/// Tests that are always generated
static STATIC_TESTS: &[&str] = &[];

/// Derive both `Encode` and `Decode`
pub fn derive_transcode(input: proc_macro::TokenStream) -> TokenStream {
    let encode = derive_encode(input.clone(), false);
    let decode = derive_decode(input.clone(), false);
    let tests = generate_tests(input, Some(STATIC_TESTS));

    quote! {
        #encode
        #decode

        #tests
    }
}
