use proc_macro2::{Ident, TokenStream};
use syn::DeriveInput;

mod list;

/// Generate tests for the `Transcode` derive
pub fn generate_tests(input: proc_macro::TokenStream, extra_tests: Option<&[&str]>) -> TokenStream {
    let DeriveInput { ident, attrs, .. } = syn::parse(input).expect("Failed to DeriveInput");

    // A collection of tests
    let mut tests = TokenStream::new();

    // Find all `#[mctest(...)]` attributes
    for attr in attrs.into_iter() {
        if attr.path().is_ident("test") {
            attr.parse_nested_meta(|meta| {
                if let Some(attr_ident) = meta.path.get_ident() {
                    let name = attr_ident.to_string();

                    tests.extend(
                        match_test(&name, &ident)
                            .unwrap_or_else(|| panic!("Unknown test: `{name}`")),
                    );
                }

                Ok(())
            })
            .unwrap();
        }
    }

    // Add extra tests
    if let Some(extra_tests) = extra_tests {
        for &test in extra_tests {
            tests.extend(
                match_test(test, &ident).unwrap_or_else(|| panic!("Unknown extra test: `{test}`")),
            );
        }
    }

    tests
}

/// Generate a test from the test name and struct/enum name
fn match_test(name: &str, ident: &Ident) -> Option<TokenStream> {
    match name {
        "encode" => Some(list::encode_test(ident)),
        "transcode" => Some(list::transcode_test(ident)),
        "var_transcode" => Some(list::var_transcode_test(ident)),
        _ => None,
    }
}
