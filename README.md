# zopro

A Groth16 BN254 prover in Rust for gnark circuits. One kernel for browsers
(WebAssembly, SIMD lanes, WebGPU), phones and laptops (native, AArch64
assembly, Metal or wgpu): the host (gnark) builds the witness, zopro solves
the circuit with its own solver plan, runs the MSMs and the quotient on the
CPU or the GPU, and the host assembles and verifies the proof.

## Layout

- `crates/zopro-core`: the field arithmetic (`ark-bn254`, vendored: 29-bit
  SIMD lanes on wasm, assembly on AArch64, arkworks' 64-bit code elsewhere),
  the signed-window MSM, the FFT plan, the witness solver and its image, the
  `Key` a host builds from gnark's proving key, and the proof assembly
  (`groth16`). `--features metal` or `--features wgpu` moves the MSMs and the
  quotient to the GPU; the G2 MSM is split between the GPU and the CPU, which
  run at once (`ZOPRO_G2_GPU_SHARE`; defaults: 0.8 on a Mac, a quarter on
  an A19 phone, none on older ones, 0.25 natively and 0.75 in a browser
  through wgpu). In a browser `init_gpu()` requests WebGPU and
  `Key::solve_parts_gpu` returns a Promise of the parts.
- `crates/zopro-metal`: the Metal driver. The host recodes and sorts the
  scalars into bucket runs; the GPU sums each run in shards (witness scalars
  are skewed, so a few buckets hold most points), reduces each window (each
  thread scales its block of buckets by the block's offset, threadgroups
  add), and runs the radix-2 transforms; the host combines. Shaders compile
  to metallibs at build time when the Metal toolchain is installed (else at
  first use), once per process; a key's MSMs share their scratch buffers and
  one command queue, and every command buffer's status is checked.
- `crates/zopro-wgpu`: the same design in WGSL for Vulkan, Metal and WebGPU,
  with async reads for the browser. The field and curve code is aeglos's
  (Apache-2.0), its multiplication rewritten as a loop of sixteen rounds.
- `crates/zopro-bench`: proves a dumped shape natively and times it
  (`RAYON_NUM_THREADS=4 zopro-bench <dump dir>`), and has modes for the GPU
  MSMs (`gpu`, `wgpu`), a key-switching stress check of results and memory
  (`stress <cycles> <dumps...>`), and field microbenchmarks (`field`,
  `wgsl-field`). Dumps and the proof referee are `TestZoproDump` in the
  web SDK's `wasm/prover-wasm`.

## Safety

A GPU can compute wrong results (two iPhones' did, from a threadgroup size
their pipelines could not run). At key load zopro compares the GPU's MSM on
small inputs with the CPU's, and a GPU that disagrees is let go (Metal first
retries with size-optimized shaders); the hosts verify every proof before it
leaves and, on a failure, prove again on the CPU (the browser) or with gnark
(mobile). A kernel or buffer the driver refuses, a lost device or an aborted
command buffer is an error, never a panic. On phones a key is proved with
zopro only when the free memory holds its peak (about nine times the key's
point vectors).

## Tests

`cargo test --workspace` (CPU), plus `--features zopro-core/metal` and
`--features zopro-core/wgpu`: the GPU crates check their MSMs against
arkworks (1 to 20 000 points, random, witness-like and zero scalars), and
zopro-core checks a GPU key's proof parts against a CPU key's byte for
byte. GPU tests skip without a GPU.

## Measured

Proof with the key loaded, 4 CPU threads, every proof verified by gnark.

| shape | gnark (M5 Pro) | zopro, Metal (M5 Pro) | web runtime 0.3.1 (Chrome, M5 Pro) | zopro, WebGPU (Chrome, M5 Pro) |
|---|---|---|---|---|
| 1 → 2 | — | 51 ms | 171 ms | 61 ms |
| 2 → 2 | 287 ms | 67 ms | 280 ms | 89 ms |
| 8 → 2 | 920 ms | 149 ms | 920 ms | 212 ms |
| 24 → 2 | 2 472 ms | 361–381 ms | 2 744 ms | 574 ms |

Phones (AWS Device Farm, release builds, against gnark on the same phone):
iPhone 17 Pro 1.7–2.25×, iPhone 15 1.45–1.7×, iPhone 12 1.3–1.5×; Android
proves on its CPU at gnark's speed (its GPUs, through wgpu, were slower than
its cores or refused a kernel).

## Building

Native: `cargo build --release`. Browser: `cargo rustc --release --target
wasm32-unknown-unknown --crate-type cdylib --features wgpu` in
`crates/zopro-core` with `nightly-2025-11-15`, then `wasm-bindgen --target
web` (0.2.128); the crate must build as a cdylib alone, since its SIMD
assembly assembles with `simd128` only in the crate that links. The web SDK's
`scripts/build-runtime-zopro.sh <zopro checkout>` does both.
