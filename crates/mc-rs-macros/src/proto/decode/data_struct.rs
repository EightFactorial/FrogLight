use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, Meta};

use crate::proto::decode::read_fields;

/// Decode a struct
pub(super) fn decode_struct(input: &DeriveInput) -> TokenStream {
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
                return decode_json(ident, data);
            } else if path.is_ident("bitset") {
                return decode_bitset(ident, data);
            }
        }
    }

    // Get a list of fields
    let mut field_list = Vec::new();
    read_fields(&data.fields, &mut field_list);

    // Generate the decode method
    let decode_method = match &data.fields {
        Fields::Named(_) => {
            quote! {
                Ok(Self {
                    #(#field_list)*
                })
            }
        }
        Fields::Unnamed(_) => {
            quote! {
                Ok(Self(
                    #(#field_list)*
                ))
            }
        }
        Fields::Unit => {
            quote! {
                Ok(Self)
            }
        }
    };

    // Finish the impl
    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                #decode_method
            }
        }

    }
}

/// Decode as a json string
fn decode_json(ident: &Ident, _data: &DataStruct) -> TokenStream {
    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                Ok(serde_json::from_str(&String::decode(buf)?)?)
            }
        }
    }
}

/// Decode as a bitset
fn decode_bitset(ident: &Ident, data: &DataStruct) -> TokenStream {
    let Fields::Named(fields) = &data.fields else {
        panic!("Bitset must be a named struct");
    };
    assert!(
        !fields.named.iter().any(|f| f.ty != syn::parse_quote!(bool)),
        "Struct fields must all be `bool`"
    );

    let field_count = fields.named.len();
    let mut field_list = Vec::new();

    for (i, field) in fields.named.iter().enumerate() {
        let field_name = field.ident.as_ref().unwrap();
        field_list.push(quote! {
            #field_name: bitset.index(#i),
        });
    }

    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                let bitset = crate::types::BitSet::<#field_count>::decode(buf)?;
                Ok(Self {
                    #(#field_list)*
                })
            }
        }
    }
}
