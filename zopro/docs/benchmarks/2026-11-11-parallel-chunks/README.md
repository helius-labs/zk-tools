# Parallel chunk prototype

The local `zopro-parallel` prototype combines Rayon's `join` with mutable chunk
processing on the caller's pool. The current implementation sets a minimum
indexed task grain; it creates no collection buffers or per-call thread pools.
It is an independent workspace and has not been integrated into the prover.

## Component measurements

AMD Ryzen 9 9950X, 16 physical cores / 32 logical CPUs, affinity 0–31,
Rust 1.98.1, Rayon 1.12.0 / rayon-core 1.13.0, default Cargo benchmark profile.
Each variant processes 65,536 `u64` values in 256-element chunks. The regular
kernel performs eight multiply/rotate rounds per element; the irregular kernel
performs 64 rounds in every eighth chunk and eight otherwise. This periodic
pattern does not test clustered expensive work or competing requests.

Each version was measured in one fixed-order Criterion campaign: 20 samples,
100 ms warmup and 300 ms measurement per case, plain Rayon before the prototype.
There was no other agent-owned work during capture; the host was shared and
frequency was not locked. These are exploratory component measurements, with
no prospective application adoption gate or claim of statistical significance.

Arithmetic mean estimates from Criterion's `new/estimates.json`, in microseconds:

| Work | Workers | Plain Rayon | Bounded indexed | Bounded / Rayon |
| --- | ---: | ---: | ---: | ---: |
| Regular | 1 | 89.069 | 84.947 | 0.954 |
| Regular | 4 | 25.252 | 24.648 | 0.976 |
| Regular | 16 | 21.379 | 18.493 | 0.865 |
| Regular | 32 | 26.416 | 21.096 | 0.799 |
| Irregular | 1 | 272.229 | 271.339 | 0.997 |
| Irregular | 4 | 70.354 | 69.369 | 0.986 |
| Irregular | 16 | 33.014 | 30.002 | 0.909 |
| Irregular | 32 | 39.442 | 38.524 | 0.977 |

An earlier shared-mutex cursor implementation passed correctness checks but
regressed at multiple workers. Its regular 32-worker mean was 56.295 µs versus
27.354 µs for plain Rayon in that campaign, a ratio of 2.058. That backend was
removed. Its source, binary and results remain available separately; the table
above uses the revised backend's own baseline, not that earlier campaign.

The revised implementation is useful for testing task grain at real call sites.
These measurements establish no full-proof gain, memory-peak improvement,
optimal policy, C7a result, phone performance or advantage over another prover.
No new SDK calls or proof exports were made. All 176 accepted prover source
hashes, including its Cargo files, remain unchanged.

## Validation and retained artifacts

Local Linux checks passed: seven integration tests, two documentation tests,
formatting, Clippy for all targets with warnings denied, benchmark compilation
and Cargo package verification. Tests exercise real 1/2/4/16/32-worker pools,
boundaries, borrowed and Send-only data, nesting, independent callers, panic
joining and pool reuse, and zero-sized values. Ubuntu/macOS CI configuration is
prepared; those remote jobs have not run.

Artifacts are retained locally under
`/home/ubuntu/src/zopro-perf-work/claimed-gather176/`:

- `cursor-v1/`: original source and binary snapshot, pins and `review.json`.
- `benchmark-runs/` and `criterion/`: original command receipt and raw samples.
- `bounded-v2/`: revised source and binary snapshot, pins and `review.json`.
- `benchmark-v2-runs/` and `criterion-v2/`: revised receipt and raw samples.
- `library-target/package/zopro-parallel-0.1.0.crate`: locally verified package.

Source is prepared for local review. Pushes and registry publication require
the user's explicit approval.
