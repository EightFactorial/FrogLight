use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DataStruct, Fields, Meta};

use crate::proto::decode::read_fields;

/// Decode a struct
pub(super) fn decode_struct(attrs: Vec<Attribute>, ident: Ident, data: DataStruct) -> TokenStream {
    for attr in attrs {
        if let Meta::Path(path) = attr.meta {
            if path.is_ident("json") {
                return decode_json(ident, data);
            } else if path.is_ident("bitfield") {
                return decode_bitfield(ident, data);
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
        Fields::Unit => panic!("Cannot derive `Decode` for a unit struct"),
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
fn decode_json(ident: Ident, _data: DataStruct) -> TokenStream {
    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                Ok(serde_json::from_str(&String::decode(buf)?)?)
            }
        }
    }
}

/// Decode as a bitfield
fn decode_bitfield(_ident: Ident, _data: DataStruct) -> TokenStream {
    todo!();
}
