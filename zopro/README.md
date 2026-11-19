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
grouping. With grouping enabled, `ZOPRO_CPU_GROUP_DIVERSITY=1` checks
eligible exclusive G2 queries before allocating the grouping map. It uses
up to 32 KiB of optional per-proof scratch to count distinct occupied bits.
If that count already exceeds the grouping cutoff, the prover skips the map.
Under memory pressure or when the count is inconclusive, the ordinary
grouping policy applies. `ZOPRO_BENCH_PROOFS=directory` saves each benchmark proof and its
public witness for independent gnark verification. On IFMA builds with at
least 16 workers, small MSMs split their points into two ranges and merge
the buckets before weighting. `ZOPRO_CPU_MERGE_SPLIT=1` keeps one range.
On Linux x86-64, servers can pass a caller-owned Rayon pool to
`zopro_prover::Prover::set_frontend_pool(Some(pool.clone()))`. The native IFMA
canonical path uses this pool for witness solving, query input preparation
and the product FFT when it is smaller than the main Rayon pool. The same
`Arc<rayon::ThreadPool>` can serve multiple loaded keys. Passing `None`
restores the main pool. Single-proof latency and concurrent throughput can
favor different pool sizes.

Linux x86-64 servers can call `Prover::set_overlap_verification(true)` on a
loaded prover to compute the public-input pairing during CPU proof generation.
This setting uses the existing Rayon pool and applies to keys with at most
sixteen public inputs. Each request computes its own pairing and verifies the
finished proof. The setting defaults to disabled. On C7a it improved 1×2
latency at 96 cores. Larger Ryzen circuits favored the default.
[Full benchmark](docs/benchmarks/2026-11-18-public-verify-release372/README.md)

On a general CPU solve, `ZOPRO_CPU_SOLVER_FRONTEND=1` also runs the witness
instructions on the attached smaller pool. Witness allocation, scalar gathering,
FFT and curve operations keep their existing worker placement. This option
creates no pool or per-key table. On C7a its effect depends on the circuit.

On native targets, `ZOPRO_SOLVE_TASK_CLAIMS=1` lets workers take successive
ready tasks from a shared counter within each solver dependency band. The
option uses the existing Rayon pool and task storage. It defaults off. On a
Ryzen 9950X it reduced 1×2 proof latency with 16 and 32 workers. The
[benchmark report](docs/benchmarks/2026-11-18-wave-claims-release379/README.md)
includes the other circuit shapes and memory measurements.

For native bounded witness schedules, `ZOPRO_SOLVE_PACK_TASKS=1` groups
adjacent independent components into jobs of up to 128 operations. A larger
component keeps its own job. Outer dependency bands account for both depth
and operation count. Preparation reuses the existing schedule scratch.
Packing defaults off and can be used independently of task claiming.
[Measurements across transfer requests](docs/benchmarks/2026-11-19-packing-varied396/README.md)
compare the two options on a Ryzen 9950X.

For prepared sparse Poseidon tasks, `ZOPRO_CPU_POSEIDON_EMIT=1` stores the
wire and constraint-row addresses for an eligible S-box in its existing three
schedule words. The prover applies the option after it recognizes the field
coefficients. A layout whose addresses do not fit keeps the original
operation indices. The option defaults off. The
[full-proof measurements](docs/benchmarks/2026-11-19-poseidon-emit-release388/README.md)
compare its effect across eight circuits on Ryzen. On a Ryzen 9950X with 32
workers, enabling compact output addresses with the ordinary H query reduced
12×2 latency from 33.412 to 33.002 ms in seven fresh matched blocks.
[12×2 configuration measurements](docs/benchmarks/2026-11-19-medium-h-ryzen406/README.md)

On Ryzen 9950X with one worker on each of 16 physical cores, 12×2 also
benefits from `ZOPRO_CPU_DENSE_H_WINDOWS=18s` and
`ZOPRO_CPU_H_WINDOW_RANGES=1`. With compact Poseidon outputs enabled,
the median paired latency fell 3.43% against ordinary H across eight
distinct transfer witnesses. The 32-worker comparison favored ordinary H
in four of seven blocks.
[16-core witness comparison](docs/benchmarks/2026-11-19-varied-h413/README.md) ·
[16- and 32-worker comparison](docs/benchmarks/2026-11-19-single-range-h411/README.md)

On native IFMA, `ZOPRO_CPU_FFT_INPUT_COLUMNS=1` packs the Euler FFT
inputs while computing the first five inverse stages for domains of
8,192 points or larger. The option defaults off and uses the existing
root tables and a single packed transform buffer. On Ryzen 9950X 12×2,
the option lowered paired latency across eight witnesses by 1.04% at
16 workers and 0.46% at 32 workers against the identical build with
the option disabled. Both widths favored it on seven of eight witnesses.
[FFT configuration measurements](docs/benchmarks/2026-11-19-varied-fft417/README.md)

At 32 Ryzen 9950X workers, the same option also reduced paired latency on
the original 16×2, 24×2 and merge-24×1 requests by 1.50%, 2.99% and 2.18%,
respectively. Peak process RSS fell on all three. These settings were
confirmed in seven fresh blocks for each shape.
[Eight-shape FFT comparison](docs/benchmarks/2026-11-19-all-input-columns425/README.md)

With FFT input packing enabled, `ZOPRO_CPU_FFT_LOCAL_EULER=1` applies the
Euler weights between the two local transforms while the expanded values
are still in the same buffer. This opt-in IFMA path reuses the existing
root tables on domains from 8,192 through 2²⁸ points. On Ryzen 9950X
12×2, it reduced median paired latency against the same build by 0.88% at 16 workers (8/8 witnesses) and 2.03% at 32 workers (8/8 witnesses).
[Varied-witness measurements](docs/benchmarks/2026-11-19-varied-column-center427/README.md)

With native IFMA G2 Horner buckets and resource scheduling,
`ZOPRO_CPU_G2_RANGES=4` selects four point ranges and 11-bit windows for
G2 queries of 1,024 through 65,535 points when the worker pool has at least
32 threads and the extra scratch fits the optional memory allowance.
The default is two ranges and 12-bit windows on this path. On a C7a 1×2
proof, four ranges improved latency at 96 physical cores and increased it
at 64 cores. The scratch is released after the query.

With four ranges enabled, `ZOPRO_CPU_G2_DECOMPOSITION=serial` splits scalar
words sequentially in place before launching the range tasks. On 1×2 at
96 physical C7a cores this lowered both median process latency and request
p95. At 32 cores it increased latency. The option uses the same scratch as
the parallel split.

On native IFMA builds with resource scheduling, `ZOPRO_CPU_DENSE_H_WINDOWS=19`
uses mixed 13- and 14-bit windows for unsigned Montgomery H queries with at
least 262,144 points. The prover checks the additional scratch against its
optional memory allowance and falls back to ordinary windows under pressure.
The buckets live for one proof. This setting is opt-in.

Two signed layouts are also available through the same setting. `18s` uses
18 overlapping Booth windows and `17s` uses 17. Both extract signed digits
directly from scalar words. They use the same per-proof memory admission as
the unsigned layout. On C7a with 96 cores, `18s` reduced 16×2 latency from
36.108 to 35.841 ms in the same build. At 64 cores the unsigned layout
was faster on that shape. The 17-window layout used more bucket space and
gave mixed timing results. [Measured comparisons](docs/benchmarks/2026-11-18-signed-h-release366/aws.md)

With a mixed-width layout enabled, `ZOPRO_CPU_H_WINDOW_RANGES=2` splits each H window
into two disjoint point ranges. Their buckets are combined before weighting.
The prover accounts for both ranges when admitting the query. If this exceeds
the optional memory allowance it tries the single-range layout.

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

The experimental [parallel execution crate](parallel/README.md) provides a
standalone task/chunk API and benchmarks. It has an independent workspace;
prover integration and end-to-end speed effects remain unmeasured.

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

The [CPU performance playbook](docs/cpu-performance-playbook.md) describes the
experiment and adoption process. The [candidate register](docs/cpu-performance-candidates.json)
separates pending optimizations from adopted builds and failed follow-ups.

The latest adopted AVX CPU source, C7a qualification, all eight supplied M5 CPU
and Metal comparisons, source hashes and validation are in the
[G2 scalar-grain adoption report](docs/benchmarks/2026-11-11-phi-grain-parent176/README.md).
Earlier local-machine measurements are in the
[bitmap-reset adoption report](docs/benchmarks/2026-11-09-busy-reset-adoption176/README.md).

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
