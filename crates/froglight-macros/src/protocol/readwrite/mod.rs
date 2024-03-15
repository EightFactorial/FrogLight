use proc_macro2::TokenStream;
use syn::{DeriveInput, Expr, Ident};

use super::{modifiers, Attributes};

mod read_enum;
mod read_struct;
mod write_enum;
mod write_struct;

pub(super) fn generate_read(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    if attrs.json {
        return modifiers::structs::read_as_json(input, attrs);
    } else if attrs.bitset.is_some() {
        return modifiers::structs::read_as_bitset(input, attrs);
    }

    match input.data {
        syn::Data::Struct(_) => read_struct::read_struct(input, attrs),
        syn::Data::Enum(_) => read_enum::read_enum(input, attrs),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    }
}

pub(super) fn generate_write(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    if attrs.json {
        return modifiers::structs::write_as_json(input, attrs);
    } else if attrs.bitset.is_some() {
        return modifiers::structs::write_as_bitset(input, attrs);
    }

    match input.data {
        syn::Data::Struct(_) => write_struct::write_struct(input, attrs),
        syn::Data::Enum(_) => write_enum::write_enum(input, attrs),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    }
}

/// Set the discriminant for the variant
fn set_discriminant(var_discriminant: &Option<(syn::Token![=], Expr)>, discriminant: &mut i32) {
    if let Some((_, expr)) = var_discriminant {
        match expr {
            // Detect positive discriminants
            Expr::Lit(lit) => {
                if let syn::Lit::Int(int) = &lit.lit {
                    *discriminant = int.base10_parse::<i32>().unwrap();
                }
            }
            // Detect negative discriminants
            Expr::Unary(unary) => {
                if let syn::UnOp::Neg(_) = unary.op {
                    if let Expr::Lit(lit) = &*unary.expr {
                        if let syn::Lit::Int(int) = &lit.lit {
                            *discriminant = -int.base10_parse::<i32>().unwrap();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Check if the field marked `#[frog(var)]`
fn is_variable(attributes: &[syn::Attribute]) -> bool {
    if let Some(attribute) = attributes.iter().find(|a| a.path().is_ident("frog")) {
        if let Ok(arg) = attribute.parse_args::<Ident>() {
            return matches!(arg.to_string().as_str(), "var" | "variant" | "variable");
        }
    }

    false
}
