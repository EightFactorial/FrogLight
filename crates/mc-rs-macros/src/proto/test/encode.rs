use proc_macro2::TokenStream;
use quote::quote;

use super::TestTrait;
use crate::{proto::test::bytes_to_tokenstream, DeriveMacroAttr};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EncodeTest;

impl TestTrait for EncodeTest {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &syn::DeriveInput) -> TokenStream {
        let ident = &input.ident;
        let fn_name = self.test_name(&input.ident);

        match attr.bytes.is_empty() {
            // Get the default value of the type, encode it to bytes.
            true => {
                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::buffer::FromValue;
                        assert!(Vec::from_value(&#ident::default()).is_ok(), "Failed to encode default value");
                    }
                }
            }
            // Encode the default value of the type to bytes.
            // Then compare the encoded bytes to the expected bytes.
            false => {
                let bytes = bytes_to_tokenstream(&attr.bytes);

                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::buffer::FromValue;
                        use pretty_assertions::assert_eq;

                        assert_eq!(Vec::from_value(&#ident::default()).unwrap(), #bytes, "Default encoded bytes do not match expected bytes");
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct VarEncodeTest;

impl TestTrait for VarEncodeTest {
    fn generate_test(&self, attr: &DeriveMacroAttr, input: &syn::DeriveInput) -> TokenStream {
        let ident = &input.ident;
        let fn_name = self.test_name(&input.ident);

        match attr.bytes.is_empty() {
            // Get the default value of the type, var_encode it to bytes.
            true => {
                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::buffer::FromValue;
                        assert!(Vec::from_var_value(&#ident::default()).is_ok(), "Failed to var-encode default value");
                    }
                }
            }
            // Var_encode the default value of the type to bytes.
            // Then compare the var_encoded bytes to the expected bytes.
            false => {
                let bytes = bytes_to_tokenstream(&attr.bytes);

                quote! {
                    #[test]
                    fn #fn_name() {
                        use crate::buffer::FromValue;
                        use pretty_assertions::assert_eq;

                        assert_eq!(Vec::from_var_value(&#ident::default()).unwrap(), #bytes, "Default var-encoded bytes do not match expected bytes");
                    }
                }
            }
        }
    }
}
