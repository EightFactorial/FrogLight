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

        match &attr.bytes {
            // Get the default value of the type,
            // encode it to bytes, then decode it back to the type.
            // Then compare the decoded value to the default value.
            None => quote! {
                #[test]
                fn #fn_name() {
                    use crate::buffer::{FromValue, Decode};
                    use pretty_assertions::assert_eq;

                    let default = #ident::default();
                    let encoded = Vec::from_value(&default).unwrap();
                    let decoded = #ident::decode(&mut encoded.as_slice()).unwrap();
                    assert_eq!(default, decoded, "Decoded value does not match default encoded value");
                }
            },
            // Decode the bytes into the type, then encode the type back to bytes.
            // Then compare the encoded bytes to the expected bytes.
            Some(bytes) => {
                let bytes = bytes_to_tokenstream(bytes);

                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::{buffer::{FromValue, Decode}, types::UnsizedByteBuffer};
                        use pretty_assertions::assert_eq;

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

        match &attr.bytes {
            // Get the default value of the type,
            // var_encode it to bytes, then var_decode it back to the type.
            // Then compare the var_decoded value to the default value.
            None => quote! {
                #[test]
                fn #fn_name() {
                    use crate::buffer::{FromValue, VarDecode};
                    use pretty_assertions::assert_eq;

                    let default = #ident::default();
                    let encoded = Vec::from_var_value(&default).unwrap();
                    let decoded = #ident::var_decode(&mut encoded.as_slice()).unwrap();
                    assert_eq!(default, decoded, "Decoded value does not match default encoded value");
                }
            },
            // Var_decode the bytes into the type, then var_encode the type back to bytes.
            // Then compare the var_encoded bytes to the expected bytes.
            Some(bytes) => {
                let bytes = bytes_to_tokenstream(bytes);

                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::{buffer::{FromValue, VarDecode}, types::UnsizedByteBuffer};
                        use pretty_assertions::assert_eq;

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
