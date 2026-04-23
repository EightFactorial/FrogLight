//! TODO
#![no_std]

use facet::Facet;
use froglight_facet::{self as mc, facet::WithFnAttr};

#[derive(Facet)]
struct Test {
    #[facet(mc::with = WithFnAttr::using(|_, _| todo!(), |_, _| todo!()))]
    field: u32,
}
