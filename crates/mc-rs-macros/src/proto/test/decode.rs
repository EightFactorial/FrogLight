use proc_macro2::TokenStream;
use quote::quote;

use super::TestTrait;
use crate::{proto::test::bytes_to_tokenstream, DeriveMacroAttr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct DecodeTest;

impl TestTrait for DecodeTest {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &syn::DeriveInput) -> TokenStream {
        assert!(
            attr.bytes.is_some(),
            "Decode tests require `bytes` to be set"
        );

        let ident = &input.ident;
        let fn_name = self.test_name(&input.ident);
        let bytes = bytes_to_tokenstream(attr.bytes.as_ref().unwrap());

        // Decode the bytes into the type.
        quote! {
            #[test]
            fn #fn_name() {
                use crate::{buffer::Decode, types::UnsizedByteBuffer};
                use pretty_assertions::assert_eq;

                let mut bytes = UnsizedByteBuffer::from_vec(#bytes);
                assert!(#ident::decode(&mut bytes).is_ok(), "Failed to decode bytes");
                assert_eq!(bytes.len(), 0, "Not all bytes were consumed");
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct VarDecodeTest;

impl TestTrait for VarDecodeTest {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &syn::DeriveInput) -> TokenStream {
        assert!(
            attr.bytes.is_some(),
            "Decode tests require `bytes` to be set"
        );

        let ident = &input.ident;
        let fn_name = self.test_name(&input.ident);
        let bytes = bytes_to_tokenstream(attr.bytes.as_ref().unwrap());

        // Var_decode the bytes into the type.
        quote! {
            #[test]
            fn #fn_name() {
                use crate::{buffer::VarDecode, types::UnsizedByteBuffer};
                use pretty_assertions::assert_eq;

                let mut bytes = UnsizedByteBuffer::from_vec(#bytes);
                assert!(#ident::var_decode(&mut bytes).is_ok(), "Failed to var-decode bytes");
                assert_eq!(bytes.len(), 0, "Not all bytes were consumed");
            }
        }
    }
}
