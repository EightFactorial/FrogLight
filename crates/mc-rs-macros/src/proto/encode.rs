use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Meta};

/// Derive `Encode`
pub fn derive_encode(input: proc_macro::TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse(input).unwrap();

    match data {
        Data::Struct(data) => encode_struct(attrs, ident, data),
        Data::Enum(data) => encode_enum(attrs, ident, data),
        Data::Union(_) => panic!("Cannot derive `Encode` for a union"),
    }
}

/// Encode a struct
fn encode_struct(attrs: Vec<Attribute>, ident: Ident, data: DataStruct) -> TokenStream {
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
    let mut fields = Vec::new();
    for field in data.fields.iter() {
        let name = field.ident.as_ref().unwrap();

        if field.attrs.iter().any(|f| {
            if let Meta::Path(path) = &f.meta {
                path.is_ident("var")
            } else {
                false
            }
        }) {
            fields.push(quote! {
                crate::buffer::VarEncode::var_encode(&self.#name, buf)?;
            });
        } else {
            fields.push(quote! {
                crate::buffer::Encode::encode(&self.#name, buf)?;
            });
        }
    }

    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                #(#fields)*
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

/// Encode an enum
fn encode_enum(_attrs: Vec<Attribute>, _ident: Ident, _data: DataEnum) -> TokenStream {
    todo!();
}
