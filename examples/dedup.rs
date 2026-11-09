//! `dedup` benchmark: removes duplicates from a fixed list of numbers,
//! keeping the first occurrence of each value.
//!
//! Run with:
//!   cargo run --release --example dedup --features hotpath-alloc

use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::hint::black_box;

const LEN: usize = 20_000;
const DISTINCT: u64 = 5_000;
const ROUNDS: usize = 10;
const SEED: u64 = 42;

/// Fixed-key hasher: the default `RandomState` is seeded per process, which
/// would make the work (and the instruction counts) differ between runs.
type FixedState = BuildHasherDefault<DefaultHasher>;

fn input() -> Vec<u64> {
    let mut state = SEED;
    (0..LEN)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) % DISTINCT
        })
        .collect()
}

#[hotpath::measure]
fn dedup(values: &[u64]) -> Vec<u64> {
    let mut seen: HashSet<u64, FixedState> = HashSet::default();
    let mut unique = Vec::new();
    for &value in values {
        if seen.insert(value) {
            unique.push(value);
        }
    }
    unique
}

#[hotpath::main]
fn main() {
    let values = input();
    for _ in 0..ROUNDS {
        black_box(dedup(black_box(&values)));
    }
}
