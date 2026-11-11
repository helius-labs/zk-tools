# zopro-parallel

An experimental Rust API for mixing task and data parallelism on an existing
Rayon pool. `join` runs independent tasks; `for_each_chunk_mut` processes
disjoint ranges inside a task with a minimum task grain.

The implementation uses safe Rust. It creates no collection buffers, key-derived
plans or per-call thread pools. An indexed Rayon iterator groups chunks into
coarser tasks without adding a shared cursor or synchronization around callbacks.
Callback order is unspecified, and callback panics propagate after outstanding
work joins. Mutations before a panic are retained.

```rust
use zopro_parallel::{for_each_chunk_mut, join};

let mut left = vec![0_u64; 4096];
let mut right = vec![0_u64; 2049];
join(
    || for_each_chunk_mut(&mut left, 256, |start, chunk| {
        for (offset, value) in chunk.iter_mut().enumerate() {
            *value = (start + offset) as u64;
        }
    }),
    || for_each_chunk_mut(&mut right, 256, |start, chunk| {
        for (offset, value) in chunk.iter_mut().enumerate() {
            *value = 2 * (start + offset) as u64;
        }
    }),
);
assert_eq!(left[4095], 4095);
assert_eq!(right[2048], 4096);
```

Both operations use the current Rayon pool. Call them inside an existing
`ThreadPool::install` to use that pool; otherwise Rayon can initialize its global
pool. The caller retains control of thread count, stack size and affinity.

## Choosing a scheduler

Ordinary Rayon indexed iterators recursively split data into tasks. This API
sets the minimum task grain to `ceil(number_of_chunks / (2 * pool_size))`.
It is intended for testing whether fewer scheduling operations help workloads
with many chunks and mixed task/data parallelism. The callback's chunk size
remains unchanged; each task can process several consecutive chunks.
Single-chunk calls execute directly, and one-worker pools iterate sequentially.

Coarser tasks reduce opportunities for load balancing and can be slower when
work is uneven. The benchmarks compare both approaches on regular and uneven
synthetic work. They do not establish a prover speedup, an optimal scheduling
policy or behavior on unmeasured devices. Rayon and callbacks can allocate;
the crate does not promise an allocation-free application.

The API is a small Rayon adapter. Research into
[work assisting](https://doi.org/10.1145/3652586.3663313) motivated investigating
mixed task and data parallelism, but this implementation uses Rayon's existing
scheduler. There is no macro language or persistent execution graph. Broader
APIs depend on measurements from real callers.

## Build and validate

This is an independent workspace alongside `xtask`; its development dependencies
do not enter the prover's dependency graph. From the repository root:

```sh
cargo test --manifest-path zopro/parallel/Cargo.toml --locked
cargo clippy --manifest-path zopro/parallel/Cargo.toml --locked --all-targets -- -D warnings
cargo bench --manifest-path zopro/parallel/Cargo.toml --locked --bench chunks
cargo package --manifest-path zopro/parallel/Cargo.toml --locked
```

Tests cover empty inputs, chunk boundaries, large chunk sizes, Send-only items,
borrowed data, nested tasks, independent callers, panic joining/pool reuse and
zero-sized elements. CI checks functionality and compilation; it imposes no
timing threshold on shared runners. Performance experiments belong on an isolated
host, with complete application measurements after component evaluation.

The crate is prepared for source review. It has not been published to crates.io.
It can be used locally with a path dependency on `zopro/parallel`.

## License

Licensed under either Apache License 2.0 or the MIT license, at your option.
