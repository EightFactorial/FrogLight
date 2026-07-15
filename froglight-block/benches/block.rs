//! TODO
#![allow(clippy::cast_possible_truncation, reason = "Ignored")]

use core::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use froglight_block::prelude::*;
use froglight_common::prelude::*;
use rand::{RngExt, SeedableRng, rngs::Xoshiro128PlusPlus};

fn bench_block(c: &mut Criterion) {
    let mut group = c.benchmark_group("Benchmarks");
    group.throughput(criterion::Throughput::Elements(1));

    let storage = V26_1::blocks();

    let states = generate();
    let blocks: Vec<_> = states.iter().filter_map(|s| storage.get_block_by_state(*s)).collect();
    let names: Vec<_> = blocks.iter().map(Block::identifier).collect();

    group.bench_with_input("GlobalStateId -> Block", &states, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| {
            let next = unsafe { iter.next().unwrap_unchecked() };
            let _block = black_box(storage.get_block_by_state(*next));
        });
    });

    group.bench_with_input("Identifier -> Block", &names, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| {
            let next = unsafe { iter.next().unwrap_unchecked() };
            let _block = black_box(storage.get_block_by_identifier(next));
        });
    });

    group.bench_with_input("Block -> GlobalStateId", &blocks, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| {
            let next = unsafe { iter.next().unwrap_unchecked() };
            let _global = black_box(next.global_id());
        });
    });

    group.bench_with_input("Block -> Identifier", &blocks, |b, input| {
        let mut iter = input.iter().cycle();
        b.iter(|| {
            let next = unsafe { iter.next().unwrap_unchecked() };
            let _ident = black_box(next.identifier());
        });
    });
}

// -------------------------------------------------------------------------------------------------

/// How many random numbers to generate for tests.
const GENERATE_SIZE: usize = 512_000;

/// Generate a set of random [`GlobalStateId`]s using a fixed seed.
fn generate() -> Vec<GlobalStateId> {
    let mut rand = Xoshiro128PlusPlus::seed_from_u64(0x1624_E40C_22FA_A61A);
    let mut input = Vec::with_capacity(GENERATE_SIZE);

    let max_valid = V26_1::blocks().metadata().len() as u32;

    for index in 0..GENERATE_SIZE {
        let random = rand.random::<u32>();

        // Guarantee at least 1/10 are valid.
        if index.is_multiple_of(10) {
            input.push(GlobalStateId::new(random % max_valid));
        } else {
            input.push(GlobalStateId::new(random));
        }
    }

    input
}

criterion_group!(benches, bench_block);
criterion_main!(benches);
