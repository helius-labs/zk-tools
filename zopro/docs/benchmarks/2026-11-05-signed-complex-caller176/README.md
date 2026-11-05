# Signed complex square in the complete G2 caller

The complete borrowed-G2 caller improves by **0.646% across all six contexts** versus fresh OFF, paired ratio **0.993537 [0.990583, 0.996525]**, 13/15 wins. Its required 32-worker aggregate is inconclusive: **0.998977 [0.992959, 1.002779]**, 11/15 wins. Against FROZEN, the 32-worker estimate is **1.001039 [0.996868, 1.002399]**, 4/15 wins.

The full gate fails exactly those two 32-worker gain checks. All six calibration checks, all twelve candidate nonregression checks and both all-six aggregate gains pass. **The implementation is not adopted or advanced to an SDK trial.** Its positive aggregate estimate is retained; the result does not prove a universal slowdown. Production176 and SDK6d83 remain unchanged, and the all-eight CPU10× goal remains unmet, 0/8.

## Change and correctness

The [qualified signed-square helper](../2026-11-05-signed-complex-square176/README.md) replaces only the real component of the native FAST borrowed-G2 reverse-loop square. It evaluates `a²−b²` using symmetric raw products and signed Montgomery cancellation. The imaginary product/doubling is unchanged. The ordinary Fq2 basis, field representation, forward norms, shared inversion, slope/final-Y arithmetic, scalar tails, points, queues, grouping and scheduling remain original.

The helper is byte-exact to the qualified parent. Its model/native field qualifications are inherited, not rerun. The caller OFF expression remains byte-for-byte original under the inverse cfg; FAST=false and unsupported paths retain original arithmetic. This avoids the refactored OFF control problem documented in the earlier [canonical-Xsum trial](../2026-10-28-g2-canonical-xsum/README.md).

The isolated snapshot has178files:171 original files exact, five reversible parents and two children. All176 original bytes recover; generated field code and generator are unchanged. ON passes129unfiltered core tests, including complete borrowed sign-mask/tail/reuse/thread-pool cases. OFF coverage is127initial successes plus two repaired exact tests. The initial OFF run omitted adapter runtime options, causing those two configuration assertions to fail. The runtime source/flags were unchanged; only the two failed tests were repeated with explicit options, followed by the not-yet-run ON suite. No successful parent model/field test or caller capture was repeated.

The coherent native libraries are frozen separately. Actual final link argv contains `-O`, `target-cpu=native`, `lto=fat`, `codegen-units=1`. The original accepted frozen adapter is reused after its source/library/qualification receipt checks. Original/fresh/candidate identities and complete commands are included.

## Fixed complete-caller comparison

Ryzen9 9950X,16physical/32logical CPUs. Solo usesCPU2, physical16 uses0–15, logical32 uses0–31. Workers pin individually; coordinator remains unpinned. These are independent kernel workers using synthetic valid ordinary curve inputs, not a full MSM, proof, Rayon proof schedule or concurrent-proof throughput result.

Eighteen32-call preflights cover200/352pairs × three topologies × FROZEN/OFF/ON. Tails are qualified by core tests, not these fixed-size preflights; the pre-capture declaration clarifies the earlier design phrase. Fifteen rotated/reversed blocks time8192calls/worker after three checked1024-call primes. The common timer includes start/done synchronization and complete call loops, including inversion, packing and scatter. Generator-scalar output validation occurs after all worker clocks. All output/capacity signatures agree, and all timed allocation counts are zero. Input points use the original immutable borrowed API.

| Pairs | Workers | FROZEN µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF [bounds] | Wins/15 |
|---|---:|---:|---:|---:|---|---:|
| 200 | 1 | 7.856265 | 7.858441 | 7.785619 | 0.992467 [0.982523, 0.999244] | 12 |
| 200 | 16 | 9.048361 | 9.040074 | 9.000163 | 0.990638 [0.986749, 0.995029] | 12 |
| 200 | 32 | 14.684698 | 14.753824 | 14.663177 | 0.998300 [0.988327, 1.003331] | 10 |
| 352 | 1 | 13.293323 | 13.290761 | 13.173640 | 0.989605 [0.985899, 0.997493] | 13 |
| 352 | 16 | 15.461016 | 15.400481 | 15.283845 | 0.992881 [0.987600, 0.999632] | 12 |
| 352 | 32 | 25.165855 | 25.233665 | 25.195422 | 1.000162 [0.993753, 1.001841] | 6 |

Displayed latency is the median of15wave observations; paired ratios are not ratios of those medians. All-six versus FROZEN is **0.993342 [0.991075, 0.995886]**, 13/15 wins. Both all-six gains pass, while both 32-worker aggregates cross parity. Keep every block, including unfavorable observations.

Required calibration intervals fit[1/1.03,1.03]; candidate upper bounds must be≤1.03; both all-six and logical32-two equal-log aggregates require upper<1 and≥10/15wins against both controls. Bounds are ordered ratios4–12of15, nominal96.484375%marginal under independent/exchangeable-block assumptions, without a familywise guarantee. No gain floor, exclusions, extension, changed threshold or favorable-topology adoption was used.

Capture contains296caller processes:18preflights,270latencies, four stats and four profiles. There are36,126,720measured latency kernel calls,62,088,384measured calls including preflight/diagnostics,15,040,512priming calls and77,128,896total calls. These are not proofs. All317capture commands complete on the first capture attempt; no owned compilation/helper/hash/archive/verifier/second benchmark overlaps recording.

## Diagnostics and memory observations

All latency precedes four grouped PMU panels, then four fixed16Mcycles:u profiles. Stats use8192calls/worker and profiles262144, after the same primes. FIFO gates exclude setup/primes and final validation; loop/synchronization/accounting edges remain. Each counter panel reports100% running coverage. Values below are per complete200-pair batch, divided by workers×rounds:

| Workers | OFF instructions | ON instructions | OFF cycles | ON cycles |
|---|---:|---:|---:|---:|
| 16 | 111,169.509 | 112,796.522 | 41,462.068 | 41,110.662 |
| 32 | 111,169.547 | 112,796.548 | 60,870.864 | 60,946.576 |

ON executes about1.46%more instructions in each diagnostic panel. These single panels have no repeated-panel confidence intervals and do not isolate why topology estimates differ. Cache-miss counts remain in raw data; they are not a cache-level/DRAM/port explanation.

Linked borrowed-kernel static counts are FROZEN/OFF1450IFMAs and8587instruction lines, versus ON1460IFMAs and8651lines. Equal counts do not prove binary equivalence. Static counts are not executed counts, spill traffic, stack bounds or a causal latency model.

| Profile | Samples | Unknown period | Sampled workers | Qualified |
|---|---:|---:|---:|---|
| 16 OFF | 10,874 | 0.0460% | 16 | yes |
| 16 ON | 10,757 | 0.0372% | 16 | yes |
| 32 OFF | 31,699 | 0.0568% | 32 | yes |
| 32 ON | 31,627 | 0.0474% | 32 | yes |

All84,957samples retain exact executable/PID/FORK membership, workerTID/CPU, user event/period and zero-loss checks. All workers are sampled; no nonworker sample occurs. Attribution thresholds are≥1000samples and≤5%unknown. Symbol shares include inlined payload; absolute period coverage, local CPI and exclusive removable wall costs are unvalidated.

Reported scratch capacity per worker remains16,000bytes at200pairs and28,160bytes at352, plus3,200/5,632fixture reference bytes. This is not all live memory. Maximum process RSS across all15latency processes at32workers is OFF/ON4.00/4.00MiB for200pairs and11.25/11.00MiB for352. Counters include fixture/setup/allocator behavior, can undercount, and do not establish lifetime workspace peak or minimum RAM. There is no new source-level heap/scratch allocation; stack/resource/pressure/mobile qualification remains unperformed.

## Independent review and decision

The independent Rust reviewer imports no campaign or parser code. It checks601pins,326successful workflow commands plus the preserved failed native command, stream hashes, protocols/defaults, original source recovery, exact qualified field child, original OFF expression, native flags/frozen libraries/links, all270integer timing observations, all paired gates/aggregates, capacities/CPU/RSS, and every profile sample's ownership/CPU/period/unknown/loss accounting. Accepted SDK and29,953-entry proof ledger remain unchanged.

Preparation failures remain explicit: the two OFF adapter option assertions; a pre-workflow pin-helper ownership compile error repaired by borrowing its path; and a first link-workflow output-directory collision with the helper binary, before any link command ran. The unchanged link plan executes once in `link-runs`. The initial independent reviewer omitted normalization of absent `unfiltered:false`; only that reader default was repaired, preserving its source/log. No successful test, link or capture was repeated to repair bookkeeping.

Close this fixed caller implementation at the failed full gate. Its arithmetic remains qualified and its small positive aggregate remains evidence. Do not claim it slower everywhere, adopt a favorable worker subset, retime it unchanged, or use the square alone as proof of a full-proof gain. A new experiment needs a distinct reduction of dominant work.

## Unchanged accepted full-proof comparison

Inherited accepted32-worker local measurements and supplied M5 references, not measurements of this candidate. Source: [accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Different machines do not establish equal hardware efficiency. The latest separate full-SDK experiment remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), with its positive primary estimate and failed full gate retained.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54x1 remains161.883899ms without a supplied M5 reference. No SDK proof/Gnark export, AWS/Aeglos, giant-circuit, phone/ARM/TEE, pressure/swap/minimum-RAM or concurrent-proof result is added. Goal remains active and unmet,0/8.

## Artifacts

[Protocol](artifacts/capture-design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source edits](artifacts/edits.json), [reproduction notes](artifacts/reproduce.md), [artifact index](artifact-index.json), [checksums](SHA256SUMS).

The gzip JSON receipt archive preserves relative member paths, exact UTF-8 contents, byte lengths and SHA256s, with exact extraction verified. Sources, original failures, native/link/capture receipts, decoded profiles/metadata/events/loss and linked assembly are included. Executables, libraries, raw perf.data and private key/image/witness/input arrays are excluded. Synthetic fixture generation remains in the source.
