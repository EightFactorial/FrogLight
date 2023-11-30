use proc_macro2::TokenStream;
use syn::DeriveInput;

use crate::DeriveMacroAttr;

use super::{
    decode::DecodeMacro,
    encode::EncodeMacro,
    macro_type::MacroTypeTrait,
    test::{TestTrait, TestType},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TranscodeMacro;

impl MacroTypeTrait for TranscodeMacro {
    const REQUIRED_TESTS: &'static [TestType] = &[];

    fn generate_macro(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        let mut derives = TokenStream::new();

        derives.extend(EncodeMacro.generate_macro(attr, input));
        derives.extend(DecodeMacro.generate_macro(attr, input));

        derives
    }

    fn generate_tests(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        let mut output = TokenStream::new();

        for test in Self::REQUIRED_TESTS {
            output.extend(test.generate_test(attr, input));
        }

        output.extend(EncodeMacro.generate_tests(attr, input));
        output.extend(DecodeMacro.generate_tests(attr, input));

        output
    }
}
