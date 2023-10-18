use proc_macro2::TokenStream;
use strum::{Display, EnumString};
use syn::DeriveInput;

use crate::DeriveMacroAttr;

use super::{
    decode::DecodeMacro,
    encode::EncodeMacro,
    test::{TestMacro, TestTrait, TestType},
    transcode::TranscodeMacro,
};

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum MacroType {
    Encode,
    Decode,
    Transcode,
    Test,
}

pub(crate) trait MacroTypeTrait {
    /// Tests that are always generated
    const REQUIRED_TESTS: &'static [TestType] = &[];

    fn generate_macro(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream;

    fn generate_tests(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        let mut output = TokenStream::new();

        for test in Self::REQUIRED_TESTS {
            output.extend(test.generate_test(attr, input));
        }

        output
    }
}

impl MacroTypeTrait for MacroType {
    /// Generate the macro for the given type
    fn generate_macro(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        match self {
            MacroType::Encode => EncodeMacro.generate_macro(attr, input),
            MacroType::Decode => DecodeMacro.generate_macro(attr, input),
            MacroType::Transcode => TranscodeMacro.generate_macro(attr, input),
            MacroType::Test => TestMacro.generate_macro(attr, input),
        }
    }

    /// Generate the tests for the given type
    fn generate_tests(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        match self {
            MacroType::Encode => EncodeMacro.generate_tests(attr, input),
            MacroType::Decode => DecodeMacro.generate_tests(attr, input),
            MacroType::Transcode => TranscodeMacro.generate_tests(attr, input),
            MacroType::Test => TestMacro.generate_tests(attr, input),
        }
    }
}
