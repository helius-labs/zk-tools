# Concurrent point-caller diagnostic

**The weighted-anchor benefit reverses under the tested SMT loads.** Borrowed signed G1 is about 1.8–2.3% faster alone or on two separate cores, but 1.5% slower on an SMT sibling pair and 1.35% slower across all 32 logical CPUs. This is evidence that a single-caller arithmetic win can fail under concurrent execution. It is not a new proof speedup, a complete explanation of the earlier SDK result, or an explanation of C7a's scaling. The weighted SDK candidate remains closed and unadopted.

The diagnostic links the exact frozen core/field/adapter libraries from the [parent caller study](../2026-10-28-g1-weighted-anchor/README.md). No Cargo/core/field build or arithmetic change occurs. The new Rust harness constructs private fixtures and scratch per OS thread and pins workers to explicit CPUs. Mode 0 and mode 1 both run ordinary arithmetic; mode 2 selects the same weighted formula tested previously. G2 is unchanged in all modes.

## Unprofiled measurements

The verified CPU layouts are solo [2], SMT siblings [2,18], separate cores [2,3], 16 physical cores [0–15], and all 32 logical CPUs [0–31]. Sysfs core/sibling records and exact worker affinity checks are archived. Fifteen rotated/reversed blocks × fifteen caller/layout contexts × three modes give 675 fresh processes. Each worker primes three checked/reset 1,024-call batches, then runs 8,192 complete kernel calls.

The main clock spans release of the start barrier through completion of the slowest worker at the done barrier. The primary metric is ns per parallel wave, not average per-worker latency. Per-worker loop clocks, thread CPU clocks, group throughput and process usage are retained. Setup, thread creation, output checks and serialization are outside all clocks; barriers prevent any worker's post-check allocation from overlapping another worker's loop.

| Caller | CPU layout | Ordinary 0 µs/wave | Weighted µs/wave | Weighted/ordinary 0 paired median | Marginal bounds | Wins |
|---|---|---:|---:|---:|---|---:|
| g1-owned | solo | 4.114401 | 4.064934 | 0.986722256 | [0.985541652, 1.000472940] | 11/15 |
| g1-owned | smt | 5.597746 | 5.629156 | 1.010809244 | [0.995785712, 1.031153416] | 7/15 |
| g1-owned | separate | 4.069378 | 4.103252 | 1.012125307 | [0.996443654, 1.013834182] | 5/15 |
| g1-owned | physical16 | 4.599406 | 4.619239 | 1.006126348 | [1.001131380, 1.010261682] | 3/15 |
| g1-owned | logical32 | 6.828873 | 7.028146 | 1.025236396 | [1.020480357, 1.037087505] | 0/15 |
| g1-borrowed-signed | solo | 7.002356 | 6.843246 | 0.981962735 | [0.976852012, 0.986184359] | 15/15 |
| g1-borrowed-signed | smt | 9.502941 | 9.661659 | 1.015249082 | [1.014501529, 1.018817788] | 0/15 |
| g1-borrowed-signed | separate | 7.033043 | 6.876540 | 0.976878898 | [0.976255067, 0.978920541] | 15/15 |
| g1-borrowed-signed | physical16 | 7.983315 | 7.832973 | 0.983873907 | [0.979999435, 0.992784331] | 12/15 |
| g1-borrowed-signed | logical32 | 11.728771 | 11.868672 | 1.013454423 | [1.004860439, 1.020333159] | 1/15 |
| g2-prefix | solo | 12.981515 | 12.975783 | 1.000382120 | [0.998616531, 1.001527357] | 7/15 |
| g2-prefix | smt | 20.256402 | 20.270049 | 0.999838394 | [0.997815316, 1.001665790] | 8/15 |
| g2-prefix | separate | 12.997019 | 13.011841 | 1.001072643 | [0.997677451, 1.003510379] | 4/15 |
| g2-prefix | physical16 | 15.135452 | 15.141582 | 1.000066707 | [0.997923459, 1.001145534] | 7/15 |
| g2-prefix | logical32 | 24.890655 | 24.906527 | 1.002025937 | [0.995781798, 1.003454478] | 6/15 |

| Caller/layout | Weighted/ordinary 1 paired median | Marginal bounds | Wins |
|---|---:|---|---:|
| g1-owned/solo | 0.994260624 | [0.992207380, 1.000611352] | 10/15 |
| g1-owned/smt | 1.002050727 | [0.993095052, 1.010784673] | 7/15 |
| g1-owned/separate | 1.007140694 | [0.995534316, 1.013716272] | 5/15 |
| g1-owned/physical16 | 1.006483992 | [1.000070081, 1.008153568] | 3/15 |
| g1-owned/logical32 | 1.024953612 | [1.017724932, 1.028841111] | 1/15 |
| g1-borrowed-signed/solo | 0.984305041 | [0.977757950, 0.988587538] | 15/15 |
| g1-borrowed-signed/smt | 1.017403369 | [1.013360565, 1.019509075] | 0/15 |
| g1-borrowed-signed/separate | 0.978281232 | [0.977208742, 0.979895827] | 15/15 |
| g1-borrowed-signed/physical16 | 0.980572864 | [0.967670288, 0.984483367] | 12/15 |
| g1-borrowed-signed/logical32 | 1.014601730 | [1.004284147, 1.017166794] | 2/15 |
| g2-prefix/solo | 0.994101683 | [0.991486591, 0.998951888] | 12/15 |
| g2-prefix/smt | 1.000630064 | [0.998697359, 1.002523383] | 5/15 |
| g2-prefix/separate | 1.000126854 | [0.999090641, 1.002489818] | 7/15 |
| g2-prefix/physical16 | 1.000965824 | [0.997574953, 1.002691294] | 5/15 |
| g2-prefix/logical32 | 1.000671303 | [0.995347075, 1.002205804] | 6/15 |

Bounds are ordered ranks 4–12 of 15 within-context paired ratios, nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. Tables show medians of process means, so ratios of table medians can differ from paired medians. All fifteen ordinary-label calibration bounds fit the predeclared factor 1.03, but this does not mean zero measurement bias: unchanged G2 solo mode 2/ordinary 1 has a roughly 0.59% lower estimate with bounds below parity. That observation remains included and is not a G2 speedup. There are no exclusions, sample extensions, successful-capture repeats, topology selections or adoption gates.

Borrowed G1 uses 352 pairs; owned G1 uses 200. Borrowing, signedness and geometry differ together and are not individually isolated. The borrowed benefit reverses on the sibling pair against both controls; both corresponding intervals lie above parity. It remains positive on separate cores and at physical16. Owned G1 at logical32 is about 2.5% slower with either control, while the solo improvement remains uncertain. Small positive estimates elsewhere remain reported.

SMT still improves aggregate throughput in these loops; a slower per-thread loop is not evidence that disabling SMT would improve the prover. Worker count also changes frequency, cache working sets, barrier/tail behavior and contention. This diagnostic does not identify an exclusive execution port, front-end bottleneck, cache level, DRAM bandwidth limit or complete SDK critical path. C7a has no SMT in the earlier measurements, so no AWS explanation follows from this local interaction.

## Counters and profiles

All 675 latency captures finish before eight FIFO-gated grouped perf-stat captures, followed by four separate 199 Hz cycles:u profiles. Counter/profile calls use 262,144 iterations per worker after the same primes. All twelve finish before decoding, hashing, compression or archive work. No owned helper/build/hash/archive/other benchmark overlaps capture. Host snapshots are retained; no exclusive-host claim is made.

| Borrowed G1 layout/mode | Cycles/kernel call | Instructions/kernel call | IPC | Generic cache misses/call |
|---|---:|---:|---:|---:|
| stat-logical32-m0 | 48900.152 | 85033.988 | 1.738931 | 42.603006 |
| stat-logical32-m2 | 49396.074 | 87367.988 | 1.768723 | 16.746566 |
| stat-physical16-m0 | 37876.859 | 85033.981 | 2.245011 | 0.014107 |
| stat-physical16-m2 | 36986.740 | 87367.981 | 2.362143 | 2.307073 |
| stat-smt-m0 | 49205.176 | 85033.989 | 1.728151 | 0.002865 |
| stat-smt-m2 | 50056.902 | 87367.989 | 1.745373 | 0.002930 |
| stat-solo-m0 | 37848.915 | 85033.986 | 2.246669 | 0.005375 |
| stat-solo-m2 | 36931.532 | 87367.986 | 2.365675 | 0.010956 |

All four counters run at 100% in every group. Counts are summed across active threads and divided by total kernel calls. Cycles on two SMT threads are logical-thread cycle counts, not disjoint physical-core cycles. These are one counter process per context, descriptive observations without confidence bounds. Weighted retires about 2.74% more instructions in every layout. Its measured cycles fall about 2.4% alone/physical16 but rise about 1.7% on the sibling pair and 1.0% at logical32. IPC alone is insufficient: it rises in both improving and regressing cases. This fits a latency/throughput tradeoff but does not prove a specific hardware cause. Generic cache-miss observations vary substantially at high concurrency, are not assigned to a cache level, and are not DRAM byte counts.

| Profile | Samples | Sampled worker TIDs | Sampled total TIDs | PIDs | Lost |
|---|---:|---:|---:|---:|---:|
| cycles-logical32-m0 | 19168 | 32 | 33 | 1 | 0 |
| cycles-logical32-m2 | 19415 | 32 | 33 | 1 | 0 |
| cycles-physical16-m0 | 6539 | 16 | 17 | 1 | 0 |
| cycles-physical16-m2 | 6460 | 16 | 17 | 1 | 0 |

Total samples: 51582. Every profile records all expected active worker TIDs plus the main control thread, one PID and zero lost records. Full symbol weights and decoded logs are archived; raw perf.data remains local with hashes. Loss-record dumps are archived losslessly with deterministic gzip. Sampled period is not a removable proof wall fraction.

## Functional and memory checks

One new unfiltered harness test exercises three callers × three modes × five layouts using short loops, independent full generator oracles, exact CPU affinity, barrier coordination, scratch reuse and zero timed allocations. Two inherited perf-helper tests also pass. The exact parent native 124-test core qualification is inherited through frozen libraries; no successful native core/field build or test is repeated.

All 687 latency/stat/profile processes pass full generator output checks and matching per-round output digests, with zero global timed allocations. They complete **110,518,272 timed kernel calls**, which are not SDK/Gnark proofs. Per-worker scratch capacities remain 40,000 bytes for owned G1, 70,400 for borrowed G1 and 28,160 for G2. The borrowed fixture reference vector is 5,632 bytes per worker, outside clocks. Multiplied scratch for 32 workers, harness barriers, fixtures and stacks are diagnostic allocations; these observations are not a production memory policy, proof RSS or RAM minimum. All production source and accepted executable hashes remain unchanged.

## Proof context and inherited comparisons

The preceding [full-SDK trial](../2026-10-28-g1-weighted-anchor-sdk/README.md) remains closed: accepted/OFF/ON primary32 medians 8.365439/8.361170/8.378203 ms, paired ON/OFF 1.003326 with bounds [0.996369,1.008874]. This diagnostic neither replaces those measurements nor rescues the candidate. It supplies a concrete reason to include concurrent callers when judging future arithmetic changes. It does not prove that SMT alone caused the SDK outcome.

The all-eight 10× CPU objective remains active and unmet, 0/8; primary target 3.7 ms. These accepted 172-source timings and supplied M5 references are inherited, not remeasured here. The accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.

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

Historical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB with no supplied M5 reference. No SDK/Gnark proof was generated; independent ledger remains 13,082. No new all-eight/54, portable/mobile/TEE, pressure, giant-circuit or AWS/Aeglos measurement is claimed. No cloud resource was created.

Next: judge future arithmetic candidates by actual work/instruction reduction and concurrent behavior as well as isolated latency. Audit previous arithmetic/reduction experiments before choosing a distinct operation-removal mechanism. Keep this candidate closed; no thread-count default, padding, code-layout or unchanged arithmetic reroll follows from the diagnostic. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [CPU topology](artifacts/topology.json), [library provenance](artifacts/library-provenance.json) and raw logs are included.
