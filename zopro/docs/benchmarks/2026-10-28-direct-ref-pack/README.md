# Direct coordinate-reference packing in G1

The candidate **fails the addition-caller screen and is not adopted**. This is an actual streaming-kernel component result, not a complete MSM or proof result. Production and the accepted SDK remain unchanged.

The four-G1-context paired aggregate selected/frozen median is **1.000899387**, interval [1.000423392, 1.001465770], 8/31 wins (+0.090% latency change; positive is slower).
The four-G1-context paired aggregate selected/fresh median is **1.001151757**, interval [1.000432622, 1.001453872], 4/31 wins (+0.115% latency change; positive is slower).

**The G2 control calibration fails by a large margin.** Fresh OFF takes about 3.11×/3.18× frozen at 200/352 pairs, despite unchanged G2 source, matching outputs and capacities, and equivalent command environments. Selected/fresh G2 ratios near 0.32 therefore receive no optimization credit. An offline [control audit](artifacts/control-layout-audit-v2.json) confirms byte-identical `.text`, `.rodata`, `.data` and `.rela.dyn` sections in the OFF executables; `.data.rel.ro` differs only in 20 source-location line numbers. The timing cause is unresolved; runtime or address-layout effects were not isolated. G1 calibration passes, but its separate aggregate gain requirements fail against both controls. No G2 timing was repeated or substituted.

## Change and scope

The new Rust AVX2/AVX-512 constructor reads eight existing 32-byte field references into vector registers, joins register halves, and performs the same transpose/radix-52 split as the original mixed-R256 constructor. It bypasses the contiguous eight-field input-array API. Only full-vector G1 forward input packs use it; field products, prefix order, scratch records, scalar tails, reverse packing, point order, G2, trees, four-lane and portable paths retain their original code. No heap buffer, key lookup table, cached plan, scheduler change or assembly rewrite is introduced.

This differs from the rejected [reference-staging experiment](../2026-10-27-g1-reference-staging/README.md), which changed temporary point arrays but still passed copied field arrays into the original constructor. It also differs from late/input-anchor records and G2 index-tree gathers. The official generated IFMA files and generator remain byte-identical and its consistency check passes. The new field impl is in a separate module.

Unsafe intrinsics are confined to field loads and vector operations. Valid references cover the complete initialized 32-byte loads, allow read aliasing, and require only field alignment; the module guard requires all CPU features used. The new field test checks both primes, lazy boundaries, noncontiguous and 32-byte-unaligned addresses, lane permutations and repeated references against an independent integer bit split and the original constructor/multiplication. It covers 24,864 lane inputs, including duplicates. Both 34-test native field suites pass.

All 172 production files remain exact. Frozen private source adds only two diagnostic core suffixes, retaining 170 exact originals. Fresh/selected private source retains 168 exact originals; the other four recover exactly through recorded inverse edits. No inverse-arithmetic change from earlier rejected candidates is inherited. Existing diagnostic adapters call the actual stream kernels; they do not reproduce the new arithmetic. G1 adapter successes from the initial baseline attempt and the corrected G2 case are retained, with 122 full core tests passing for fresh and selected.

## Fixed caller measurements

Six contexts use G1 owned unsigned, G1 borrowed signed and G2 prefix/norm additions at 200/352 pairs. These are synthetic valid generator multiples at primary batch sizes, not a captured witness distribution or complete bucket scheduler. Each process uses CPU 2, one active caller, three eight-round primes/checks, an outside-clock reset, and 1,024 complete stream calls. Fixture construction and independent weighted-generator checks are outside timing; the call loop, loading/packing, arithmetic, record traffic and destination writes are inside. Every destination is checked.

Values are medians of process means, in microseconds per complete batch. Paired ratios use corresponding blocks.

| Caller | Pairs | Frozen | Fresh OFF | Selected ON |
|---|---:|---:|---:|---:|
| g1-owned | 200 | 4.067 | 4.069 | 4.075 |
| g1-owned | 352 | 6.661 | 6.661 | 6.677 |
| g1-borrowed-signed | 200 | 4.276 | 4.274 | 4.274 |
| g1-borrowed-signed | 352 | 7.029 | 7.033 | 7.023 |
| g2-prefix | 200 | 7.703 | 23.960 | 7.705 |
| g2-prefix | 352 | 13.070 | 41.628 | 13.077 |

| Caller | Pairs | Paired ratio | Median | Marginal interval | Wins | Check |
|---|---:|---|---:|---:|---:|---|
| g1-owned | 200 | fresh/frozen | 0.999133968 | [0.998300257, 1.000711190] | 18/31 | pass |
| g1-owned | 200 | selected/frozen | 1.001766977 | [1.000929938, 1.003153870] | 5/31 | pass |
| g1-owned | 200 | selected/fresh | 1.002129528 | [1.001098080, 1.003472623] | 5/31 | pass |
| g1-owned | 352 | fresh/frozen | 1.000227781 | [0.999457197, 1.000655116] | 14/31 | pass |
| g1-owned | 352 | selected/frozen | 1.002000622 | [1.001524939, 1.002762959] | 0/31 | pass |
| g1-owned | 352 | selected/fresh | 1.002020904 | [1.001310868, 1.002801851] | 3/31 | pass |
| g1-borrowed-signed | 200 | fresh/frozen | 0.999442012 | [0.997928926, 1.003235124] | 17/31 | pass |
| g1-borrowed-signed | 200 | selected/frozen | 1.000548451 | [0.999076689, 1.001550549] | 14/31 | pass |
| g1-borrowed-signed | 200 | selected/fresh | 1.000015927 | [0.997926907, 1.001228890] | 15/31 | pass |
| g1-borrowed-signed | 352 | fresh/frozen | 1.000158340 | [0.999352356, 1.000803586] | 14/31 | pass |
| g1-borrowed-signed | 352 | selected/frozen | 0.999503858 | [0.998736948, 1.000151715] | 20/31 | pass |
| g1-borrowed-signed | 352 | selected/fresh | 0.999669363 | [0.997797042, 1.000632459] | 18/31 | pass |
| g2-prefix | 200 | fresh/frozen | 3.112582219 | [3.106478755, 3.115384164] | 0/31 | fail |
| g2-prefix | 200 | selected/frozen | 1.001103592 | [0.999172213, 1.001636627] | 13/31 | pass |
| g2-prefix | 200 | selected/fresh | 0.321448735 | [0.321078645, 0.322331363] | 31/31 | pass |
| g2-prefix | 352 | fresh/frozen | 3.186119550 | [3.179765521, 3.195859916] | 0/31 | fail |
| g2-prefix | 352 | selected/frozen | 1.000253266 | [0.999478865, 1.001206268] | 14/31 | pass |
| g2-prefix | 352 | selected/fresh | 0.313980797 | [0.313084821, 0.314395551] | 31/31 | pass |

Failed checks: `["g2-prefix/200/fresh/frozen","g2-prefix/352/fresh/frozen","aggregate/selected/fresh","aggregate/selected/frozen"]`. The fixed gate requires every fresh/frozen calibration interval within [1/1.03, 1.03], selected nonregression upper bounds ≤1.02 for all six contexts against both controls, and four-G1-context geometric aggregate upper <1 with at least 21/31 wins against both. The two G2 cases are controls, not included in the gain aggregate. Bounds use ordered ranks 10 through 22 of 31 paired process means, nominal 97.055% marginal coverage with no familywise guarantee. No gain floor, exclusions, extensions, successful-capture repeats or parameter sweeps were used.

All 558 latency captures match independent weighted-generator outputs, pairwise digests and scratch capacities; all report zero timed allocations. G1 record payload is 40,000/70,400 bytes at 200/352; G2 is 16,000/28,160. The borrowed diagnostic input vector is 3,200/5,632 bytes outside timing, not a proposed new prover allocation. Unchanged capacities do not establish compiler stack behavior or a whole-process RAM saving. CPU/process-RSS observations are retained in analysis; they are not minimum-RAM measurements.

## Code generation and profiles

The [code-generation audit](artifacts/codegen-audit.json) uses the actual linked caller binaries and retains every selected G1 stream disassembly. Counts below are static whole-function sites; they are not retired instructions, dynamic copies, spills or removable latency. Loads may fold into memory-operand inserts.

| Variant | Stream | Text bytes | Instructions | IFMA sites | Stack-address sites | Insert sites / memory inserts |
|---|---|---:|---:|---:|---:|---:|
| frozen | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26564 | 4396 | 800 | 645 | 34 / 16 |
| frozen | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 27120 | 4533 | 800 | 665 | 33 / 21 |
| frozen | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 27015 | 4521 | 800 | 660 | 33 / 21 |
| frozen | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26356 | 4419 | 800 | 627 | 33 / 21 |
| frozen | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26216 | 4412 | 800 | 615 | 33 / 21 |
| fresh | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26564 | 4396 | 800 | 645 | 34 / 16 |
| fresh | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 27120 | 4533 | 800 | 665 | 33 / 21 |
| fresh | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 27015 | 4521 | 800 | 660 | 33 / 21 |
| fresh | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26356 | 4419 | 800 | 627 | 33 / 21 |
| fresh | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26216 | 4412 | 800 | 615 | 33 / 21 |
| selected | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, false, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26488 | 4415 | 800 | 610 | 34 / 28 |
| selected | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26890 | 4493 | 800 | 629 | 30 / 26 |
| selected | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, false, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 27043 | 4539 | 800 | 628 | 30 / 28 |
| selected | `zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &zopro_bn254::curve::Affine<zopro_bn254::curve::G1Config>>` | 26278 | 4448 | 800 | 586 | 31 / 29 |

Four FIFO-gated warm 199 Hz cycles:u profiles follow all latency: G1 owned/200 and borrowed-signed/352, fresh/selected, 262,144 calls each after three eight-call primes. All profiles precede decoding, hashes and archive work. No owned helper, build, hash, archive or other benchmark overlaps capture. Unrelated host work is recorded, with no exclusive-host claim.

| Caller | Variant | Samples | Lost |
|---|---|---:|---:|
| g1-borrowed-signed | fresh | 364 | 0 |
| g1-borrowed-signed | selected | 363 | 0 |
| g1-owned | fresh | 205 | 0 |
| g1-owned | selected | 200 | 0 |

Total samples: 1132. Every profile has one sampled PID and zero lost records. Symbol weights are component observations, not full-proof wall-time fractions. Raw perf.data files stay local with hashes; decoded observations are included.

The control audit freezes every transitive rlib and shared object immediately after each isolated variant build, checks actual field/core rustflag fingerprints, and links only the frozen dependency directory. OFF/ON field and probe hashes differ. Compiler, source, protocol, code-generation observations and binaries were pinned before timing; all final pins pass after capture.

Two preparation failures are preserved. The original native test environment omitted the prefix-record flag required by the G2 control; both successful G1 adapter tests were retained and only the failed G2 test was retried with the accepted runtime settings. The next freezer assumed a hashed core rlib filename, while Cargo emits an un-hashed core rlib for its cdylib/rlib target. The freezer now audits matching core fingerprints and freezes that artifact immediately; the successful G2 test and library build were not repeated. These repairs occurred before all timing and changed no candidate kernel source.

## Accepted comparison (inherited)

The original all-eight 10× CPU objective remains active and unmet, 0/8. These are accepted 172-source full-proof observations, not this component's all-shape measurements. The M5 CPU and Metal times are supplied reference data.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted primary historical peak RSS: 134.50 MiB; merge54: 161.341835 ms / 1910.1875 MiB peak, with no supplied M5 reference. Primary CPU target: 3.7 ms. The accepted FFT adoption retains its explicit engineering exception and false full-performance/qualified-scope gates.

No new SDK/Gnark proof was generated; the independently verified ledger remains 12,983. No new complete-MSM, all-eight/54, portable/mobile/TEE, pressure/concurrency, giant-circuit or AWS/Aeglos result is claimed. No cloud resource was created.

Research checked [PSSM](https://www.mdpi.com/2079-9292/15/16/3660), which schedules independent scalar-multiplication sequences, and [Versal AIE point additions](https://arxiv.org/abs/2502.11660), which target different hardware and field size. Neither supplies a matched performance prediction for this already vectorized prover. The implemented load change follows the current source dataflow; no paper or external code speedup is transferred. Earlier SRE, memory-adaptive buckets, GLV/ring and shifted-base investigations remain as recorded.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [capture seal](artifacts/capture-seal.json), [identity audit](artifacts/control-identity-audit.json) and all raw logs are archived.

Close this exact direct-input candidate without SDK integration. Preserve all positive estimates and control failures; do not repeat unchanged input layout or intrinsic variants. Before further G2 kernel work, investigate the control discrepancy with a separately declared baseline diagnostic that records runtime data layout and profiles unchanged arithmetic; this does not rescue the closed G1 candidate.

An additional post-capture diagnostic mistake is recorded in [control restoration](artifacts/control-restoration.json). GNU `objcopy --dump-section INPUT` with no separate output rewrote the two original OFF ELF files after all captures, workflow pin checks and initial analysis had finished. Those rewritten files were preserved locally. Repeating only the recorded probe-link commands, with the same source, frozen libraries, output paths and working directory, reconstructed both exact pre-capture SHA-256 values. All final capture pins pass after restoration. No Cargo build, test, timing or profile was repeated. The selected executable was untouched. The first offline source-location reader also missed PIE relocation addends; the corrected reader uses direct ELF reads of the restored originals and preserves the original failure log.
