# zopro

**Experimental.** zopro is new and unaudited: its field arithmetic, its
solver and its GPU kernels were checked against gnark and arkworks on the
devices we have, and every proof is verified before it is returned, but
nothing here has had an independent review. Do not rely on it where a
wrong or leaked proof would cost anything.

A Groth16 BN254 prover in Rust for gnark circuits. One kernel for browsers
(WebAssembly, SIMD lanes, WebGPU), phones and laptops (native, AArch64
assembly, Metal or wgpu): given a circuit's inputs by gnark's names, zopro
solves the circuit with its own solver plan, runs the MSMs and the quotient
on the CPU or the GPU, and verifies the proof before it returns it, in
gnark's encodings. gnark stays offline: a zopro key is built once from a
gnark constraint system and its keys (`tools/gnark`), and gnark's verifier
accepts zopro's proofs.

Circuits the kernel proves alone: BN254, no commitments (so not gnark's
range-check API), and no hints but gnark's `InvZeroHint` (`IsZero`,
`Inverse`) and `bits.nBits` (`ToBinary`). A key of any other circuit fails
to build, not to prove.

```sh
tools/gnark/pack.sh circuit.r1cs circuit.pk circuit.vk my_circuit my_circuit.zopro
```

```rust
let mut prover = zopro_prover::Prover::load(&std::fs::read("my_circuit.zopro")?)?;
let mut inputs = zopro_prover::Assignment::default();
inputs.set("X".into(), Fr::from(3u64));
inputs.set("Y".into(), Fr::from(35u64));
let proved = prover.prove(&prover.witness(&inputs)?)?;
let (proof, public_witness) = (proved.proof.to_gnark(), proved.public_witness());
```

## Layout

- `crates/zopro-bn254`: BN254 without arkworks: Fq and Fr in Montgomery
  form (assembly on AArch64, 29-bit SIMD-friendly limbs on wasm, 128-bit
  products elsewhere), the tower to Fq12, G1 and G2 with the point at
  infinity as (0, 0), and the pairing that verifies proofs. Its tests check
  every operation against arkworks, which is only a dev-dependency.
- `crates/zopro-core`: the signed-window MSM, the FFT plan, the witness
  solver and its image, the `Key` a host builds from gnark's proving key,
  and the proof assembly (`groth16`). `--features metal` or `--features
  wgpu` moves the MSMs and the quotient to the GPU; the G2 MSM is split
  between the GPU and the CPU, which run at once (`ZOPRO_G2_GPU_SHARE`;
  defaults: none on a Mac, a quarter on an A19 phone, none on older ones,
  0.25 natively through wgpu and 2.2 / threads in a browser). In a
  browser `init_gpu()` requests WebGPU and `Key::solve_parts_gpu` returns a
  Promise of the parts.
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
- `crates/zopro-prover`: the prover. A zopro key file (`artifact`: gnark's
  vectors compressed, the solver program, the input names, the verifying
  key and the application's metadata; the fixed points are checked against
  the verifying key) loads into a `Prover`, which proves an `Assignment` of
  the inputs by name and verifies. `raw` is the form a host caches in place
  of the key file (vectors decompressed, the solver's image). In a browser,
  `WebProver` takes the inputs as an object of decimal or hex strings and
  returns gnark's compressed proof and public witness.
- `crates/zopro-zolana`: Zolana's proof requests on the prover: the SDKs'
  `/prove` JSON onto the transfer and merge circuits' inputs, the prover
  server's proof JSON, and the key metadata (shape, the gnark key's sha256
  that proofs report). Its cdylib is the Zolana browser runtime
  (`ZolanaProver` beside `WebProver`).
- `crates/zopro-bench`: proves a dumped shape natively and times it
  (`RAYON_NUM_THREADS=4 zopro-bench <dump dir>`), and has modes for the GPU
  MSMs (`gpu`, `wgpu`), a key-switching stress check of results and memory
  (`stress <cycles> <dumps...>`), and field microbenchmarks (`field`,
  `wgsl-field`).
- `tools/gnark`: gnark's side, offline, with gnark alone: `zopro-dump`
  writes a constraint system's and its keys' parts (`zoprognark.Dump` for
  Go callers), and `pack.sh` makes the zopro key.
- `tools/zolana`: Zolana's keys: the dump of a Zolana gnark key with its
  metadata, the zopro keys of a whole key set with their lockfile
  (`build-keys.sh`), and gnark's verifier as the referee of zopro's proofs.

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
zopro-bn254's reference MSM (1 to 20 000 points, random, witness-like and
zero scalars), and
zopro-core checks a GPU key's proof parts against a CPU key's byte for
byte. GPU tests skip without a GPU. `zopro-prover`'s `tests/cubic.rs` proves a small
gnark circuit end to end from a committed dump (`tools/gnark`'s test writes
it with `ZOPRO_CUBIC`). With `ZOPRO_DUMP` (dumps from `tools/zolana`), the
Zolana tests prove the 2 → 2 transfer and the 24-note merge, check the
witness against gnark's byte for byte, and leave raw proofs that
`tools/zolana`'s test verifies with gnark.

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

Native: `cargo build --release`. Browser: `CARGO_PROFILE_RELEASE_LTO=false
cargo rustc --release --target wasm32-unknown-unknown --crate-type cdylib
--features wgpu` in `crates/zopro-prover` (or `crates/zopro-zolana` for
Zolana's runtime) with `nightly-2025-11-15`, then
`wasm-bindgen --target web` (0.2.128). Without LTO: zopro-core's SIMD
assembly does not assemble with `simd128` in an rlib built for LTO. The web
SDK's `scripts/build-runtime.sh` does both.
