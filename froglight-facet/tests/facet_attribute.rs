//! TODO

use facet::Facet;
use froglight_facet::{self as mc, facet::WithFnAttr};

#[derive(Facet)]
struct Test {
    #[facet(mc::with = WithFnAttr::new::<u32>())]
    field: u32,
}
