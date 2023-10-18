use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Meta};

use crate::proto::encode::read_fields;

/// Encode a struct
pub(super) fn encode_struct(input: &DeriveInput) -> TokenStream {
    let DeriveInput {
        ident,
        attrs,
        data: Data::Struct(data),
        ..
    } = input
    else {
        panic!("Expected struct");
    };

    for attr in attrs {
        if let Meta::Path(path) = &attr.meta {
            if path.is_ident("json") {
                return encode_json(ident, data);
            } else if path.is_ident("bitset") {
                return encode_bitset(ident, data);
            }
        }
    }

    // Encode each field
    let mut field_list = Vec::new();
    read_fields(&data.fields, &mut field_list);

    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                #(#field_list)*
                Ok(())
            }
        }
    }
}

/// Encode as a json string
fn encode_json(ident: &Ident, _data: &DataStruct) -> TokenStream {
    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                serde_json::to_string(&self)?.encode(buf)?;
                Ok(())
            }
        }
    }
}

/// Encode as a bitset
fn encode_bitset(ident: &Ident, data: &DataStruct) -> TokenStream {
    let Fields::Named(fields) = &data.fields else {
        panic!("Bitset must be a named struct");
    };
    if fields.named.iter().any(|f| f.ty != syn::parse_quote!(bool)) {
        panic!("Struct fields must all be `bool`");
    };

    let field_count = fields.named.len();
    let mut field_list = Vec::new();

    for (i, field) in fields.named.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();
        field_list.push(quote! {
            bitset.set_bit(#i, self.#field_name);
        });
    }

    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                let mut bitset = crate::types::BitSet::<#field_count>::new();
                #(#field_list)*
                crate::buffer::Encode::encode(&bitset, buf)
            }
        }
    }
}
