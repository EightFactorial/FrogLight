//! Benchmark for decoding chunks
#![allow(missing_docs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use froglight_protocol::io::{FrogRead, FrogVarRead};

fn decode<T: FrogRead>(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = std::io::Cursor::new(data);
    Ok(T::fg_read(&mut cursor).map_or_else(|err| Err(Box::new(err)), |_| Ok(()))?)
}
fn var_decode<T: FrogVarRead>(data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut cursor = std::io::Cursor::new(data);
    Ok(T::fg_var_read(&mut cursor).map_or_else(|err| Err(Box::new(err)), |_| Ok(()))?)
}

// TODO: Create datasets for benchmarks
// TODO: Use reflection to automatically generate benchmarks for desired types
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("decode_u8", |b| {
        b.iter(|| decode::<u8>(black_box(include_bytes!("data/empty.bin"))));
    });
    c.bench_function("decode_u16", |b| {
        b.iter(|| decode::<u16>(black_box(include_bytes!("data/empty.bin"))));
    });
    c.bench_function("decode_u32", |b| {
        b.iter(|| decode::<u32>(black_box(include_bytes!("data/empty.bin"))));
    });
    c.bench_function("decode_u64", |b| {
        b.iter(|| decode::<u64>(black_box(include_bytes!("data/empty.bin"))));
    });
    c.bench_function("decode_u128", |b| {
        b.iter(|| decode::<u128>(black_box(include_bytes!("data/empty.bin"))));
    });

    c.bench_function("var_decode_u16", |b| {
        b.iter(|| var_decode::<u16>(black_box(include_bytes!("data/empty.bin"))));
    });
    c.bench_function("var_decode_u32", |b| {
        b.iter(|| var_decode::<u32>(black_box(include_bytes!("data/empty.bin"))));
    });
    c.bench_function("var_decode_u64", |b| {
        b.iter(|| var_decode::<u64>(black_box(include_bytes!("data/empty.bin"))));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
