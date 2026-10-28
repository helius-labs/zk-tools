# Zolana's zopro keys

Zolana's gnark keys for zopro, from zolana's prover at the release the keys
come from (`./fetch-zolana.sh` checks it out into `zolana/`, which `go.mod`
builds against; its aeglos GPU files need a module this does not fetch, so
`go mod tidy -e`).

- `TestZoproDump` writes a Zolana gnark key's dump with its metadata (shape,
  and the sha256 of the key file, which proofs report)
  (`ZOPRO_DUMP=<dir> ZOPRO_KEYS=<gnark keys> ZOPRO_SHAPES=2x2,merge-24x1 go
  test -run TestZoproDump .`), with `ZOPRO_REQUESTS=<requests>` the
  request's witness too, and gnark's verdict on a raw proof zopro left in a
  shape's dump (`zopro-zolana`'s `tests/prove.rs` leaves one).
- `build-keys.sh <gnark proving-keys.lock> <out>` builds the zopro key of
  every confidential transfer and merge key of a key set and the
  `zopro-keys.lock` that pins them, beside the gnark key each was built
  from. The SDKs copy the lockfile and serve the keys from the key set's
  CDN folder.
- `fixtures/` holds a `/prove` request per shape (`transfer-<n>x2.json`,
  `merge-<n>x1.json`, written by zolana's `TestRequestFixture` with the
  shape changed) and `zopro-keys.lock` the key set they prove with; both
  feed `tools/bench.sh`.

## CPU basis images

Native little-endian hosts can load `zopro-basis-v1` and `zopro-basis-v2` through
`Prover::load_image`. This format replaces the raw image's K/Z queries with
folded private and product queries. Keep the original raw image for GPU and
browser use. Preparing an image is an offline operation; proving maps the
prepared output without retaining the original queries or preparation cache.

On a little-endian Linux preparation host:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --release -p zopro-bench --example cpu-basis
target/release/examples/cpu-basis prepare input.image prepared.image
```

The command accepts a trusted `zopro-raw-3` or `zopro-raw-4` image, refuses to replace an
existing output, and removes its own temporary files on ordinary errors.
It uses one file-backed projective array (96 bytes per domain element),
a temporary query cache (128 bytes per element plus its header), and a
bounded point tile, defaulting to 4,096 points. The OS can reclaim scratch
pages, but this is not a fixed total RAM bound: conversion loads one solver
plan and the worker pool also uses memory. Use a preparation host with
sufficient disk space and RAM; distribute the resulting image to clients.

To retain and reuse the intermediate cache, run the two stages separately:

```sh
target/release/examples/cpu-basis cache input.image queries.bin
target/release/examples/cpu-basis image input.image queries.bin prepared.image
```

Conversion validates the cache fingerprint and payload hash against the
source image. The cache is not needed during proving. An optional final
argument sets the point tile size, from 1 to 65,536. Preparation currently
supports little-endian Linux; the original raw-image APIs remain available
on all previously supported platforms.

New basis and Euler images use version 2 with 64-bit header and solver byte
lengths. Their readers retain version-1 compatibility.

Linux hosts that use `zopro_core::alloc::Allocator` as their global allocator
also release large proof buffers promptly when workspace forecasts exceed
available host/cgroup memory. This policy does not guarantee a proof can fit
an arbitrary memory limit. Other operating systems retain their existing
allocation policy. See the recorded [CPU qualification results](../../docs/benchmarks/2026-10-25-cpu-checkpoint/README.md#validation-and-limits)
for the tested circuit sizes and limits.


## Sparse Poseidon CPU evaluation

When resource-aware scheduling is compiled (`--cfg zopro_resource_schedule`),
AVX-512 IFMA CPU builds enable exact sparse Poseidon evaluation by default.
`ZOPRO_CPU_SPARSE_POSEIDON=0` disables it. Recognition uses the existing solver
tasks and checks coefficient identities; it does not change the circuit, key
image or proof format. Unmatched layouts and denied preparation use the generic
solver.

Preparation charges 2 MiB of temporary analysis space and discards it afterward.
Fixed public parameters occupy 432,384 bytes; no second circuit plan or retained
per-key lookup table is added. The all-width row-equivalence fixtures can be
regenerated with `python3 scripts/poseidon_fixtures.py`.
