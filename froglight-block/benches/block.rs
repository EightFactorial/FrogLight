//! TODO

use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use froglight_block::prelude::*;
use froglight_common::prelude::*;

fn bench_block(c: &mut Criterion) {
    let storage = V26_1::blocks();
    let max_state = storage.metadata().len() as u32;

    c.bench_function("GlobalBlockId -> Block", |b| {
        b.iter_batched(
            || {
                let mut blocks = Vec::with_capacity(1024);
                for _ in 0..1024 {
                    let id = rand::random_range(0..max_state);
                    let block = GlobalStateId::new(id);
                    blocks.push(block);
                }
                blocks
            },
            |blocks| {
                for block in blocks {
                    black_box(storage.get_block_by_state(block).unwrap());
                }
            },
            BatchSize::LargeInput,
        );
    });

    c.bench_function("Block -> GlobalBlockId", |b| {
        b.iter_batched(
            || {
                let mut blocks = Vec::with_capacity(1024);
                for _ in 0..1024 {
                    let id = rand::random_range(0..max_state);
                    let block = storage.get_block_by_state(GlobalStateId::new(id)).unwrap();
                    blocks.push(block);
                }
                blocks
            },
            |blocks| {
                for block in blocks {
                    black_box(block.global_id());
                }
            },
            BatchSize::LargeInput,
        );
    });
}

criterion_group!(benches, bench_block);
criterion_main!(benches);
