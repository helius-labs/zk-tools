# Framework CPU performance checkpoint

The validated CPU implementation proves the SDK's seven transfer shapes in
31–286 ms on a Framework Ryzen AI 9 HX 370. It is 3.1–6.0 times faster than
the initial Linux measurements and 2.9–5.8 times faster than the same-machine
gnark comparison. The M5 Pro Metal target remains ahead by 19–62%; this
checkpoint does not establish Metal parity.

## Measurements

CPU only, 24 Rayon threads, five fresh proofs per shape, two outputs.
Proof times are medians in milliseconds. Downloads and key preparation are
outside the proof timer. Every proof is verified before it is returned.

| Shape | Initial Linux CPU | gnark CPU | Optimized CPU | M5 Pro CPU | M5 Pro Metal |
|---|---:|---:|---:|---:|---:|
| 1x2 | 96 | 90 | 31 | 37 | 24 |
| 2x2 | 173 | 148 | 40 | 59 | 30 |
| 4x2 | 313 | 268 | 57 | 109 | 48 |
| 8x2 | 539 | 491 | 92 | 189 | 74 |
| 12x2 | 815 | 781 | 142 | 284 | 101 |
| 16x2 | 941 | 908 | 157 | 335 | 124 |
| 24x2 | 1475 | 1516 | 286 | 528 | 176 |

The initial Linux and M5 measurements were supplied by the user. The gnark
comparison runs on this Framework with gnark v0.16.3 and gnark-crypto v0.21.0,
the versions used by zk-tools main, using the same SHA-pinned setup as zopro.
Its Zolana checkout was main at `bbdecee0161cc02057ab7840d68127709082f79a`.

Memory is the benchmark process's own memory in MB: anonymous resident
memory on Linux. It excludes file-backed mapped key pages and is not total
RSS. First load creates a private solver/key image; later load maps it.

| Shape | First load ms | First load peak MB | Load ms | Key held MB | Proof peak MB | Witness ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 209 | 22 | 25 | 16 | 80 | 0 |
| 2x2 | 372 | 29 | 25 | 19 | 128 | 0 |
| 4x2 | 658 | 54 | 48 | 34 | 186 | 0 |
| 8x2 | 1312 | 108 | 52 | 66 | 285 | 0 |
| 12x2 | 2054 | 173 | 72 | 107 | 397 | 0 |
| 16x2 | 2571 | 196 | 80 | 128 | 466 | 0 |
| 24x2 | 4288 | 346 | 119 | 211 | 725 | 1 |

Repeated runs vary. In a separate comparison with saved stage-12 and
stage-13 binaries, reversing their order between repetitions, the 24x2
medians were 279/273 ms before solver fusion and 268/266 ms after it.
The full-run table retains its measured 286 ms result. This laptop mixes
four Zen 5 cores and eight Zen 5c cores with SMT; sustained all-core work
also reaches its thermal limit. No C7a or TEE measurements have been made.

## Implementation

- BMI2/ADX field arithmetic and four/eight-lane AVX-512 IFMA arithmetic,
  plus a faster binary-GCD field inverse.
- Streaming affine G1/G2 batches, grouping equal witness scalars afresh
  per proof, cache-sized large MSM windows, two-way small MSM splitting,
  and parallel trees for thin final windows.
- Six-transform quotient construction, paired eight-lane FFTs, fused
  cache tiles, and reusable in-place FFT buffers.
- Interned solver coefficients and shared prefixes, with eight-product
  IFMA sums and paired tails. Existing solver images remain readable.
- Optional GPU dependencies for CPU builds, native CPU flags in the local
  benchmark, accurate process-memory sampling, and proof export for an
  independent gnark verifier.

Architecture-specific defaults leave portable, AArch64, and WebAssembly
paths available. The escape settings are documented in the main README.
No witness or proof results are cached between proofs.

## Validation and reproduction

The measured source was checked against `validated-stage13.patch`, SHA-256
`fb7aa1d7e81e9bb3ceb6ce3c7cac788dfcc589027bf366c73d512193444aa7c6`,
relative to `41fbf26f1f1749a5b9c668e21eb3540882cbb769`. It was built with
rustc 1.99.1 / LLVM 23.1.1 and `-C target-cpu=native`. The measured executable
SHA-256 is recorded in [results.json](results.json), together with all proof
samples, key/proof hashes, and verification summaries.

- All 35 fresh benchmark proofs independently passed gnark verification,
  with byte-exact public-witness equality and distinct proof hashes.
- Another 35 proofs from legacy solver images independently passed gnark.
- Native field tests: 18; native core tests: 27 plus three integration tests.
- Portable field tests: eight; portable core tests: 26 plus three integration
  tests, including the forced fused-sum fallback.
- AArch64 Android and WebAssembly SIMD cross-target checks passed.
- A `znver4` build completed; that is a build check, not a C7a measurement.
- Arithmetic tests compare against arkworks; FFT polynomial-reference tests
  check the quotient against scalar convolution across transform schedules.

From the zopro directory, reproduce the native benchmark with the pinned
SDK fixtures and keys:

```sh
RAYON_NUM_THREADS=24 ZOPRO_BENCH_PROOFS=/tmp/zopro-proofs \
  tools/bench.sh 1x2,2x2,4x2,8x2,12x2,16x2,24x2 5
```

The lockfile pins `proving-keys/3a1c88f90f609eb1`. Use the corresponding
gnark raw keys for independent verification; another setup with the same
shape is insufficient. `ZOPRO_BENCH_PROOFS` writes each gnark-format proof
and public witness after its timer stops. CPU-only validation commands:

```sh
RUSTFLAGS='-C target-cpu=native' cargo test --release \
  -p zopro-bn254 -p zopro-core -p zopro-prover -p zopro-zolana
RUSTFLAGS='' cargo test --release \
  -p zopro-bn254 -p zopro-core -p zopro-prover -p zopro-zolana
```

After rebasing this checkpoint onto `sergei/zopro` at `4c0f88f`, the release
suites passed all 59 native and 48 portable tests (one doctest is ignored
in each), and the ARM/wasm checks passed again. See
[validation.json](validation.json). The newer base adds the Apple allocator
and Metal memory changes; performance was not retimed after the rebase.

## Work remaining

The optimization goal is paused at this checkpoint. Large transfers,
24/54-input merges, and nullifier-tree batches were being prepared; they
have no new measured results yet. M5 data currently includes merge-24 at
175 ms on Metal and 507 ms on CPU, but no supplied M5 merge-54 or large
nullifier-batch baseline.

Experimental fixed-point bucket sets with extra 2P/3P tables, alternate
recoding layouts, lazy FFT reductions, and parallel FFT output collection
have not demonstrated a consistent full-proof improvement. Their code is
kept in the local experiment worktree, outside this validated checkpoint.
The next untested ideas include prepacking recurring solver coefficient
vectors and table-free SIMD scalar recoding. C7a/TEE work awaits an actual
test host and an explicit user instruction to resume.
