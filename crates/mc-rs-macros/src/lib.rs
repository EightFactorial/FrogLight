use attribute_derive::Attribute;
use proc_macro::TokenStream;
use syn::DeriveInput;

mod proto;

use proto::{
    macro_type::{MacroType, MacroTypeTrait},
    test::{TestTrait, TestType},
};

/// Derive `State<V>` for a network state
///
/// This allows the state to be used in a connection for that version.
#[proc_macro]
pub fn impl_state(input: TokenStream) -> TokenStream { proto::state::impl_state(input) }

/// Derive `Encode` for a struct or enum
///
/// This allows the struct or enum to be encoded into a buffer.
#[proc_macro_derive(Encode, attributes(bitset, json, var, mctest))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    DeriveMacroAttr::from_input(input, MacroType::Encode)
}

/// Derive `Decode` for a struct or enum
///
/// This allows the struct or enum to be decoded from a buffer.
#[proc_macro_derive(Decode, attributes(bitset, json, var, mctest))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    DeriveMacroAttr::from_input(input, MacroType::Decode)
}

/// Derive both `Encode` and `Decode` for a struct or enum
///
/// This allows the struct or enum to be encoded into and decoded from a buffer.
#[proc_macro_derive(Transcode, attributes(bitset, json, var, mctest))]
pub fn derive_transcode(input: TokenStream) -> TokenStream {
    DeriveMacroAttr::from_input(input, MacroType::Transcode)
}

/// Derive tests for a struct or enum
///
/// The tests are generated based on the `#[mctest(...)]` attributes.
///
/// These attributes can also be used with the [Transcode], [Encode], and [Decode] macros.
#[proc_macro_derive(Test, attributes(mctest))]
pub fn derive_test(input: TokenStream) -> TokenStream {
    DeriveMacroAttr::from_input(input, MacroType::Test)
}

/// A parser for the `#[mctest(...)]` attribute
#[derive(Debug, Clone, PartialEq, Eq, Attribute)]
#[attribute(ident = mctest)]
struct DeriveMacroAttr {
    /// Tests to generate
    #[attribute(optional)]
    tests: Vec<TestType>,

    /// Example bytes to verify tests
    #[attribute(optional)]
    bytes: Option<Vec<u8>>,
}

impl DeriveMacroAttr {
    fn from_input(input: TokenStream, macro_type: MacroType) -> TokenStream {
        // Parse the input into a DeriveInput
        let input: DeriveInput = syn::parse(input).expect("Unable to parse input");

        // Create a DeriveMacroAttr from the input attributes
        let derive = Self::from_attributes(&input.attrs).unwrap_or_else(|err| {
            panic!(
                "Invalid arguments for `{}` macro, {err}",
                macro_type.to_string().to_lowercase(),
            );
        });

        // Create the output token stream
        let mut output = proc_macro2::TokenStream::new();

        // Generate the macro and tests
        output.extend(macro_type.generate_macro(&derive, &input));
        output.extend(macro_type.generate_tests(&derive, &input));

        for test in derive.tests.iter() {
            output.extend(test.generate_test(&derive, &input));
        }

        output.into()
    }
}
