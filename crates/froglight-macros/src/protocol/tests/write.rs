use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use super::Attributes;

pub(super) fn write_default(input: &DeriveInput, _: &Attributes, output: &mut TokenStream) {
    let test_name = super::test_name(input, "write_default");
    let item_name = input.ident.to_string();

    output.extend(quote! {
        #[test]
        fn #test_name() {
            let mut data = Vec::new();
            if let Err(err) = <Self as crate::io::FrogWrite>::fg_write(&Self::default(), &mut data) {
                panic!("Failed to write `{}`: {err}", #item_name);
            }
        }
    });
}

pub(super) fn write_example(
    input: &DeriveInput,
    test_attrs: &Attributes,
    output: &mut TokenStream,
) {
    let test_name = super::test_name(input, "write_verify");
    let item = input.ident.clone();
    let item_name = input.ident.to_string();
    let bytes = &test_attrs.bytes;

    output.extend(quote! {
        #[test]
        fn #test_name() {
            let mut data = Vec::new();
            if let Err(err) = <#item as crate::io::FrogWrite>::fg_write(&#item::default(), &mut data) {
                panic!("Failed to write `{}`: {err}", #item_name);
            }

            assert_eq!(data, vec![#(#bytes),*]);
        }
    });
}
