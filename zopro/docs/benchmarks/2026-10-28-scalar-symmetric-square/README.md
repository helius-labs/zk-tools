# Scalar symmetric square component screen

The Rust scalar-square candidate passes this component screen. Across the three Fr contexts, the paired median geometric-mean latency ratio is **0.933235** versus fresh OFF (6.68% lower latency; marginal interval [0.932493, 0.933602], 31/31 wins). **0.932736** versus accepted library (6.73% lower latency; marginal interval [0.932043, 0.933579], 31/31 wins). No production change is adopted, and this is not a full-proof speedup. The result permits an actual solver/full-SDK qualification.

The private change replaces only native BMI2/ADX scalar **Fr** square dispatch with a safe Rust symmetric wide square and Montgomery reduction. The wide product uses six cross products and four diagonal products, versus 16 for a general four-limb product; reduction still uses 16. Fq and IFMA dispatch remain unchanged. No new unsafe code, assembly, heap allocation, runtime dependency, key table, retained plan or cache is introduced. The source-level wide temporary is 64 bytes plus fixed carries; this is not a measured stack bound or RAM minimum.

The arithmetic is adapted from the ordinary Rust fallback in [arkworks algebra v0.5.0](https://github.com/arkworks-rs/algebra/blob/v0.5.0/ff/src/fields/models/fp/montgomery_backend.rs), with attribution and the upstream MIT license included. Its assembly branch is not used in the new kernel.

## Measurements

Values are medians across 31 process means, in ns per stream update. Every update adds a fixed field constant before the square or complete fifth power. The four-stream cases charge all four operations, recurrence, black boxes and final output; setup and oracle generation are outside timing. Ratios below use paired block observations, not ratios of table medians.

| Field | Context | Accepted | Fresh OFF | ON | ON/OFF paired median | Marginal interval | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| fq | dependent | 13.926 | 13.933 | 12.939 | 0.929019 | [0.925435, 0.930142] | 31/31 |
| fq | four | 12.793 | 12.805 | 12.848 | 1.003190 | [1.001813, 1.004770] | 1/31 |
| fq | fifth | 40.865 | 40.851 | 40.840 | 0.999604 | [0.998713, 1.000113] | 20/31 |
| fr | dependent | 13.932 | 13.931 | 12.918 | 0.927411 | [0.926548, 0.928339] | 31/31 |
| fr | four | 12.891 | 12.886 | 12.632 | 0.981050 | [0.976872, 0.981619] | 31/31 |
| fr | fifth | 40.467 | 40.453 | 36.171 | 0.893890 | [0.892796, 0.894367] | 31/31 |

The unchanged-dispatch Fq dependent case also improves by about 7%. That improvement cannot be attributed to the Fr arithmetic reduction: code generation, layout or register allocation can affect this component probe. No specific cause is established. Fq four-stream and fifth-power controls remain near parity. The Fr fifth-power result is approximately 10.6% lower latency. A real solver and full proof are required to establish benefit in the prover.

Fresh/accepted calibration bounds are within [1/1.03, 1.03] in every context; all ON nonregression upper bounds are ≤1.02 against both controls. The Fr aggregate upper bounds are <1 with 31/31 wins against both. Intervals use ranks 10 through 22 of 31 paired ratios, with nominal 97.055% marginal coverage and no familywise guarantee. No gain floor, exclusions, schedule extension, successful-capture repeats or parameter sweep was used. The two OFF probes are byte-distinct; all library and probe identities are in the artifact receipt.

## Validation and profiling

Complete native field suites passed **35/35 OFF and 35/35 ON**. New tests compare 16,902 raw 512-bit square fixtures with independent Arkworks BigInt multiplication and 8,703 Montgomery inputs per field (including repeated boundary cases), plus eight repeated-square/fifth-power checks per input, with independent Arkworks field results, old multiply, dispatch and canonical-bound checks. Existing field, tower, curve and pairing tests remain. These checks cover this component; no core/prover/portable suite or new proof is claimed.

The fixed schedule was 31 rotated/reversed blocks, six contexts and three variants: 558 latency processes, 262,144 rounds each after three same-size primes, pinned to CPU 2. All 562 latency/profile processes matched the independent pinned Arkworks final-state fixture and reported zero timed allocations. All output digests were recomputed independently after capture.

Four separate FIFO-gated 199 Hz cycles:u profiles followed all latency captures: Fr dependent/fifth, OFF/ON, 4,194,304 rounds each after three same-size primes. All profiles preceded decoding and archive work. No owned helper, build, hash, archive or second benchmark overlapped captures. An unrelated host ceremony job was present in the recorded host snapshot; no exclusive-host claim is made.

| Profile | Samples | Sampled PIDs | Lost records |
|---|---:|---:|---:|
| profile-fr-dependent-fresh | 10 | 1 | 0 |
| profile-fr-dependent-candidate | 10 | 1 | 0 |
| profile-fr-fifth-fresh | 134 | 1 | 0 |
| profile-fr-fifth-candidate | 109 | 1 | 0 |

Total samples: 263. The dependent profiles are short and sparsely sampled. Symbol weights describe these loops and include small perf-control/accounting edges; they are not full-proof bottleneck shares or removable wall-time fractions. Raw perf.data files remain local with hashes; decoded logs are included.

Preparation failures are preserved. The first native compilation stopped before tests because two new test constructors lacked explicit field types; only test annotations/intermediate naming changed, and the production kernel bytes were checked equal. The repaired full suites then passed once each. Initial formatting also encountered a missing local perf module before setup copied it. After all captures, the first analysis reader rejected this perf version's combined PID/TID column; v3 corrected the reader and used the original decoded logs. No successful test/capture/profile was rerun for either correction.

Cargo primary-target mutable rlibs were not used. Each variant was frozen immediately from its hashed rmeta companion, with exact rustflags checked. OFF/ON library and executable hashes differ. All capture pins were checked before/after by xtask and again in analysis. The source audit verifies all 172 production files, 170 unchanged private originals, and exact inverse recovery of the original square fallback and lib suffix. The accepted SDK hash beginning with `14da` remains unchanged.

## Accepted prover comparison (inherited)

These are the accepted 172-source local full-proof observations, **not measurements of this candidate**. The supplied M5 CPU/Metal times are reference data. The all-eight CPU 10× objective remains unmet: 0/8; the primary target is 3.7 ms.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.44x | 2.88x |
| 2x2 | 11.460675 | 59 | 30 | 5.15x | 2.62x |
| 4x2 | 15.854365 | 109 | 48 | 6.88x | 3.03x |
| 8x2 | 24.084626 | 189 | 74 | 7.85x | 3.07x |
| 12x2 | 33.693397 | 284 | 101 | 8.43x | 3.00x |
| 16x2 | 39.981636 | 335 | 124 | 8.38x | 3.10x |
| 24x2 | 66.010670 | 528 | 176 | 8.00x | 2.67x |
| merge24 | 65.086624 | 507 | 175 | 7.79x | 2.69x |

Accepted merge54: 161.341835 ms; no supplied M5 reference. Historical accepted 1x2 peak RSS was 134.50 MiB; merge54 peaked at 1910.1875 MiB. These are observations, not RAM minima. The accepted FFT adoption retains its explicit engineering exception and false full-performance/qualified-scope gates. This component screen does not upgrade those qualifications.

No new SDK/Gnark export was generated; the independently verified proof ledger remains 12,884. No new all-eight/54, giant-circuit, pressure/concurrency, mobile/ARM/TEE or AWS/Aeglos measurement is claimed. The latest AWS comparison still uses the previous 171-source implementation: 1x2 Zopro at 11.966327 ms with 32 workers versus pinned Aeglos at 7.990218 ms with 96 workers. It is not a result for the current 172-source baseline or this candidate. No cloud resources were created.

## Reproduction and next action

See the frozen [design](artifacts/design.md), [source audit](artifacts/source-audit.json), [analysis](artifacts/analysis.json), [closeout](artifacts/closeout.json), [library/probe receipt](artifacts/probe-artifacts.json), full raw workflow logs and Rust helpers. Absolute local paths and input hashes are retained. Completed observations must not be rerun to repair tooling. Reproduction elsewhere requires rebuilding and remapping the pinned paths as a new study.

Next: qualify the existing sparse Poseidon solver with the same arithmetic, then a fresh full-SDK comparison. Do not select a different field, inlining setting, register schedule or layout to search for a passing result. A component gain alone does not justify adoption.
