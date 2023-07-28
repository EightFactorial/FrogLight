use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DataStruct, Meta};

use crate::proto::encode::read_fields;

/// Encode a struct
pub(super) fn encode_struct(attrs: Vec<Attribute>, ident: Ident, data: DataStruct) -> TokenStream {
    for attr in attrs {
        if let Meta::Path(path) = attr.meta {
            if path.is_ident("json") {
                return encode_json(ident, data);
            } else if path.is_ident("bitfield") {
                return encode_bitfield(ident, data);
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
fn encode_json(ident: Ident, _data: DataStruct) -> TokenStream {
    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                serde_json::to_string(&self)?.encode(buf)?;
                Ok(())
            }
        }
    }
}

/// Encode as a bitfield
fn encode_bitfield(_ident: Ident, _data: DataStruct) -> TokenStream {
    todo!();
}
