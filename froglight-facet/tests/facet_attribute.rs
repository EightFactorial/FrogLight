//! TODO

use facet::Facet;
use froglight_facet::{self as mc, facet::WithFnAttr};

#[derive(Facet)]
struct Test {
    #[facet(mc::with = WithFnAttr::using(|_| todo!(), |_| todo!()))]
    field: u32,
}
