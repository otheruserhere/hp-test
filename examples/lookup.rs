//! `lookup` benchmark: binary-searches a sorted table of key/value pairs.
//!
//! Run with:
//!   cargo run --release --example lookup --features hotpath-alloc

use std::hint::black_box;

const ENTRIES: usize = 50_000;
const LOOKUPS: usize = 200_000;
const SEED: u64 = 7;

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// Even keys only, so roughly half of the lookups miss.
fn table() -> Vec<(u64, u64)> {
    (0..ENTRIES as u64).map(|i| (i * 2, i * i)).collect()
}

#[hotpath::measure]
fn find(table: &[(u64, u64)], key: u64) -> Option<u64> {
    table
        .binary_search_by_key(&key, |&(k, _)| k)
        .ok()
        .map(|index| table[index].1)
}

#[hotpath::main]
fn main() {
    let table = table();
    let mut lcg = Lcg(SEED);
    let max_key = ENTRIES as u64 * 2;
    let mut hits = 0usize;
    for _ in 0..LOOKUPS {
        if find(black_box(&table), lcg.next() % max_key).is_some() {
            hits += 1;
        }
    }
    black_box(hits);
}
