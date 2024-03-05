use attribute_derive::FromAttr;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod modifiers;
mod readwrite;
mod states;
mod tests;

/// Generates a `FrogRead` and `FrogWrite` implementation for the given struct
/// or enum.
pub(super) fn frog_read_write(tokens: TokenStream, tests: bool) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");
    let attrs = Attributes::from_attributes(&input.attrs).expect("Failed to parse attributes");

    let mut output = TokenStream::new();
    output.extend(TokenStream::from(readwrite::generate_read(&input, &attrs)));
    output.extend(TokenStream::from(readwrite::generate_write(&input, &attrs)));
    if tests {
        output.extend(TokenStream::from(tests::generate_tests(&input, &attrs)));
    }
    output
}

/// Generates a `FrogRead` implementation for the given struct or enum.
pub(super) fn frog_read(tokens: TokenStream, tests: bool) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");
    let attrs = Attributes::from_attributes(&input.attrs).expect("Failed to parse attributes");

    let mut output = TokenStream::new();
    output.extend(TokenStream::from(readwrite::generate_read(&input, &attrs)));
    if tests {
        output.extend(TokenStream::from(tests::generate_tests(&input, &attrs)));
    }
    output
}

/// Generates a `FrogWrite` implementation for the given struct or enum.
pub(super) fn frog_write(tokens: TokenStream, tests: bool) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");
    let attrs = Attributes::from_attributes(&input.attrs).expect("Failed to parse attributes");

    let mut output = TokenStream::new();
    output.extend(TokenStream::from(readwrite::generate_write(&input, &attrs)));
    if tests {
        output.extend(TokenStream::from(tests::generate_tests(&input, &attrs)));
    }
    output
}

/// Generates tests for `FrogRead` and `FrogWrite` implementations.
pub(super) fn frog_test(tokens: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");
    let attrs = Attributes::from_attributes(&input.attrs).expect("Failed to parse attributes");
    TokenStream::from(tests::generate_tests(&input, &attrs))
}

pub(super) fn frog_state(tokens: TokenStream) -> TokenStream {
    TokenStream::from(states::generate_state(tokens.into()))
}

/// Attributes for the `frog` attribute macro.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
struct Attributes {
    /// A list of tests to run.
    #[attribute(optional)]
    tests: Vec<String>,

    /// Example bytes used for test verification.
    #[attribute(optional)]
    bytes: Vec<u8>,

    /// The size of the bitset.
    #[attribute(optional, conflicts = [json])]
    bitset: Option<usize>,

    /// Whether to read/write as JSON.
    #[attribute(conflicts = [bitset])]
    json: bool,
}
