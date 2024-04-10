use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use super::Attributes;

pub(super) fn read_example(input: &DeriveInput, test_attrs: &Attributes, output: &mut TokenStream) {
    let crate_path = crate::protocol::get_protocol_path();

    let test_name = super::test_name(input, "read_example");
    let item = input.ident.clone();
    let item_name = input.ident.to_string();
    let bytes = &test_attrs.bytes;

    output.extend(quote! {
        #[test]
        fn #test_name() {
            let data: Vec<u8> = vec![#(#bytes),*];
            let mut cursor = std::io::Cursor::new(data.as_slice());

            if let Err(err) = <#item as #crate_path::protocol::FrogRead>::fg_read(&mut cursor) {
                panic!("Failed to read `{}`: {err}", #item_name)
            }
            assert_eq!(cursor.position() as usize, data.len());
        }
    });
}

pub(super) fn read_default(input: &DeriveInput, test_attrs: &Attributes, output: &mut TokenStream) {
    let crate_path = crate::protocol::get_protocol_path();

    let test_name = super::test_name(input, "read_verify");
    let item = input.ident.clone();
    let item_name = input.ident.to_string();
    let bytes = &test_attrs.bytes;

    output.extend(quote! {
        #[test]
        fn #test_name() {
            let data: Vec<u8> = vec![#(#bytes),*];
            let mut cursor = std::io::Cursor::new(data.as_slice());

            match <#item as #crate_path::protocol::FrogRead>::fg_read(&mut cursor) {
                Err(err) => panic!("Failed to read `{}`: {err}", #item_name),
                Ok(value) => assert_eq!(value, #item::default()),
            }
            assert_eq!(cursor.position() as usize, data.len());
        }
    });
}
