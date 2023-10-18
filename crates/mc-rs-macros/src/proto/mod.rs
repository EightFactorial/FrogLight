use std::ops::Deref;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, ExprUnary, Lit, Token};

pub(super) mod decode;
pub(super) mod encode;
pub(super) mod transcode;

pub(super) mod macro_type;
pub(super) mod test;

pub(super) mod state;

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

#[test]
fn test_discriminant() {
    use proc_macro2::Span;
    use syn::{Expr, ExprLit, ExprUnary, Lit, LitInt, UnOp};

    // Given the following enum:
    // enum ExampleEnum {
    //     A = 1,
    //     B,
    //     C = -1,
    //     D,
    // }

    // A = 1
    let discriminant_a = Some((
        syn::token::Eq::default(),
        Expr::Lit(ExprLit {
            attrs: Vec::new(),
            lit: Lit::Int(LitInt::new("1", Span::call_site())),
        }),
    ));

    // B
    let discriminant_b = None;

    // C = -1
    let discriminant_c = Some((
        syn::token::Eq::default(),
        Expr::Unary(ExprUnary {
            attrs: Vec::new(),
            // Negate the discriminant
            op: UnOp::Neg(syn::token::Minus::default()),
            // Copy discriminant_a
            expr: Box::new(discriminant_a.as_ref().unwrap().1.clone()),
        }),
    ));

    // D
    let discriminant_d = None;

    // Store the discriminant between each iteration
    let mut discriminant = 0;

    // Iterate over each discriminant
    for (index, variant) in [
        discriminant_a,
        discriminant_b,
        discriminant_c,
        discriminant_d,
    ]
    .iter()
    .enumerate()
    {
        // The discriminant is incremented by 1, so subtract 1
        let _ = get_discriminant(variant, &mut discriminant);

        // The discriminant for each variant should be:
        match index {
            // A = 1
            0 => assert_eq!(discriminant - 1, 1),
            // B = 2
            1 => assert_eq!(discriminant - 1, 2),
            // C = -1
            2 => assert_eq!(discriminant - 1, -1),
            // D = 0
            3 => assert_eq!(discriminant - 1, 0),
            _ => panic!("Invalid index {index}"),
        }
    }
}
