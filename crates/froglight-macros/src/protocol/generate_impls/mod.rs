use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Expr, Ident, Lit, UnOp};

use super::Attributes;

mod bitset;
mod enums;
mod json;
mod structs;

/// Generate a `FrogRead` implementation.
pub(super) fn generate_read(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    if attrs.bitset {
        assert!(matches!(input.data, Data::Struct(_)), "Bitset items must be structs");
        return bitset::generate_read(input);
    } else if attrs.json {
        assert!(matches!(input.data, Data::Struct(_)), "JSON items must be structs");
        return json::generate_read(input);
    }

    match input.data {
        Data::Struct(_) => structs::generate_read(input),
        Data::Enum(_) => enums::generate_read(input),
        Data::Union(_) => panic!("Unions are not supported"),
    }
}

/// Generate a `FrogWrite` implementation.
pub(super) fn generate_write(input: &DeriveInput, attrs: &Attributes) -> TokenStream {
    if attrs.bitset {
        assert!(matches!(input.data, Data::Struct(_)), "Bitset items must be structs");
        return bitset::generate_write(input);
    } else if attrs.json {
        assert!(matches!(input.data, Data::Struct(_)), "JSON items must be structs");
        return json::generate_write(input);
    }

    match input.data {
        Data::Struct(_) => structs::generate_write(input),
        Data::Enum(_) => enums::generate_write(input),
        Data::Union(_) => panic!("Unions are not supported"),
    }
}

/// Set the discriminant for the variant
fn _set_discriminant(
    variant_discriminant: &Option<(syn::Token![=], Expr)>,
    discriminant: &mut i32,
) {
    if let Some((_, expr)) = variant_discriminant {
        match expr {
            // Detect positive discriminants
            Expr::Lit(lit) => {
                if let syn::Lit::Int(int) = &lit.lit {
                    *discriminant = int.base10_parse::<i32>().unwrap();
                }
            }
            // Detect negative discriminants
            Expr::Unary(unary) => {
                if let UnOp::Neg(_) = unary.op {
                    if let Expr::Lit(lit) = &*unary.expr {
                        if let Lit::Int(int) = &lit.lit {
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
