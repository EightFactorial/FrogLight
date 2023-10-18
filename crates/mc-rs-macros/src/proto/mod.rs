use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, ExprUnary, Lit, Token};

pub(crate) mod decode;
pub(crate) mod encode;
pub(crate) mod transcode;

pub(crate) mod macro_type;
pub(crate) mod test;

pub(crate) mod state;

/// Get the discriminant for an enum variant.
/// If no discriminant is specified, the stored discriminant is returned and incremented by 1.
fn get_discriminant(expr: &Option<(Token![=], Expr)>, discriminant: &mut i32) -> TokenStream {
    let disc = if let Some((_, expr)) = &expr {
        match expr {
            Expr::Unary(ExprUnary { expr, .. }) => {
                let Expr::Lit(ExprLit {
                    lit: Lit::Int(int), ..
                }) = expr.deref()
                else {
                    panic!("Invalid discriminant");
                };

                *discriminant = -int
                    .base10_digits()
                    .parse::<i32>()
                    .expect("Unable to parse discriminant");
            }
            Expr::Lit(ExprLit {
                lit: Lit::Int(int), ..
            }) => {
                *discriminant = int
                    .base10_digits()
                    .parse::<i32>()
                    .expect("Unable to parse discriminant");
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
