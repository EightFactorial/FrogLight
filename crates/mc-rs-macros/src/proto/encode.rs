use std::ops::Deref;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr, Fields, Index, Lit, Meta};

/// Derive `Encode`
pub fn derive_encode(input: proc_macro::TokenStream) -> TokenStream {
    let DeriveInput {
        attrs, ident, data, ..
    } = syn::parse(input).expect("Unable to DeriveInput");

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

/// Encode an enum
fn encode_enum(_attrs: Vec<Attribute>, ident: Ident, data: DataEnum) -> TokenStream {
    // Encode each variant
    let mut variant_list = Vec::new();
    let mut discriminant = 0;

    for variant in data.variants.iter() {
        let ident = &variant.ident;

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

        // Encode each field
        let mut field_list = Vec::new();
        read_fields(&variant.fields, &mut field_list);

        variant_list.push(quote! {
            Self::#ident => {
                crate::buffer::VarEncode::var_encode(&#disc, buf)?;
                #(#field_list)*
            }
        });
    }

    // Finish the impl
    quote! {
        impl crate::buffer::Encode for #ident {
            fn encode(&self, buf: &mut impl std::io::Write) -> Result<(), crate::buffer::EncodeError> {
                match self {
                    #(#variant_list,)*
                }
                Ok(())
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
                        crate::buffer::VarEncode::var_encode(&self.#name, buf)?;
                    });
                } else {
                    field_list.push(quote! {
                        crate::buffer::Encode::encode(&self.#name, buf)?;
                    });
                }
            }
        }
        Fields::Unnamed(fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let index = Index::from(i);

                if field.attrs.iter().any(|f| {
                    if let Meta::Path(path) = &f.meta {
                        path.is_ident("var")
                    } else {
                        false
                    }
                }) {
                    field_list.push(quote! {
                        crate::buffer::VarEncode::var_encode(&self.#index, buf)?;
                    });
                } else {
                    field_list.push(quote! {
                        crate::buffer::Encode::encode(&self.#index, buf)?;
                    });
                }
            }
        }
        Fields::Unit => {}
    }
}
