use attribute_derive::FromAttr;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod generate_impls;
mod generate_state;
mod generate_tests;

/// Generate a `FrogRead`, `FrogWrite`, or both implementations.
///
/// Also generates tests if specified in the attributes.
pub(super) fn frog_protocol(tokens: TokenStream, kind: GenerateType) -> TokenStream {
    let input: DeriveInput = syn::parse(tokens).expect("Failed to parse input");
    let attrs = Attributes::from_attributes(&input.attrs).expect("Failed to parse attributes");

    // Create a new token stream to hold the output.
    let mut output = TokenStream::new();

    match kind {
        // Generate a `FrogRead` implementation.
        GenerateType::Read => {
            output.extend(generate_impls::generate_read(&input, &attrs));
        }
        // Generate a `FrogWrite` implementation.
        GenerateType::Write => {
            output.extend(generate_impls::generate_write(&input, &attrs));
        }
        // Generate both `FrogRead` and `FrogWrite` implementations.
        GenerateType::ReadWrite => {
            output.extend(generate_impls::generate_read(&input, &attrs));
            output.extend(generate_impls::generate_write(&input, &attrs));
        }
        GenerateType::Tests => {}
    }

    // Generate tests
    if !attrs.tests.is_empty() {
        output.extend(generate_tests::generate_tests(&input, &attrs));
    }

    output
}

/// The type of implementations to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum GenerateType {
    Read,
    Write,
    ReadWrite,
    Tests,
}

/// Attributes for the `frog` attribute macro.
#[derive(Debug, Clone, FromAttr)]
#[attribute(ident = frog)]
#[allow(dead_code)]
struct Attributes {
    /// A list of tests to run.
    #[attribute(optional)]
    tests: Vec<String>,

    /// Example bytes used for test verification.
    #[attribute(optional)]
    bytes: Vec<u8>,

    /// Whether to read/write as a bitset.
    #[attribute(conflicts = [json])]
    bitset: bool,

    /// Whether to read/write as JSON.
    #[attribute(conflicts = [bitset])]
    json: bool,
}

/// Generate a version state implementation and packet enums.
pub(super) fn frog_state(_tokens: TokenStream) -> TokenStream { todo!() }
