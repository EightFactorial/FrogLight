//! TODO
#![expect(dead_code, reason = "Compile-time assertion")]

use froglight_mutf8::prelude::*;

const CONST_HELLO: &MStr = mutf8!("Hello, world!");
static STATIC_HELLO: &MStr = mutf8!("Hello, world!");
