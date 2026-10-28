# Paired SIMD GCD rows in concurrent point callers

The candidate **fails the concurrent actual-caller gate and is not adopted**. Its 32-logical-CPU real-stream aggregate ON/OFF is **1.005899577**, bounds [1.004370989,1.009570679], 5/31 wins. ON/frozen is **1.008568442**, bounds [1.006469150,1.012498678], 4/31 wins. Calibration true, nonregression false, gain false. Production and the accepted SDK remain unchanged.

This ports the exact runtime helper from the [complete-inverse prototype](../2026-10-28-gcd-vector-rows/README.md), which improved 32-thread Fq inversion by about 1.46% versus a copied ordinary routine and 0.76% versus the accepted field-library control. Four VPMULUDQ products compute both full-integer 31-bit GCD rows after shared radix-32 conversion. Masked corrections handle signed coefficients; scalar carries reconstruct the existing signed five-word values, then the original absolute-value, row-sign and 31-bit shift follow. Matrices, modular 31/62-bit coefficient updates, Euclid finish, scale and all surrounding point/IFMA arithmetic remain unchanged.

The new helper is selected only under a private cfg plus x86-64 AVX512F/AVX2 and not(no_ifma). The original two integer updates remain verbatim in the fallback. Test reference paths alone differ from the prototype; the runtime prefix is byte-exact. Bounded intrinsic loads/stores are the same as the prototype. There is no added heap buffer, point/key table, cached plan, assembly backend or public runtime option. Active-word dispatch and all rejected weighted/square/finish code are absent.

## Complete caller measurements

31 fixed rotated/reversed blocks cover G1 borrowed-signed batches 200/352, G2 prefix/200, and G1 owned/200 as a supplement. Each runs solo [2], physical16 [0–15], logical32 [0–31], with independent frozen accepted-library, fresh OFF and paired ON executables. All share the same harness source. Three checked/reset 1,024-call primes precede 8,192 complete calls per worker. Ready/start/done/release barriers prevent any worker's checking/allocation from overlapping any clock. Main ns/wave includes the slowest worker and barrier tail; per-worker elapsed/CPU and throughput remain in the raw results. These are synthetic ordinary points at real stream sizes, not witness/MSM/proof latency.

| Caller | Pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | ON/OFF paired median | Marginal bounds | Wins |
|---|---:|---|---:|---:|---:|---:|---|---:|
| g1-owned | 200 | solo | 4.159859 | 4.142672 | 4.216830 | 1.015706648 | [1.011224651,1.020586305] | 0/31 |
| g1-owned | 200 | physical16 | 4.652289 | 4.632835 | 4.748938 | 1.025166047 | [1.022791644,1.027607963] | 2/31 |
| g1-owned | 200 | logical32 | 6.932486 | 6.912337 | 6.994933 | 1.012561610 | [1.009184474,1.018008652] | 4/31 |
| g1-borrowed-signed | 200 | solo | 4.356240 | 4.342114 | 4.377531 | 1.006186951 | [1.004072879,1.012606177] | 0/31 |
| g1-borrowed-signed | 200 | physical16 | 4.861432 | 4.837312 | 4.922825 | 1.015435707 | [1.012689905,1.019759323] | 2/31 |
| g1-borrowed-signed | 200 | logical32 | 7.045710 | 7.079656 | 7.114640 | 1.009091599 | [1.001174798,1.012887435] | 8/31 |
| g1-borrowed-signed | 352 | solo | 7.138184 | 7.112123 | 7.139600 | 1.003808924 | [1.002259632,1.009098593] | 2/31 |
| g1-borrowed-signed | 352 | physical16 | 8.106213 | 8.035449 | 8.138520 | 1.008491794 | [1.005267755,1.011384930] | 7/31 |
| g1-borrowed-signed | 352 | logical32 | 11.777112 | 11.801072 | 11.886607 | 1.009102847 | [1.004621707,1.016978692] | 8/31 |
| g2-prefix | 200 | solo | 7.872007 | 7.842853 | 7.914124 | 1.007296585 | [1.005725060,1.010831475] | 3/31 |
| g2-prefix | 200 | physical16 | 9.132208 | 9.002177 | 9.190353 | 1.011121077 | [1.007971531,1.018626644] | 1/31 |
| g2-prefix | 200 | logical32 | 14.744629 | 14.752230 | 14.805360 | 1.002532332 | [0.998348996,1.005582386] | 12/31 |

| Caller/pairs/layout | OFF/frozen paired median | Marginal bounds | ON/frozen paired median | Marginal bounds |
|---|---:|---|---:|---|
| g1-owned/200/solo | 0.993653479 | [0.992215216,1.002776270] | 1.014002622 | [1.005606371,1.018571640] |
| g1-owned/200/physical16 | 0.997683673 | [0.994527502,1.000490236] | 1.022871070 | [1.019955675,1.026572680] |
| g1-owned/200/logical32 | 0.998939689 | [0.989455186,1.006512674] | 1.012774457 | [1.001396436,1.022579938] |
| g1-borrowed-signed/200/solo | 0.994041786 | [0.992491664,1.003114508] | 1.003404888 | [0.999212716,1.007668560] |
| g1-borrowed-signed/200/physical16 | 0.998831042 | [0.996223700,1.000656888] | 1.013000173 | [1.009501369,1.016514742] |
| g1-borrowed-signed/200/logical32 | 1.002557626 | [0.993133045,1.010436471] | 1.007832046 | [1.003735429,1.014560416] |
| g1-borrowed-signed/352/solo | 0.992247563 | [0.991239793,1.001728083] | 0.997226046 | [0.994847388,1.003561033] |
| g1-borrowed-signed/352/physical16 | 0.994016151 | [0.990689946,1.001698250] | 1.005026747 | [0.996294668,1.008028190] |
| g1-borrowed-signed/352/logical32 | 1.000071503 | [0.993680734,1.005112183] | 1.008832074 | [1.004145297,1.014163686] |
| g2-prefix/200/solo | 0.993848587 | [0.991658354,1.003071200] | 1.002451602 | [0.999522629,1.008649376] |
| g2-prefix/200/physical16 | 0.993866258 | [0.989086362,1.001201496] | 1.006879093 | [1.003106873,1.010747933] |
| g2-prefix/200/logical32 | 1.003673242 | [0.998571980,1.008167822] | 1.007957241 | [1.000693243,1.012753378] |

The native real-stream aggregate is 0.590% slower than fresh OFF and 0.857% slower than frozen, with both marginal intervals above parity. Every one of the twelve ON/OFF paired medians is above one. The owned/200 physical16 estimate is about 2.52% slower than OFF and fails the nonregression bound; other failed bounds are listed in result-interpretation.json. A small roughly 0.277% lower borrowed/352 solo estimate against frozen remains included, but it crosses parity and fresh OFF is already faster than frozen there.

This reversal also appears without SMT: borrowed/200 solo ON/OFF is about 0.619% slower, and G2/200 solo about 0.730% slower, with bounds above parity. SMT alone therefore cannot explain the integration result. The standalone prototype and native caller differ in surrounding computation, input distributions and compiled context; exact helper source does not establish equivalent whole-routine execution cost. The exclusive cause is unresolved. The positive inverse-only result is retained within its measured scope, while this native integration is closed without an SDK trial.

The primary aggregate is the per-block geometric mean of borrowed G1/200, borrowed G1/352 and G2/200 at logical32. It requires upper bounds below parity and at least 21/31 wins against both controls. All twelve contexts require ON upper bounds at most 1.02; OFF/frozen calibration bounds must fit [1/1.03,1.03]. Ordered ranks 10–22 of 31 ratios give nominal 97.055063% marginal coverage under independent exchangeable blocks, without familywise guarantees. Owned G1 remains a nonregression supplement. All small positive estimates, uncertainty and failed checks remain visible; there is no positive-gain floor, exclusion, extension, successful-capture retry or unchanged reroll. Ratios of table medians need not equal paired medians.

## Native validation and provenance

Three full native field suites pass 35 tests each: checked active ON with overflow/debug checks, ordinary OFF, and active ON. They reuse the exact prototype's two pair/carry oracles (65,616 pairs / 131,232 rows) with only parent-reference paths adapted, plus the accepted 33 field tests. The rows include every bit length 1–256, arbitrary bounded rows, zero, cancellation, ±2^31 and full-word cases. Existing full inverse/Fermat/raw-power/one-word tests remain. Repeated cases across builds are not additional distinct inputs.

Both complete native core suites pass 122 tests, including the three original owned/borrowed G1/G2 generator adapters. Each frozen/OFF/ON harness passes three tests, including concurrency/affinity/barrier/output and inherited perf-helper checks. Some optional external-fixture core cases return without fixtures; those external scenarios are not claimed. There is no raw/cubic/SDK qualification at this stage.

The initial source-generator attempt stopped before native setup because it expected spaces around a retained long-string assignment. The preserved versioned fix changes that source-format match only. No native test/build/capture had run before it, and no successful command was repeated. A later link-plan preparer stopped before probe linking on inherited make-harness filenames; its preserved fix pins the actual make-native helpers. No native build/test or capture was repeated. The native/link/capture workflows themselves pass on their first attempts. An inherited console label says field34x3, while the enforced assertions and all three unfiltered field-suite logs correctly record 35; the original output is retained. The runtime helper matches the passed prototype exactly before the test module. Reversing the private inverse edit and removing diagnostic reexports recovers all 172 originals; 169 originals remain byte-exact. All transitive rlibs/proc-macro objects are frozen immediately in separate target roots. Native flags/features and distinct field/core hashes are checked, and read-only disassembly records actual inverse code inside the point-caller executables.

All **1,124** latency/stat/profile processes pass independent full generator output, digest and capacity checks with zero global timed allocations. They execute **216,432,640 timed kernel calls**, not SDK/Gnark proofs. Per-worker G1/200 scratch is 40,000 bytes, G1/352 70,400 and G2/200 16,000. Borrowed fixture reference vectors are created outside clocks. The helper's fixed local arrays are not a measured stack bound or proof-RSS/RAM-minimum claim.

## Counters and profiles

All 1,116 latency captures finish before four FIFO-gated grouped perf-stat processes, followed by four separate 199 Hz cycles:u profiles. Each uses logical32 and 262,144 calls per worker after the same primes. All eight finish before decode/hash/compression/archive. No owned build/helper/hash/archive/other benchmark overlaps capture. Host/compiler/topology snapshots are archived without an exclusive-host claim.

| Context/mode | Cycles/kernel call | Instructions/kernel call | IPC | Branch misses/call | Generic cache misses/call |
|---|---:|---:|---:|---:|---:|
| stat-g1-borrowed-signed-352-m1 | 48930.671 | 85029.988 | 1.737765 | 12.844268 | 30.379079 |
| stat-g1-borrowed-signed-352-m2 | 49013.139 | 84798.518 | 1.730118 | 11.856059 | 10.465682 |
| stat-g2-prefix-200-m1 | 60575.607 | 109428.854 | 1.806484 | 14.297085 | 0.922407 |
| stat-g2-prefix-200-m2 | 61153.522 | 109197.274 | 1.785625 | 13.766471 | 15.426591 |

All groups at 100% running: true. Counts sum logical-thread events and divide by all kernel calls; SMT cycles are not disjoint physical-core cycles. Each counter row is a single descriptive observation, not a confidence bound or exclusive cost attribution. IPC, sampled period and generic cache misses do not establish removable proof time, an execution-port cause, cache level or DRAM bandwidth.

| Profile | Samples | Sampled worker TIDs | PIDs | Lost |
|---|---:|---:|---:|---:|
| cycles-g1-borrowed-signed-352-m1 | 19429 | 32 | 1 | 0 |
| cycles-g1-borrowed-signed-352-m2 | 19359 | 32 | 1 | 0 |
| cycles-g2-prefix-200-m1 | 24341 | 32 | 1 | 0 |
| cycles-g2-prefix-200-m2 | 24409 | 32 | 1 | 0 |

Total samples: 87538. Decodes and full symbol weights are archived. Raw perf.data stays local with hashes; loss dumps are archived losslessly using deterministic gzip.

## Inherited full-proof comparison

No proof timing changes here. The all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source timings and supplied M5 references are inherited. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.

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

Historical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof is generated; ledger remains 13,082. No new all-eight/54, portable/mobile/TEE, whole-proof concurrency/pressure, giant-circuit or AWS/Aeglos result is claimed. No cloud resource is created.

The exact native integration is closed at this caller gate. Retain the complete-inverse gain and every point-caller positive estimate/failure; no unchanged arithmetic, annotation/layout or scheduling reroll follows.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [runtime provenance](artifacts/runtime-provenance.json), [source audit](artifacts/source-audit.json), [actual code](artifacts/codegen.json), [native validation](artifacts/native/result.json) and raw logs are included.
