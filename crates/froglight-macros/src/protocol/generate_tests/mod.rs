use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident};

use super::Attributes;

mod read_tests;
mod write_tests;

/// Generate tests for `FrogRead` and `FrogWrite` implementations.
pub(super) fn generate_tests(input: &DeriveInput, attrs: &Attributes) -> proc_macro::TokenStream {
    // Collect tokens from generated tests
    let mut output = TokenStream::new();

    for test in &attrs.tests {
        match test.as_str() {
            "read_example" => read_tests::read_example(input, attrs, &mut output),
            "write_example" => write_tests::write_example(input, attrs, &mut output),
            "read_verify" => read_tests::read_default(input, attrs, &mut output),
            "write_verify" => write_tests::write_default(input, attrs, &mut output),
            unk => panic!("Unknown test: `{unk}`"),
        }
    }

    output.into()
}

/// Generate a test name for the given input and test name.
fn test_name(input: &DeriveInput, test: &str) -> Ident {
    let item_name = input.ident.to_string().to_lowercase();
    Ident::new(&format!("{item_name}_{test}"), input.ident.span())
}
