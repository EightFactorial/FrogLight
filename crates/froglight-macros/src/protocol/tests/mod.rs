use attribute_derive::FromAttr;
use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident};

mod read;
mod write;

pub(super) fn generate_tests(input: &DeriveInput) -> TokenStream {
    let Ok(test_attrs) = TestAttributes::from_attributes(&input.attrs) else {
        return TokenStream::new();
    };

    let mut output = TokenStream::new();
    for test in &test_attrs.tests {
        match test.as_str() {
            "read_default" => read::read_default(input, &test_attrs, &mut output),
            "read_example" => read::read_example(input, &test_attrs, &mut output),
            "write_default" => write::write_default(input, &test_attrs, &mut output),
            "write_example" => write::write_example(input, &test_attrs, &mut output),
            unk => {
                panic!("Could not generate unknown test: '{unk}'!")
            }
        }
    }

    output
}

/// Attributes for test generation.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
struct TestAttributes {
    /// A list of tests to run.
    tests: Vec<String>,

    /// Example bytes used for test verification.
    #[attribute(optional)]
    bytes: Option<Vec<u8>>,
}

/// Generate a test name for the given input and test name.
fn test_name(input: &DeriveInput, test: &str) -> Ident {
    let item_name = input.ident.to_string().to_lowercase();
    Ident::new(&format!("{item_name}_{test}"), input.ident.span())
}
