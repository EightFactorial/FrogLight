use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident};

use super::Attributes;

mod read;
mod write;

pub(super) fn generate_tests(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    let mut output = TokenStream::new();
    for test in &attrs.tests {
        match test.as_str() {
            "read_example" => read::read_example(input, attrs, &mut output),
            "read_verify" => read::read_default(input, attrs, &mut output),
            "write_default" => write::write_default(input, attrs, &mut output),
            "write_verify" => write::write_example(input, attrs, &mut output),
            unk => {
                panic!("Could not generate unknown test: '{unk}'!")
            }
        }
    }

    output
}

/// Generate a test name for the given input and test name.
fn test_name(input: &DeriveInput, test: &str) -> Ident {
    let item_name = input.ident.to_string().to_lowercase();
    Ident::new(&format!("{item_name}_{test}"), input.ident.span())
}
