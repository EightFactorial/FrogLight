use std::fmt::Debug;

use attribute_derive::ConvertParsed;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use strum::{Display, EnumString};
use syn::{DeriveInput, LitStr};

use super::macro_type::MacroTypeTrait;
use crate::DeriveMacroAttr;

use self::{
    decode::{DecodeTest, VarDecodeTest},
    encode::{EncodeTest, VarEncodeTest},
    transcode::{TranscodeTest, VarTranscodeTest},
};

mod decode;
mod encode;
mod transcode;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub(crate) struct TestMacro;

impl MacroTypeTrait for TestMacro {
    const REQUIRED_TESTS: &'static [TestType] = &[];

    fn generate_macro(&self, _attr: &DeriveMacroAttr, _input: &syn::DeriveInput) -> TokenStream {
        TokenStream::new()
    }

    fn generate_tests(&self, _attr: &DeriveMacroAttr, _input: &syn::DeriveInput) -> TokenStream {
        TokenStream::new()
    }
}

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum TestType {
    Encode,
    VarEncode,
    Decode,
    VarDecode,
    Transcode,
    VarTranscode,
}

pub(crate) trait TestTrait: Debug {
    fn test_name(&self, ident: &Ident) -> Ident {
        let item_name = ident.to_string().to_case(Case::Snake);
        let test_name = format!("{self:?}").to_case(Case::Snake);

        Ident::new(&format!("{item_name}_auto_{test_name}"), ident.span())
    }

    fn generate_test(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream;
}

impl TestTrait for TestType {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &DeriveInput) -> TokenStream {
        match self {
            TestType::Encode => EncodeTest.generate_test(attr, input),
            TestType::VarEncode => VarEncodeTest.generate_test(attr, input),
            TestType::Decode => DecodeTest.generate_test(attr, input),
            TestType::VarDecode => VarDecodeTest.generate_test(attr, input),
            TestType::Transcode => TranscodeTest.generate_test(attr, input),
            TestType::VarTranscode => VarTranscodeTest.generate_test(attr, input),
        }
    }
}

impl ConvertParsed for TestType {
    type Type = LitStr;

    fn convert(value: Self::Type) -> syn::Result<Self> {
        let str = value.value();

        Ok(str.parse().unwrap_or_else(|_| {
            panic!("Invalid test `{str}` specified");
        }))
    }
}

/// Convert a byte slice to a token stream
fn bytes_to_tokenstream(bytes: &[u8]) -> TokenStream {
    if bytes.is_empty() {
        quote! { Vec::<u8>::new() }
    } else {
        let bytes = bytes.iter().map(|byte| quote!(#byte));

        quote! {
            vec![#(#bytes),*]
        }
    }
}
