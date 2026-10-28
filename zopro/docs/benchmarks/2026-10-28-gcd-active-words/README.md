# Active-word GCD integer updates

**Rejected at the concurrent actual-caller gate; no SDK trial or adoption.** The three real streaming contexts at 32 logical CPUs have ON/fresh-OFF paired aggregate 1.000239921, bounds [0.994370045,1.009596838], 14/31 wins. ON/frozen is 1.002958819, bounds [0.997131296,1.007339453], 12/31 wins. Calibration and nonregression pass, but the declared aggregate gain does not.

The candidate shortens only the full GCD integer updates after the existing bit-length test: two words through 128 bits, three through 192, otherwise four. A single width dispatch selects both matrix rows. Known-zero high words no longer participate in signed products/carries. The binary-GCD matrices, row-sign normalization, 31/62-bit modular coefficient updates, exact one-word Euclid finish and final Montgomery scale remain unchanged. This adds no heap buffer, lookup table, cached key/plan, assembly, unsafe operation or public runtime option.

For N-word A/B and row L1 norm at most 2^31, the signed combination's absolute magnitude is below 2^(64N+31). After the existing exact division by 2^31, it fits N words. Signed intermediates fit i128; two's-complement negation covers N+1 words and output upper words are zero. Modular coefficients remain four words. This differs from the earlier rejected signed62 safegcd algorithm, which changed the entire inversion method and also shrank integers. No previous inverse/weighted/square candidate source is inherited.

## Complete caller measurements

The new concurrency harness is derived from the preceding [diagnostic](../2026-10-28-concurrent-callers/README.md), with the weighted setter removed. Three executables link independent immutable libraries: frozen accepted arithmetic from direct-ref-pack, fresh OFF, and active-word ON. Both fresh variants share identical private source; only the active-word compile cfg differs. No mutable Cargo-root library is reused.

31 rotated/reversed blocks cover G1 owned/200, G1 borrowed-signed/200 and /352, and G2 prefix/200, on one thread [2], 16 physical cores [0–15] and 32 logical CPUs [0–31]. Every worker has independent fixtures/scratch, three checked/reset 1,024-call primes, then 8,192 timed complete calls. Main elapsed spans start to done barriers, including the slowest worker; per-worker elapsed, thread CPU, throughput and usage are preserved. Output checks and all allocations happen outside every worker's clock. These synthetic ordinary points use real stream batch sizes but are not witness/MSM/proof timing.

| Caller | Pairs | Layout | Frozen µs/wave | Fresh OFF µs/wave | ON µs/wave | ON/OFF paired median | Marginal bounds | Wins |
|---|---:|---|---:|---:|---:|---:|---|---:|
| g1-owned | 200 | solo | 4.159051 | 4.145036 | 4.118231 | 0.991638173 | [0.990045353, 0.998618619] | 24/31 |
| g1-owned | 200 | physical16 | 4.664638 | 4.632856 | 4.639628 | 0.997539121 | [0.994073715, 1.000543653] | 20/31 |
| g1-owned | 200 | logical32 | 6.906715 | 6.889446 | 6.861497 | 0.999265074 | [0.988860909, 1.003304545] | 17/31 |
| g1-borrowed-signed | 200 | solo | 4.364413 | 4.343723 | 4.331513 | 0.997110204 | [0.992826211, 1.002428293] | 21/31 |
| g1-borrowed-signed | 200 | physical16 | 4.868379 | 4.851123 | 4.858600 | 1.001092841 | [0.997872837, 1.003301720] | 12/31 |
| g1-borrowed-signed | 200 | logical32 | 7.058096 | 7.083829 | 7.102364 | 1.002376904 | [0.997652675, 1.009569277] | 13/31 |
| g1-borrowed-signed | 352 | solo | 7.158289 | 7.121947 | 7.098317 | 0.997266919 | [0.995665571, 1.002007655] | 21/31 |
| g1-borrowed-signed | 352 | physical16 | 8.118133 | 8.131362 | 8.110196 | 1.000907387 | [0.997803268, 1.004301412] | 15/31 |
| g1-borrowed-signed | 352 | logical32 | 11.837098 | 11.814889 | 11.827071 | 1.002790316 | [0.997797422, 1.008018935] | 12/31 |
| g2-prefix | 200 | solo | 7.906354 | 7.840833 | 7.794076 | 0.994894794 | [0.992188019, 0.996978056] | 26/31 |
| g2-prefix | 200 | physical16 | 9.152897 | 9.057739 | 9.114924 | 0.997008807 | [0.995311291, 1.002756128] | 18/31 |
| g2-prefix | 200 | logical32 | 14.741424 | 14.730824 | 14.698214 | 0.997410947 | [0.992866145, 1.001238571] | 19/31 |

| Caller/pairs/layout | Fresh/frozen paired median | Marginal bounds | ON/frozen paired median | Marginal bounds |
|---|---:|---|---:|---|
| g1-owned/200/solo | 0.994282970 | [0.991674328, 1.002269384] | 0.987243381 | [0.985095864, 0.991556668] |
| g1-owned/200/physical16 | 0.998206050 | [0.995202653, 1.000552460] | 0.996548134 | [0.992298889, 0.998691527] |
| g1-owned/200/logical32 | 0.998988016 | [0.994284927, 1.001146714] | 0.992383587 | [0.984952118, 1.000255822] |
| g1-borrowed-signed/200/solo | 0.993545979 | [0.992081844, 0.996254020] | 0.992426399 | [0.988561710, 0.995389772] |
| g1-borrowed-signed/200/physical16 | 0.995957256 | [0.992031295, 0.999578232] | 0.997024780 | [0.995415463, 1.000976209] |
| g1-borrowed-signed/200/logical32 | 1.003348068 | [1.000613294, 1.009942497] | 1.007452535 | [1.003740479, 1.009989283] |
| g1-borrowed-signed/352/solo | 0.992025311 | [0.990513877, 1.000971543] | 0.993451717 | [0.987676075, 0.997472045] |
| g1-borrowed-signed/352/physical16 | 0.996293197 | [0.990536924, 0.999910006] | 0.997742317 | [0.993024615, 1.001124905] |
| g1-borrowed-signed/352/logical32 | 0.997381583 | [0.993426073, 1.002704227] | 1.001860120 | [0.996361505, 1.009087910] |
| g2-prefix/200/solo | 0.991216522 | [0.987040533, 1.002027395] | 0.986974107 | [0.983485985, 0.993954825] |
| g2-prefix/200/physical16 | 0.997268402 | [0.985663835, 1.002001432] | 0.993060822 | [0.990794251, 0.998189777] |
| g2-prefix/200/logical32 | 0.999989389 | [0.993952936, 1.012528809] | 0.996635837 | [0.987769002, 1.004019948] |

The primary geometric aggregate includes borrowed G1/200, borrowed G1/352 and G2/200 at logical32. Owned G1 is a nonregression supplement. Aggregate upper bounds must be below parity with at least 21/31 wins against both controls; every context's ON upper bound must be at most 1.02, and fresh/frozen calibration must fit [1/1.03,1.03]. Bounds are ranks 10–22 of 31 paired ratios, nominal 97.055063% marginal coverage under independent exchangeable blocks, without a familywise guarantee. No gain floor, exclusion, extension or successful-capture retry is used. All context calibrations and nonregressions pass; both primary gain comparisons fail.

Small improvements are retained: owned G1 solo ON/OFF is about 0.836% lower with bounds below parity; G2 solo is about 0.511% lower with bounds below parity. Borrowed solo estimates improve about 0.27–0.29% with bounds crossing parity. G2 logical32 has a positive roughly 0.259% estimate but also crosses parity. These are valid scoped observations, not zero gains or evidence of a qualified parallel prover gain. Several fresh/frozen controls differ despite unchanged baseline arithmetic; all controls and raw observations remain included.

## Counters and profiles

All 1,116 unprofiled latency processes finish first, followed by four FIFO-gated grouped perf-stat processes, then four separate 199 Hz cycles:u profiles. Each uses logical32 and 262,144 calls per worker after the same primes. All eight finish before decode/hash/compression/archive work. No owned build/helper/hash/archive/other benchmark overlaps capture. Host/compiler/topology snapshots and pins are archived; no exclusive-host claim is made.

| Context/mode | Cycles/kernel call | Instructions/kernel call | IPC | Branch misses/call | Generic cache misses/call |
|---|---:|---:|---:|---:|---:|
| stat-g1-borrowed-signed-352-m1 | 48676.667 | 85029.988 | 1.746833 | 12.356988 | 10.867897 |
| stat-g1-borrowed-signed-352-m2 | 49080.562 | 84667.327 | 1.725068 | 13.515409 | 24.785486 |
| stat-g2-prefix-200-m1 | 60055.376 | 109428.421 | 1.822125 | 14.444045 | 17.998794 |
| stat-g2-prefix-200-m2 | 60530.352 | 109066.333 | 1.801845 | 14.851468 | 7.988780 |

All four grouped events run at 100%. Counts sum logical-thread events and divide by all kernel calls; SMT cycles are not disjoint physical-core cycles. These are single counter processes, descriptive observations without confidence bounds. Retired instructions fall about 0.427% for G1 and 0.331% for G2, while cycles rise about 0.830% and 0.791% in these stat observations. Branch misses also rise. Fewer executed instructions alone do not establish throughput or latency improvement; these counters do not prove an exclusive branch, port or cache cause. Generic cache misses are neither cache-level attribution nor DRAM bandwidth.

| Profile | Samples | Sampled worker TIDs | PIDs | Lost |
|---|---:|---:|---:|---:|
| cycles-g1-borrowed-signed-352-m1 | 19391 | 32 | 1 | 0 |
| cycles-g1-borrowed-signed-352-m2 | 19363 | 32 | 1 | 0 |
| cycles-g2-prefix-200-m1 | 24437 | 32 | 1 | 0 |
| cycles-g2-prefix-200-m2 | 24249 | 32 | 1 | 0 |

Total samples: 87440. Every profile has one PID, all 32 active workers sampled and zero lost records. Full symbol weights and decodes are retained; sampled inverse/stream periods are not removable wall time. Raw perf.data remains local with hashes, and loss-record dumps are archived losslessly with deterministic gzip.

## Correctness, provenance and memory

All 34 unfiltered native field tests pass in three configurations: ordinary OFF, active ON, and active ON with overflow/debug checks. The one new test performs 48,655 row comparisons at every bit length 65–254, including 128/192 boundaries, full-word corners and pure ±2^31 rows. It compares the original four-word method with a separate unsigned five-word magnitude oracle and the shortened method. The same cases rerun across builds are not additional distinct inputs. Existing 10,000 random full inverse tests per field, raw powers, canonical boundaries and one-word finishing checks retain their independent Fermat identity.

Both full native core suites pass 122 tests: 119 accepted cases plus three unchanged diagnostic caller oracles. Each of the three linked harness configurations also passes three tests, including the concurrency/affinity/barrier/output test and two inherited perf-helper tests. All tests/builds/captures succeed on their first attempts; no successful command is repeated. The skipped-word thresholds are exact machine-word boundaries, not tuned parameters.

The 172-file private snapshot has 169 exact originals, only inverse.rs arithmetic dispatch/module inclusion, and diagnostic-only core/MSM reexports. Reverse edits recover every original byte. G1/G2 arithmetic, coefficients and Euclid finish are unchanged. All transitive rlibs and proc-macro objects are frozen immediately after each separate target build; exact flags/features and independent frozen/fresh/candidate hashes are checked. No rejected weighted-anchor, direct-finish or symmetric-square kernel is included.

All **1,124** latency/stat/profile processes pass full independent generator output/digest/capacity checks and zero timed global allocations. They execute **216,432,640 timed kernel calls**, not SDK proofs. Per-worker scratch is unchanged: G1/200 40,000 bytes, G1/352 70,400, G2/200 16,000; borrowed reference vectors 3,200/5,632 bytes are created outside clocks. The helper has fixed small local arrays; no measured stack bound, proof peak RSS, RAM minimum or pressure benefit is established.

## Research audit and inherited proof comparison

The reviewed [SQIsign paper, sections 3.1–3.7](https://arxiv.org/html/2608.13948v2), reorganizes SIMD dependency graphs and uses a Montgomery-friendly prime. Its scheme, parameters and reported speedups are not BN254 predictions. The [Cd monograph's introductory geometry and operation tables](https://arxiv.org/pdf/2608.01675v5) give an untwisted family with rational four-torsion. That family cannot directly represent the odd-order BN254 G1 group by a ground-field isomorphism; no curve-model port was selected. Only the stated sections were reviewed, not the entire monograph. The implemented active-word change follows the existing source's zero high words and does not copy either implementation. Earlier GLV/ring, SRE, memory-adaptive buckets, shared-complex products and inverse trials were audited and remain separate closed evidence.

The all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source timings and supplied M5 references are inherited, not remeasured here. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Historical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof was generated; ledger remains 13,082. No new all-eight/54, portable/mobile/TEE, pressure, giant-circuit or AWS/Aeglos result is claimed. No cloud resource was created.

The exact active-word candidate is closed without SDK integration. Retain its solo gains and failed parallel aggregate; do not rerun the same thresholds, arithmetic or annotation/layout variants to seek a pass. The next mechanism needs a distinct reduction in arithmetic or bookkeeping cost that survives complete concurrent caller tests. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native validation](artifacts/native/result.json) and raw logs are included.
