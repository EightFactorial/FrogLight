use proc_macro2::TokenStream;
use quote::quote;

use super::TestTrait;
use crate::{proto::test::bytes_to_tokenstream, DeriveMacroAttr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TranscodeTest;

impl TestTrait for TranscodeTest {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &syn::DeriveInput) -> TokenStream {
        let ident = &input.ident;
        let fn_name = self.test_name(&input.ident);

        match attr.bytes.is_empty() {
            true => quote! {
                #[test]
                fn #fn_name() {
                    use crate::buffer::{FromValue, Decode};
                    let default = #ident::default();

                    let encoded = Vec::from_value(&default).unwrap();
                    let decoded = #ident::decode(&mut encoded.as_slice()).unwrap();
                    assert_eq!(default, decoded, "Decoded value does not match default encoded value");
                }
            },
            false => {
                let bytes = bytes_to_tokenstream(&attr.bytes);

                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::{buffer::{FromValue, Decode}, types::wrappers::UnsizedByteBuffer};

                        let mut bytes = UnsizedByteBuffer::from_vec(#bytes);
                        let decoded = #ident::decode(&mut bytes).unwrap();
                        assert_eq!(Vec::from_value(&decoded).unwrap(), #bytes, "Decoded bytes do not match encoded bytes");
                        assert_eq!(bytes.len(), 0, "Not all bytes were consumed");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VarTranscodeTest;

impl TestTrait for VarTranscodeTest {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &syn::DeriveInput) -> TokenStream {
        let ident = &input.ident;
        let fn_name = self.test_name(&input.ident);

        match attr.bytes.is_empty() {
            true => quote! {
                #[test]
                fn #fn_name() {
                    use crate::buffer::{FromValue, VarDecode};
                    let default = #ident::default();

                    let encoded = Vec::from_var_value(&default).unwrap();
                    let decoded = #ident::var_decode(&mut encoded.as_slice()).unwrap();
                    assert_eq!(default, decoded, "Decoded value does not match default encoded value");
                }
            },
            false => {
                let bytes = bytes_to_tokenstream(&attr.bytes);

                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::{buffer::{FromValue, VarDecode}, types::wrappers::UnsizedByteBuffer};

                        let mut bytes = UnsizedByteBuffer::from_vec(#bytes);
                        let decoded = #ident::var_decode(&mut bytes).unwrap();
                        assert_eq!(Vec::from_var_value(&decoded).unwrap(), #bytes, "Decoded bytes do not match encoded bytes");
                        assert_eq!(bytes.len(), 0, "Not all bytes were consumed");
                    }
                }
            }
        }
    }
}
