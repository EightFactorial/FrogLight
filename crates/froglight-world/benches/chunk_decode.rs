//! Benchmark for decoding chunks
#![allow(missing_docs)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use froglight_world::world::{section::Section, Chunk};

fn decode_chunk<const HEIGHT: usize>(data: &[u8]) -> Result<(), Box<dyn std::error::Error>>
where
    [(); HEIGHT / Section::HEIGHT]:,
{
    let mut cursor = std::io::Cursor::new(data);

    Ok(Chunk::<HEIGHT>::read_from_buffer(&mut cursor)
        .map_or_else(|err| Err(Box::new(err)), |_| Ok(()))?)
}

// TODO: Dump packets to file for benchmarking
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("chunk_1 256", |b| {
        b.iter(|| decode_chunk::<256>(black_box(include_bytes!("data/chunk1_256.bin"))));
    });
    c.bench_function("chunk_2 256", |b| {
        b.iter(|| decode_chunk::<256>(black_box(include_bytes!("data/chunk2_256.bin"))));
    });

    c.bench_function("chunk_1 384", |b| {
        b.iter(|| decode_chunk::<512>(black_box(include_bytes!("data/chunk1_384.bin"))));
    });
    c.bench_function("chunk_2 384", |b| {
        b.iter(|| decode_chunk::<512>(black_box(include_bytes!("data/chunk2_384.bin"))));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
