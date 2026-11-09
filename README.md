# hotpath-demo

A small demo repository for reviewing the **hotpath-cloud Claude connector**
(the hotpath-cloud MCP server). It has a real benchmark history: baselines on
`main`, uploaded by CI, and open pull requests whose reports are compared
against them.

Every push to `main` and every pull request runs both benchmarks in hotpath's
merged mode (one native run for timing and allocations, then one run under
Valgrind's Callgrind for instruction counts) and uploads one report per
benchmark to [hotpath.rs](https://hotpath.rs). Each pull request gets a
hotpath-cloud comment and a `hotpath / <benchmark>` check per benchmark.

## Benchmarks

Both are deterministic: their inputs come from a hand-written LCG with a fixed
seed, so two runs of the same commit do the same work.

| Benchmark | Example | What it measures |
|-----------|---------|------------------|
| `dedup` | `examples/dedup.rs` | `dedup` removes duplicates from 20,000 `u64` values (5,000 distinct), keeping first-seen order, 10 times. Uses a `HashSet` with a fixed-key hasher so instruction counts don't depend on a random seed. |
| `lookup` | `examples/lookup.rs` | `find` runs 200,000 `binary_search_by_key` lookups in a sorted table of 50,000 key/value pairs. A second series; no pull request changes it. |

Policies: `hotpath/dedup-policy.toml` and `hotpath/lookup-policy.toml`.
Instruction counts are judged and fail the check (`fail_check = true`),
allocations are judged, and timing is shown but not judged.

Run locally (no hotpath features means no profiling, so the macros compile
to nothing):

```sh
cargo run --release --example dedup --features hotpath-alloc
cargo run --release --example lookup --features hotpath-alloc
```

## Open pull requests

| Branch | Change | Expected result |
|--------|--------|-----------------|
| `regression/dedup-vec-contains` | `dedup` tracks seen values with `Vec::contains` instead of a `HashSet` (quadratic). | `dedup` instruction counts and timing regress sharply; verdict regressed, check fails. Allocations drop, since no hash set is built. |
| `improvement/dedup-with-capacity` | `dedup` pre-sizes the `HashSet` and the output `Vec` with the number of distinct values. | Allocations and instruction counts of `dedup` improve (no regrowth or rehashing). |
| `noop/readme-tweak` | README only. | No findings; verdict passed. |

`lookup` is unchanged in every pull request, so its reports should pass with
no findings.
