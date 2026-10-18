# ringil

A Groth16 BN254 prover in Rust for gnark circuits, proving from key images:
the kernel's point vectors and solver plan as a host keeps them, with nothing
decoded or rebuilt at load. One crate for the browser (WebAssembly, SIMD
lanes) and for phones and laptops (native, AArch64 assembly), with the MSMs
and the quotient's transforms on Apple GPUs through Metal; Vulkan and WebGPU
to come.

## Layout

- `crates/ringil-core`: the field arithmetic (`ark-bn254`, vendored: 29-bit
  SIMD lanes on wasm, assembly on AArch64, arkworks' 64-bit code elsewhere),
  the signed-window MSM, the FFT plan, the witness solver and its image, the
  `Key` a host builds from gnark's proving key, and the proof assembly
  (`groth16`). `--features metal` moves the MSMs and the quotient to the GPU.
- `crates/ringil-metal`: the Metal driver. The host recodes and sorts the
  scalars into bucket runs; the GPU sums each run in shards (witness scalars
  are skewed, so a few buckets hold most points), reduces each window, and
  runs the radix-2 transforms; the host combines. The field and curve shaders
  are aeglos's (Apache-2.0).
- `crates/ringil-bench`: proves a dumped shape natively and times its stages
  (`RAYON_NUM_THREADS=4 ringil-bench <dump dir>`; `--features
  ringil-core/trace` prints each stage; `ringil-bench gpu <dump dir>` times the
  GPU MSMs alone; `ringil-bench field` the field multiply). The dumps and the
  proof referee are `TestRingilDump` in the web SDK's `wasm/prover-wasm`;
  every proof is verified by gnark.

## Measured

Apple M5 Pro, 4 CPU threads (phone-like) plus the GPU, proof with the key
loaded, verified by gnark:

| shape | gnark native | ringil CPU | ringil CPU + GPU |
|---|---|---|---|
| 2 → 2 | 287 ms | 217 ms | 88 ms |
| 8 → 2 | 920 ms | 731 ms | 209 ms |
| 24 → 2 | 2 472 ms | 2 150 ms | 423 ms |

## Building

Native: `cargo build --release`. Browser: `cargo rustc --release --target
wasm32-unknown-unknown --crate-type cdylib` in `crates/ringil-core` with
`nightly-2025-11-15`, then `wasm-bindgen --target web` (0.2.128); the crate
must build as a cdylib alone, since its SIMD assembly assembles with
`simd128` only in the crate that links. With Apple's Metal toolchain
installed, `ringil-metal` compiles its shaders to a metallib at build time;
without it they compile from source at first use (the G2 kernels take about
a minute, once per machine).
