use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// A simple test name generator
fn test_name(ident: &Ident, test_name: &str) -> Ident {
    Ident::new(
        &format!("{}_{}_test", ident.to_string().to_lowercase(), test_name),
        ident.span(),
    )
}

/// A test that checks that the value can be encoded and decoded again
pub(super) fn transcode_test(ident: &Ident) -> TokenStream {
    let fn_name = test_name(ident, "transcode");

    let mut tests = TokenStream::new();

    tests.extend(encode_test(ident));

    tests.extend(quote! {
        #[test]
        fn #fn_name() {
            use crate::buffer::{FromValue, Encode, Decode};

            let default = #ident::default();

            let encoded = Vec::from_value(&default).expect("Failed to encode value");
            let decoded = #ident::decode(&mut &encoded[..]).expect("Failed to decode value");

            assert_eq!(default, decoded);
        }
    });

    tests
}

/// A test that checks that the value can be encoded
///
/// This **does NOT** check that the encoded value is correct
pub(super) fn encode_test(ident: &Ident) -> TokenStream {
    let fn_name = test_name(ident, "encode");

    quote! {
        #[test]
        fn #fn_name() {
            use crate::buffer::{Encode, FromValue};

            assert!(Vec::from_value(&#ident::default()).is_ok());
        }
    }
}

/// A test that checks that the value can be var_encoded and var_decoded again
pub(super) fn var_transcode_test(ident: &Ident) -> TokenStream {
    let fn_name = test_name(ident, "var_transcode");

    let mut tests = TokenStream::new();

    tests.extend(var_encode_test(ident));

    tests.extend(quote! {
        #[test]
        fn #fn_name() {
            use crate::buffer::{FromValue, VarEncode, VarDecode};

            let default = #ident::default();

            let encoded = Vec::from_var_value(&default).expect("Failed to encode value");
            let decoded = #ident::var_decode(&mut &encoded[..]).expect("Failed to decode value");

            assert_eq!(default, decoded);
        }
    });

    tests
}

/// A test that checks that the value can be var_encoded
///
/// This **does NOT** check that the encoded value is correct
pub(super) fn var_encode_test(ident: &Ident) -> TokenStream {
    let fn_name = test_name(ident, "var_encode");

    quote! {
        #[test]
        fn #fn_name() {
            use crate::buffer::{VarEncode, FromValue};

            assert!(Vec::from_var_value(&#ident::default()).is_ok());
        }
    }
}
