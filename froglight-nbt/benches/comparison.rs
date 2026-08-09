//! TODO
#![allow(clippy::match_bool, reason = "Readability")]

extern crate alloc;

use core::hint::black_box;
use std::io::Cursor;

use criterion::Criterion;

fn main() {
    let mut c = Criterion::default().configure_from_args();

    complex_player(&mut c);

    c.final_summary();
}

// -------------------------------------------------------------------------------------------------

macro_rules! bench {
    ( $group:ident = $path:literal : $($name:ident => $fn:path),* ) => {
        fn $group(c: &mut Criterion) {
            static INPUT: &'static [u8] = include_bytes!($path);

            let mut group = c.benchmark_group(stringify!($group));
            group.throughput(criterion::Throughput::Elements(1));

            $(
                group.bench_with_input(stringify!($name), INPUT, |b, input| {
                    b.iter(|| black_box($fn(input).unwrap()));
                });
            )*
        }
    };
}

bench!(
    complex_player = "../tests/nbt/complex_player.nbt":
    froglight_nbt => froglight_nbt::prelude::IndexedNbtSlice::new_named,
    simdnbt => simdnbt_read
);

fn simdnbt_read(input: &[u8]) -> Result<simdnbt::borrow::Nbt<'_>, simdnbt::Error> {
    simdnbt::borrow::read(&mut Cursor::new(input))
}
