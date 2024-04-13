use attribute_derive::FromAttr;
use proc_macro::TokenStream;
use syn::DeriveInput;

use crate::manifest::ProjectManifest;

mod generate_impls;
mod generate_state;
mod generate_tests;

/// Get the path to the `froglight_protocol` crate.
///
/// Used for generating `FrogRead` and `FrogWrite` implementations with the
/// correct path to the traits.
pub(crate) fn get_protocol_path() -> syn::Path {
    let mut path = ProjectManifest::get().get_path("froglight_protocol");

    if let Some(segment) = path.segments.first() {
        // If the path is `froglight` or `froglight_app`,
        // remove an extra `protocol` segment.
        //
        // This is done because only the `froglight_protocol::protocol`
        // module is re-exported
        if segment.ident == "froglight" || segment.ident == "froglight_app" {
            let segments = path.segments.into_iter().enumerate().filter(|(i, _)| i != &1);
            path.segments = segments.map(|(_, s)| s).collect();
        }
    }

    path
}

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
        GenerateType::Test => {}
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
    Test,
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

    /// Whether to read/write as a bitset.
    #[attribute(conflicts = [json])]
    bitset: bool,

    /// Whether to read/write as JSON.
    #[attribute(conflicts = [bitset])]
    json: bool,
}

/// Generate a version state implementation and packet enums.
pub(super) fn frog_state(tokens: TokenStream) -> TokenStream {
    generate_state::generate_state(tokens).into()
}
