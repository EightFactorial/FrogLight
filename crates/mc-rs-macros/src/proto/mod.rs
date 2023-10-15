use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, Lit, Token};

mod encode;
pub(crate) use encode::derive_encode;

mod decode;
pub(crate) use decode::derive_decode;

mod transcode;
pub(crate) use transcode::derive_transcode;

mod state;
pub(crate) use state::impl_state;

mod tests;
pub(crate) use tests::generate_tests;

/// Get the discriminant for an enum variant.
/// If no discriminant is specified, the stored discriminant is returned and incremented by 1.
fn get_discriminant(expr: &Option<(Token![=], Expr)>, discriminant: &mut i32) -> TokenStream {
    let disc = if let Some((_, expr)) = &expr {
        match expr {
            Expr::Unary(unary) => {
                if let Expr::Lit(lit) = unary.expr.deref() {
                    if let Lit::Int(int) = &lit.lit {
                        *discriminant = -int
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
                    *discriminant = int
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

    *discriminant += 1;

    disc
}
