use std::ops::Deref;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Lit, Meta};

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

/// Decode an enum
fn decode_enum(_attrs: Vec<Attribute>, ident: Ident, data: DataEnum) -> TokenStream {
    // Generate a decode method for each variant
    let mut variants = Vec::new();
    let mut discriminant = 0;

    for variant in data.variants.iter() {
        // Get the discriminant
        let disc = if let Some((_, expr)) = &variant.discriminant {
            match expr {
                Expr::Unary(unary) => {
                    if let Expr::Lit(lit) = unary.expr.deref() {
                        if let Lit::Int(int) = &lit.lit {
                            discriminant = -int
                                .base10_digits()
                                .parse::<i32>()
                                .expect("Unable to parse discriminant");
                        } else {
                            panic!("Invalid discriminant")
                        }
                    } else {
                        panic!("Invalid discriminant")
                    }
                }
                Expr::Lit(lit) => {
                    if let Lit::Int(int) = &lit.lit {
                        discriminant = int
                            .base10_digits()
                            .parse::<i32>()
                            .expect("Unable to parse discriminant");
                    } else {
                        panic!("Invalid discriminant")
                    }
                }
                _ => panic!("Invalid discriminant"),
            }

            quote! { #discriminant }
        } else {
            quote! { #discriminant }
        };
        discriminant += 1;

        let variant_ident = &variant.ident;

        // Get a list of fields
        let mut field_list = Vec::new();
        read_fields(&variant.fields, &mut field_list);

        // Generate the decode method
        let decode_method = match &variant.fields {
            Fields::Named(_) => {
                quote! {
                    Ok(Self::#variant_ident {
                        #(#field_list)*
                    })
                }
            }
            Fields::Unnamed(_) => {
                quote! {
                    Ok(Self::#variant_ident(
                        #(#field_list)*
                    ))
                }
            }
            Fields::Unit => {
                quote! {
                    Ok(Self::#variant_ident)
                }
            }
        };

        variants.push(quote! {
            #disc => #decode_method,
        });
    }

    // Finish the impl
    quote! {
        impl crate::buffer::Decode for #ident {
            fn decode(buf: &mut impl std::io::Read) -> Result<Self, crate::buffer::DecodeError> {
                match crate::buffer::VarDecode::var_decode(buf)? {
                    #(#variants)*
                    id => Err(crate::buffer::DecodeError::InvalidEnumId(id)),
                }
            }
        }
    }
}

fn read_fields(fields: &Fields, field_list: &mut Vec<TokenStream>) {
    match fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let Some(name) = &field.ident else {
                    continue;
                };

                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        #name: crate::buffer::VarDecode::var_decode(buf)?,
                    });
                } else {
                    field_list.push(quote! {
                        #name: crate::buffer::Decode::decode(buf)?,
                    });
                }
            }
        }
        Fields::Unnamed(fields) => {
            for field in fields.unnamed.iter() {
                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        crate::buffer::VarDecode::var_decode(buf)?,
                    });
                } else {
                    field_list.push(quote! {
                        crate::buffer::Decode::decode(buf)?,
                    });
                }
            }
        }
        Fields::Unit => {}
    }
}
