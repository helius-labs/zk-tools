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
  form (assembly on AArch64 and BMI2/ADX x86, AVX-512 IFMA field batches,
  29-bit SIMD-friendly limbs on wasm, 128-bit products elsewhere), the tower
  to Fq12, G1 and G2 with the point at
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
  (`RAYON_NUM_THREADS=4 zopro-bench <dump dir>`), proves a Zolana request
  with its key as the mobile SDK does and reports times and memory
  (`zolana <key.zopro> <request.json>`; `tools/bench.sh` runs it shape by
  shape), and has modes for the GPU MSMs (`gpu`, `wgpu`), a key-switching
  stress check of results and memory (`stress <cycles> <dumps...>`), and
  field microbenchmarks (`field`).
- `tools/gnark`: gnark's side, offline, with gnark alone: `zopro-dump`
  writes a constraint system's and its keys' parts (`zoprognark.Dump` for
  Go callers), and `pack.sh` makes the zopro key.
- `tools/zolana`: Zolana's keys: the dump of a Zolana gnark key with its
  metadata, the zopro keys of a whole key set with their lockfile
  (`build-keys.sh`; `zopro-keys.lock` pins the key set the bench uses),
  request fixtures per shape (`fixtures/`), and gnark's verifier as the
  referee of zopro's proofs.
- `tools/bench.sh [shapes] [rounds]`: benches this computer on Zolana's
  keys (downloaded once into `~/.cache/zopro-keys`, checked against the
  lockfile) and requests, 1 → 2 to 24 → 2 by default: a key's first load,
  its later loads, the memory it holds, and each proof's time and peak.
- `xtask`: prepares pinned candidate snapshots, runs build and test plans,
  and compares SDK latency with automatic warm CPU profiles and independent
  proof verification. See the [CPU experiment workflow](xtask/README.md).

On x86, `tools/bench.sh` builds for the local CPU (`-C target-cpu=native`)
unless `RUSTFLAGS` is already set. The native build enables BMI2/ADX and,
where supported, AVX-512 IFMA. To build another executable the same way:

```sh
RUSTFLAGS="-C target-cpu=native" cargo build --release -p zopro-bench
```

Deploy that binary only to compatible CPUs, or build for the deployment CPU.
For a Genoa/C7a deployment, use `RUSTFLAGS="-C target-cpu=znver4"`; that
build also runs on the Framework AI 300. Measure on the deployment host
with its allocated physical cores, including the enclave allocation.
Build without CPU-specific flags for the portable backend. CPU builds need
no GPU libraries. Native IFMA MSMs group equal scalars afresh for every
proof; the gain varies with the witness. `ZOPRO_CPU_DEDUP=0` disables that
grouping. `ZOPRO_BENCH_PROOFS=directory` saves each benchmark proof and its
public witness for independent gnark verification. On IFMA builds with at
least 16 workers, small MSMs split their points into two ranges and merge
the buckets before weighting. `ZOPRO_CPU_MERGE_SPLIT=1` keeps one range.
Final windows with at most 256 buckets use a parallel affine tree instead
of the scalar collision fallback; `ZOPRO_CPU_TOP_TREE=0` disables it.
Large IFMA MSMs use width-13 bucket tables to reduce cache pressure;
`ZOPRO_CPU_CACHE_WIDTH=0` restores automatic width selection, and
`ZOPRO_CPU_LARGE_WIDTH=10..15` explicitly selects it for large MSMs.
Streaming IFMA batches round up to the SIMD lane count so a full batch
has no scalar tail; `ZOPRO_CPU_ALIGN_BATCH=0` restores the original sizes.
Native eight-lane FFTs pair A/B, then A*B/C, and subtract C's coefficients
after the final inverse transform, saving C's forward coset transform.
`ZOPRO_CPU_QUOTIENT8=0` selects four lanes; also set
`ZOPRO_CPU_QUOTIENT6=0` to restore the original seven-transform quotient.
For domains of at least 2^19 elements, paired FFTs run four wide stages
per cache tile. `ZOPRO_CPU_FFT_STAGES=1..4` selects the stage count;
`ZOPRO_CPU_FFT2=0` also restores separate stages.
Portable, AArch64 and WebAssembly defaults keep their existing FFT path.
Native plans also intern constant coefficients before sharing expression
prefixes, reducing key memory and temporary sum slots. Existing solver
images remain readable and their coefficient tables compact at load;
`ZOPRO_CPU_COMPACT_COEFFS=0` disables this preparation pass. Other targets
keep their existing preparation defaults.
Native solvers combine consecutive coefficient products with eight IFMA
lanes, and pairs for shorter runs. Stored prefixes retain their original
values and dependencies. `ZOPRO_CPU_FUSE_SUMS=0` restores scalar products;
`=1` uses pairs only. Other targets keep scalar products by default.

## Safety

A GPU can compute wrong results (two iPhones' did, from a threadgroup size
their pipelines could not run). At key load zopro compares the GPU's MSM on
small inputs with the CPU's, and a GPU that disagrees is let go (Metal first
retries with size-optimized shaders); the hosts verify every proof before it
leaves and, on a GPU's failure, prove again on the CPU. A kernel or buffer
the driver refuses, a lost device or an aborted command buffer is an error,
never a panic.

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

The latest adopted AVX CPU results, including all eight supplied M5 CPU and
Metal comparisons, source hashes and validation, are in the
[G2 vector-sign adoption report](docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/README.md).

The earlier x86 CPU checkpoint, including comparison with M5 Pro Metal,
measurement details, and reproducible commands, is in the
[Framework CPU report](docs/benchmarks/2026-10-20/README.md).

`tools/bench.sh` on an M5 Pro (Metal, every proof verified; memory is the
process's physical footprint, Metal's buffers included):

| shape | first load | load | key held | proof peak | proof |
|---|---|---|---|---|---|
| 1 → 2 | 141 ms | 79 ms | 350 MB | 391 MB | 24 ms |
| 2 → 2 | 215 ms | 89 ms | 397 MB | 462 MB | 31 ms |
| 4 → 2 | 372 ms | 93 ms | 484 MB | 605 MB | 53 ms |
| 8 → 2 | 724 ms | 110 ms | 645 MB | 893 MB | 68 ms |
| 12 → 2 | 1188 ms | 135 ms | 812 MB | 1176 MB | 99 ms |
| 16 → 2 | 1528 ms | 151 ms | 969 MB | 1362 MB | 125 ms |
| 24 → 2 | 2355 ms | 190 ms | 1273 MB | 1862 MB | 180 ms |

On a phone's CPU (a Seeker, Android, anonymous memory) 12 → 2 proves in
2.45 s with a 194 MB peak, against gnark's 4.79 s and 790 MB. Phones in AWS
Device Farm (release builds, against gnark on the same phone): proofs 23 to
49 % faster on Android's CPUs (Galaxy A16, A56), 21 to 58 % on a Galaxy
S25, and 43 to 76 % on iPhones 12, 15 and 17 Pro with Metal.

## Building

Native: `cargo build --release`. Browser: `CARGO_PROFILE_RELEASE_LTO=false
cargo rustc --release --target wasm32-unknown-unknown --crate-type cdylib
--features wgpu` in `crates/zopro-prover` (or `crates/zopro-zolana` for
Zolana's runtime) with `nightly-2025-11-15`, then
`wasm-bindgen --target web` (0.2.128). Without LTO: zopro-core's SIMD
assembly does not assemble with `simd128` in an rlib built for LTO. The web
SDK's `scripts/build-runtime.sh` does both.
