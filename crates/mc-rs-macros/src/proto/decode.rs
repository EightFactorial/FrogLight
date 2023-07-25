use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Meta};

/// Derive `Decode`
pub fn derive_decode(input: proc_macro::TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse(input).unwrap();

    match data {
        Data::Struct(data) => decode_struct(attrs, ident, data),
        Data::Enum(data) => decode_enum(attrs, ident, data),
        Data::Union(_) => panic!("Cannot derive `Decode` for a union"),
    }
}

/// Decode a struct
fn decode_struct(attrs: Vec<Attribute>, ident: Ident, data: DataStruct) -> TokenStream {
    for attr in attrs {
        if let Meta::Path(path) = attr.meta {
            if path.is_ident("json") {
                return decode_json(ident, data);
            } else if path.is_ident("bitfield") {
                return decode_bitfield(ident, data);
            }
        }
    }

    // Decode each field
    let mut fields = Vec::new();
    for field in data.fields.iter() {
        let name = field.ident.as_ref().unwrap();

        fields.push(quote! {
            #name: crate::buffer::Decode::decode(buf)?,
        });
    }

    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                Ok(Self {
                    #(#fields)*
                })
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

/// Decode an enum
fn decode_enum(_attrs: Vec<Attribute>, _ident: Ident, _data: DataEnum) -> TokenStream {
    todo!();
}
