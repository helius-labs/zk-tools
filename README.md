# ringil

A Groth16 BN254 prover in Rust for gnark circuits, proving from key images:
the kernel's point vectors and solver plan as a host keeps them, with nothing
decoded or rebuilt at load. One crate for the browser (WebAssembly, SIMD
lanes) and for phones and laptops (native, AArch64 assembly), with the MSMs
and the quotient's transforms on the GPU: Apple's through Metal, others
through wgpu (Vulkan today; WebGPU to come).

## Layout

- `crates/ringil-core`: the field arithmetic (`ark-bn254`, vendored: 29-bit
  SIMD lanes on wasm, assembly on AArch64, arkworks' 64-bit code elsewhere),
  the signed-window MSM, the FFT plan, the witness solver and its image, the
  `Key` a host builds from gnark's proving key, and the proof assembly
  (`groth16`). `--features metal` moves the MSMs and the quotient to the GPU
  through Metal, `--features wgpu` through wgpu (the G2 MSM stays on the
  CPU there, and on a phone with Metal too: `RINGIL_G2=gpu|cpu` overrides).
  At key load the key hands each vector to the GPU and keeps no copy, and
  checks every unit against another implementation (`RINGIL_TRACE selftest`
  lines): a GPU whose MSM disagrees is let go and the key decoded again for
  the CPU, a quotient that disagrees stays on the CPU. `RINGIL_GPU=0` keeps
  everything on the CPU, `RINGIL_GPU_QUOTIENT=0` only the quotient.
- `crates/ringil-metal`: the Metal driver. The host recodes and sorts the
  scalars into bucket runs; the GPU sums each run in shards (witness scalars
  are skewed, so a few buckets hold most points), reduces each window, and
  runs the radix-2 transforms; the host combines. The field and curve shaders
  are aeglos's (Apache-2.0). The library compiles once per process; the G2
  kernels only when a G2 MSM asks (they take minutes on a phone).
- `crates/ringil-wgpu`: the same design in WGSL for wgpu. aeglos's field
  arithmetic in 16-bit halves, with the multiply rewritten as a loop of
  sixteen rounds: the unrolled 2 700-line form took a Galaxy S25's Adreno 16 s
  to compile, a Galaxy A16's Mali minutes, and a Galaxy A56's Xclipse 196 s,
  and the loop is faster even on the M5 Pro (B 126→69 ms). Kernels that fail
  to build and buffers that do not fit are errors, not a lost device.
- `crates/ringil-bench`: proves a dumped shape natively and times its stages
  (`RAYON_NUM_THREADS=4 ringil-bench <dump dir>`; `--features
  ringil-core/trace` prints each stage, and `RINGIL_TRACE_MSMS=1` each CPU
  MSM alone after the proof; `ringil-bench gpu <dump dir>` times the Metal
  MSMs alone, `ringil-bench wgpu <dump dir>` the wgpu ones, `ringil-bench
  field` the field multiply). The dumps and the
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

With the G2 MSM on the CPU (a phone's default) the CPU G2 is the critical
path: Metal 8 → 2 336 ms, 24 → 2 905 ms; wgpu on the same GPU 400 / 1 010 ms.

Phones, in AWS Device Farm, release builds, the proof with the key loaded
(gnark in brackets): iPhone 17 Pro, Metal with the G2 MSM on the CPU, 1 → 2
0.11 s (0.20), 8 → 2 0.53 s (1.17), 24 → 2 1.49 s (3.57) — every proof
verified; the shaders compile from source in about 6 s once per process
(9–11 s on an iPhone 12 or 15), which a precompiled metallib would remove.

## Building

Native: `cargo build --release`. Browser: `cargo rustc --release --target
wasm32-unknown-unknown --crate-type cdylib` in `crates/ringil-core` with
`nightly-2025-11-15`, then `wasm-bindgen --target web` (0.2.128); the crate
must build as a cdylib alone, since its SIMD assembly assembles with
`simd128` only in the crate that links. With Apple's Metal toolchain
installed, `ringil-metal` compiles its shaders to a metallib at build time;
without it they compile from source at first use (the G2 kernels take about
a minute, once per machine).
