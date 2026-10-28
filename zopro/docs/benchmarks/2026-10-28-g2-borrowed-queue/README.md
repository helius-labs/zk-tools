# Borrowed pending operands in G2 phi MSM

The candidate **fails the declared complete-MSM screen and is not adopted**. At30,089points/32workers, ON/OFF paired ratio is **0.899430758**, marginal interval [0.743087048,0.995865445], 22/31wins; ON/frozen is **0.964468720**, interval [0.779341112,1.049003679], 19/31wins. Production and accepted SDK remain unchanged.

## Change and scope

Ordinary G2 phi windows store borrowed16byte (encoded bucket index,point reference) entries instead of owned136byte point entries. Signs are retained in the index until bucket insertion/projective fallback or Y packing. The new window omits the two scalar denominator/prefix vectors that packed dispatch does not use. The signed borrowed prefix uses the accepted field operations, formula and Fq2AddScratch prefix/norm/tail storage. No field arithmetic, scalar decomposition, width12/parts2, key representation, persistent table/cache or execution plan changes. Phi still transforms temporary high-component bucket sums, not the input key.

A cfg-only dispatch selects this window after the unchanged special top-window branch. Unsupported packed/direct/canonical/vector/prefix options retain the old window. Existing identity/equal/opposite handling and collision/projective fallback remain. References borrow immutable key operands for the window lifetime.168 original private files remain exact; reverse edits recover all172 accepted files. An independent diagnostic frozen copy has170 exact originals and only two identical full-MSM export suffixes. Field source remains byte-exact, and its accepted qualification is inherited.

This differs from earlier borrowed sparse-group views and the accepted G1 borrowed queue: it replaces G2's internal pending batch and collision queue. Earlier packing/record negatives remain valid, and no large gain is inferred from source byte counts.

## Complete-MSM measurements

Ryzen9950X,31fixed rotated/reversed blocks,3point sizes×soloCPU2/physical16CPUs0–15/logical32CPUs0–31×frozen/OFF/ON. Each process builds public synthetic generator-derived bases and full-width scalars, shares a fixed block/size seed across variants, makes3checked primes, then7measured complete G2 phi MSMs. Timing includes queue fill/flush, scalar-input reset copy, field arithmetic, inverse, merging, weighted reduction, allocation and deallocation. The input reset is necessary because raw_exclusive consumes its words and is charged equally. Every full output is checked against a known generator-weighted scalar sum after timing. These are complete-MSM component measurements, not real proof or SDK latencies.

| Points | Workers | Frozen ms/MSM | OFF ms/MSM | ON ms/MSM | ON/OFF | Marginal interval | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| 4096 | solo | 11.791134 | 13.741849 | 11.740793 | 0.854921572 | [0.853764927,0.856097603] | 31/31 |
| 4096 | physical16 | 1.111088 | 1.250852 | 1.113476 | 0.893147182 | [0.883042394,0.909884598] | 29/31 |
| 4096 | logical32 | 2.335394 | 2.281027 | 2.300429 | 1.068137248 | [0.888061641,1.177528652] | 12/31 |
| 16384 | solo | 26.713576 | 36.904140 | 25.896301 | 0.703117013 | [0.702606583,0.703732159] | 31/31 |
| 16384 | physical16 | 2.279641 | 3.217780 | 2.191881 | 0.686954294 | [0.670545999,0.700433246] | 31/31 |
| 16384 | logical32 | 5.565089 | 5.492679 | 5.098280 | 0.873444197 | [0.782680097,1.071205886] | 19/31 |
| 30089 | solo | 42.408273 | 61.799660 | 40.870575 | 0.661089810 | [0.660385336,0.662080346] | 31/31 |
| 30089 | physical16 | 3.858426 | 5.738800 | 3.592712 | 0.622718366 | [0.615344762,0.631910865] | 31/31 |
| 30089 | logical32 | 6.887203 | 7.513792 | 6.575172 | 0.899430758 | [0.743087048,0.995865445] | 22/31 |

| Points/workers | OFF/frozen | Interval | ON/frozen | Interval |
|---|---:|---|---:|---|
| 4096/solo | 1.165129046 | [1.163959175,1.167341727] | 0.996635425 | [0.994144023,0.998059431] |
| 4096/physical16 | 1.117466212 | [1.101456587,1.134789978] | 1.001988378 | [0.977348120,1.013833750] |
| 4096/logical32 | 0.943013389 | [0.753166763,1.054812828] | 0.939902030 | [0.689573033,1.165993891] |
| 16384/solo | 1.379402843 | [1.378610358,1.380946300] | 0.969994425 | [0.969373993,0.970769587] |
| 16384/physical16 | 1.408810825 | [1.392373917,1.420777828] | 0.956497242 | [0.947677413,0.976180625] |
| 16384/logical32 | 1.026692903 | [0.879082004,1.246836639] | 0.880999770 | [0.740907108,1.127670765] |
| 30089/solo | 1.456495478 | [1.455361528,1.458477722] | 0.963415358 | [0.962024690,0.965285773] |
| 30089/physical16 | 1.480389974 | [1.460105272,1.507125448] | 0.931932747 | [0.913341239,0.935001223] |
| 30089/logical32 | 1.066417003 | [0.920528891,1.190886751] | 0.964468720 | [0.779341112,1.049003679] |

Calibration **false**, nonregression **false**, primary gain **false**. Calibration failures:["g2-msm/16384/logical32/1-over-0","g2-msm/16384/physical16/1-over-0","g2-msm/16384/solo/1-over-0","g2-msm/30089/logical32/1-over-0","g2-msm/30089/physical16/1-over-0","g2-msm/30089/solo/1-over-0","g2-msm/4096/logical32/1-over-0","g2-msm/4096/physical16/1-over-0","g2-msm/4096/solo/1-over-0"]; nonregression failures:["g2-msm/16384/logical32/2-over-0","g2-msm/16384/logical32/2-over-1","g2-msm/30089/logical32/2-over-0","g2-msm/4096/logical32/2-over-0","g2-msm/4096/logical32/2-over-1"].

All9OFF/frozen intervals must fit[1/1.03,1.03]; all18ON/frozen+ON/OFF upper ratios<=1.02. Primary30,089/logical32 requires upper<1 and>=21/31wins vsboth controls; legacy logical32_aggregate contains only this primary case. Ordered ranks10–22of31 provide nominal97.055%marginal coverage under independent exchangeable blocks, not familywise. No gain floor, excluded blocks, extensions or unchanged reroll. All positive estimates and uncertainty are retained.

## Separate requested-memory observations

Ordinary timing and PMU executables use the original uninstrumented allocator; no allocation counters run in those intervals.27separate resource processes enable requested-allocation accounting in a different executable, after3primes, around1MSM. Their elapsed times do not enter timing gates. Requested peak extra is the maximum tracked live requested bytes above the warmed fixture/pool baseline; it excludes allocator internals and is not physical memory, minimumRAM or a universal bound. Whole-processRSS includes fixtures, pool and primes. Counting code snapshots raw counters before constructing its JSON report.

| Points/workers | OFF peak extra MiB | ON peak extra MiB | OFF requests | ON requests | OFF/ON RSS MiB |
|---|---:|---:|---:|---:|---:|
| 4096/solo | 0.6391 | 0.6009 | 3104 | 3064 | 5.500/5.000 |
| 4096/physical16 | 6.1332 | 5.9411 | 3648 | 3608 | 12.000/10.750 |
| 4096/logical32 | 7.3264 | 6.5565 | 3648 | 3608 | 12.750/12.750 |
| 16384/solo | 0.6194 | 0.6094 | 4543 | 4503 | 10.250/10.250 |
| 16384/physical16 | 5.9123 | 5.6938 | 5083 | 5043 | 16.250/16.500 |
| 16384/logical32 | 7.3960 | 7.0727 | 5083 | 5043 | 17.750/18.250 |
| 30089/solo | 0.6127 | 0.6076 | 4935 | 4895 | 15.500/15.500 |
| 30089/physical16 | 6.6355 | 5.7172 | 5475 | 5435 | 22.500/21.500 |
| 30089/logical32 | 7.4639 | 6.1115 | 5475 | 5435 | 23.500/23.500 |

The resource dictionary retains the historical JSON field name capacities_per_worker, but entries here are whole-process resource probes, not per-worker capacities. All raw requested bytes, live-before/after and frozen observations remain available. Fewer queue payload bytes do not by themselves prove lower process peak or faster proofs. No key-sized cache is added.

## Correctness, controls and profiles

Native OFF passes119core tests; ON passes122including3new tests. Signed borrowed prefix coverage includes lengths0/1/7/8/9/15/16/32/200/352/511, sign patterns, tails and3changing rounds. Full-width phi MSM checks1024/4096/8192points against generator sums; adversarial1024/2048point cases include zero, identity, equal/opposite bases and scalar edges. Original tests cover unaffected paths; optional missing-fixture scenarios are not claimed. Three harness tests pervariant pass (one MSM/reset/affinity test and two inherited FIFO tests). One separately compiled exact heap-accounting test passes, without repeating the harness suite.

All runtime rlibs/proc macros are frozen from independent target roots and actual core/field flags/features checked. Before timing, complete.text/.rodata and all other runtime section bytes match frozen/OFF except individually decoded panic source-line fields+2 in g2_phi and build ID. No other control difference is accepted. Static IFMA/disassembly counts are recorded, not dynamic work or spill/stack bounds. Initial prefix generation expected two index sites but found three; its repair masks all three including scatter before builds, preserving initial generator and receipts. Any other failures/repairs remain recorded. No successful tests or captures are repeated for bookkeeping.

All837latency processes finish before4FIFOgrouped perf-stat, then4FIFO199Hzcycles profiles(30,089points,OFF/ON,16/32workers,256MSMs after3primes), then12owner/decode/loss commands. After these,27resource processes run separately. There are7907timed complete MSM calls in ordinary/PMU captures plus2535primes; resource probes add108diagnosticMSMs. These counts exclude native/harness test internals and are not proofs. No owned build/helper/hash/archive/otherbenchmark overlaps captures. Host snapshots do not establish exclusive use.

| Counter case | Cycles/MSM | Instructions/MSM | IPC |
|---|---:|---:|---:|
| stat-g2-msm-30089-logical32-m1 | 454245771.9296875 | 606284264.203125 | 1.3347053548293049 |
| stat-g2-msm-30089-logical32-m2 | 341681094.69921875 | 618952413.0195312 | 1.8114915417383393 |
| stat-g2-msm-30089-physical16-m1 | 363043566.62890625 | 584390596.3515625 | 1.6096982568180624 |
| stat-g2-msm-30089-physical16-m2 | 242916669.03515625 | 598076733.7382812 | 2.4620654322068125 |

All event groups100% running: **true**. Counters do not isolate RAM bandwidth, an execution port or removable proof latency. SMT cycles sum logical-thread events.

| Profile | Samples | Sampled pool workers | Lost |
|---|---:|---:|---:|
| cycles-g2-msm-30089-logical32-m1 | 3585 | 32 | 0 |
| cycles-g2-msm-30089-logical32-m2 | 2680 | 32 | 0 |
| cycles-g2-msm-30089-physical16-m1 | 3586 | 16 | 0 |
| cycles-g2-msm-30089-physical16-m2 | 2541 | 16 | 0 |

Total12392 samples; one PID/profile and zeroLost. Actual sampled worker coverage is reported, without assuming every configured worker receives a sample. Full symbol maps are retained. Rawperf/executables stay local with hashes;4loss dumps use deterministic gzip and byte-exact round-trip verification. Captured logs/measured source bytes, including trailing blank lines, are preserved; authored helper/report whitespace checked separately.

The exact borrowed-queue implementation is closed without a full SDK trial or adoption. No unchanged queue/layout reroll is selected.

## Inherited CPU and Metal comparison

No new full-proof latency/RSS is established. Accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The all-eight CPU10x objective remains active and unmet,0/8. These accepted local figures and supplied M5 references are inherited.

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37.0 | 24.0 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59.0 | 30.0 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109.0 | 48.0 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189.0 | 74.0 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284.0 | 101.0 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335.0 | 124.0 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528.0 | 176.0 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507.0 | 175.0 | 7.790× | 2.689× |

Historical primary134.50MiB; merge54 remains161.341835ms/1910.1875MiB without a supplied M5 reference. No new SDK/Gnark proof, mobile/TEE, minimumRAM, memory-pressure, giant-circuit or AWS/Aeglos result. Proofledger remains13,181.

The memory-band paper reviewed during selection was already tested in an earlier experiment; its BLS12-381 results are not borrowed-queue or Zopro speed evidence ([Noël/Plantard2026](https://link.springer.com/article/10.1007/s13389-026-00393-z), [prior negative bands](../2026-10-25-g1-sequential-bands-component/README.md)).

[Design](artifacts/design.md), [new window](artifacts/g2_borrowed_window.rs), [new signed prefix](artifacts/g2_borrowed_prefix.rs), [analysis](artifacts/analysis.json), [native suites](artifacts/native/result.json), [harness checks](artifacts/linked/result.json), [all timing/PMU](artifacts/runs/result.json), [resource probes](artifacts/resources/result.json), [source audit](artifacts/source-audit.json), [control audit](artifacts/control-locations.json).

## Positive estimates and unresolved control discrepancy

The large16worker case improves37.73% against OFF (31/31wins) and6.81% against frozen (29/31wins, interval0.91334–0.93500). The corresponding solo case improves33.89% against OFF. At32workers, ON/OFF improves10.06% (22/31wins, interval0.74309–0.99587), while ON/frozen improves3.55% with interval0.77934–1.04900 and19/31wins. These positive observations remain in scope; the declared complete gate fails and no proof gain is established.

Frozen/OFF code and constants are byte-identical, yet the30,089point solo OFF/frozen ratio is1.45650 with a narrow1.45536–1.45848 interval; physical16 is1.48039 with interval1.46011–1.50713. This cannot be dismissed as ordinary zero-mean timing noise from these observations. Whole-MSM requested allocation counts/bytes agree between those controls. Source audit identifies only panic-line/build-ID byte differences, and the mode argument labels output only. Executable backing/addresses, process/allocator layout and runtime scheduling remain unresolved possibilities, not established causes. The large fresh-control gain is therefore not a reliable estimate of the optimization's isolated benefit.

The primary resource-only probe reports7,826,504 versus6,408,372peak extra requested bytes: about1.35MiB less with borrowing. Both OFF/ON processes reach23.5MiB RSS in that observation. Requested bytes perMSM drop from33,031,031 to31,559,031 and allocation requests5475→5435, consistent with removing unused buffers and smaller entries. These are single diagnostic resource observations, not minimumRAM or actual proof peaks.

The16worker counter observations show584.39M→598.08M instructions/MSM and363.04M→242.92M cycles/MSM. At32workers they show606.28M→618.95M instructions and454.25M→341.68M cycles. Thus the sampled improvement does not consist of fewer retired instructions. Kernel profiles locate the borrowed path and retain full symbol weights; no exclusive memory-bandwidth or spill cause is inferred.

The next distinct experiment will diagnose the byte-identical control discrepancy using unchanged binaries, mode labels and equal-length executable aliases. This is a control-identity diagnostic, not a rerun of the optimization gate, and it does not promote this candidate. Resolve the control problem before deciding whether a separately declared full-proof diagnostic is warranted.
