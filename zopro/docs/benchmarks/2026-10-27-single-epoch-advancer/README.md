# One nonblocking epoch-advancement scanner

The local speed screen fails; the candidate is not adopted. Primary32 is 8.327 ms ON versus 8.302 ms accepted. The paired ON/OFF saving estimate is 16.822 microseconds, with an interval spanning parity; the 16-worker control fails the nonregression bound. The original all-eight CPU 10× goal remains active and unmet, 0/8.

The private Crossbeam-epoch 0.9.21 candidate allows one participant-list advancement scanner per collector. A contending caller returns an observed epoch without waiting or scanning, and can still collect already expired bags. The original scan, epoch store, queue operations, pin frequency and collection expiration checks remain. Failed permit acquisition retains the original epoch-load ordering and SeqCst fence. A scoped permit releases on every exit and before the normal deferred-destructor loop. Explicit `ZOPRO_CPU_SINGLE_EPOCH_ADVANCER=1`, read at collector construction, enables the mode; OFF/default and no-std use the baseline path. One padded flag and immutable policy add fixed collector metadata; there are no key tables or proof buffers.

This targets redundant concurrent collector scans, separately from prior empty-victim, guard batching, collection-interval, retained-buffer and bounded-victim-search trials. Their failed or partial results remain. The [current-source C7a profile](../2026-10-27-pmu-c7a-aeglos/README.md) motivates the hypothesis but is not a measurement of this candidate. No fresh AWS or Aeglos comparison occurs here.

## Complete SDK latency

Ryzen 9950X, nine fixed rotated/reversed fresh-process blocks. Accepted uses the exact retained 172-file source SDK; OFF/ON share the new executable. All nine shapes use 32 workers on CPUs 0–31, plus primary with 16 workers pinned to CPUs 0–15, one logical CPU per physical core. Primary32 has 151 measured calls after three primes, primary16 has 251, others 31. Loading/primes are excluded; fresh solving, randomness, proving, verification and serialization are timed. No owned build/helper/hash/archive or second benchmark overlaps SDK/profile requests.

Times are medians of process medians. Ratios pair corresponding blocks and need not equal ratios of displayed medians.

|Case|Accepted ms|OFF ms|ON ms|ON/accepted paired|ON/OFF paired|
|---|---:|---:|---:|---:|---:|
|1x2-w32|8.302345|8.335117|8.327132|1.000548|0.997988|
|2x2-w32|11.412078|11.470458|11.392968|0.997128|0.991249|
|4x2-w32|15.914125|15.934471|15.960166|0.998684|0.994974|
|8x2-w32|24.009335|24.029863|23.918371|0.997680|0.994763|
|12x2-w32|33.924576|33.563598|33.868743|1.001514|1.005938|
|16x2-w32|39.901053|39.808306|39.723730|0.993294|0.996520|
|24x2-w32|66.315940|66.540656|66.531005|1.003243|1.001166|
|merge-24x1-w32|65.051242|65.444828|65.177690|1.003784|0.998263|
|merge-54x1-w32|162.670614|160.943732|162.066337|0.999593|1.006975|
|1x2-w16|9.970658|10.022468|10.270333|1.029167|1.025768|

The declared gate uses individual distribution-free median intervals with endpoints 2..8 over nine blocks (96.09375% nominal coverage under independent/exchangeable blocks; no familywise guarantee). Calibration intervals must lie wholly within factor 1.03 for OFF/accepted in all 10 cases; ON nonregression upper bounds must be ≤1.02 against both controls in every case; primary32 strict gain upper bounds must be <1, and original-eight block-geometric aggregate upper bounds ≤1. No minimum gain floor, exclusions, extensions or failed-bound waiver. Failed checks: calibration **1**, nonregression **2**, primary gain **2**, aggregate **2**. Positive estimates and uncertainty remain in [analysis](artifacts/analysis.json).

## CPU work and memory

CPU is median summed user+system time per timed request; RSS is maximum whole-process high water across ordinary latency processes, including load/primes. Neither isolates useful arithmetic or minimum RAM.

|Case|Mode|CPU ms/request|Peak RSS MiB|
|---|---|---:|---:|
|1x2-w32|accepted|190.374|137.50|
|1x2-w32|off|190.378|137.00|
|1x2-w32|on|187.823|134.00|
|1x2-w16|accepted|124.272|92.50|
|1x2-w16|off|124.764|90.50|
|1x2-w16|on|123.678|91.00|
|merge-54x1-w32|accepted|4447.590|1825.69|
|merge-54x1-w32|off|4412.347|1817.44|
|merge-54x1-w32|on|4422.988|1867.46|

## Separate warm profiles

Six 199 Hz user-cycle profiles follow all latency: primary32 uses 251 measured calls after three primes, merge54 uses 17. Zero lost records and one sampled SDK PID are checked. Flat shares have symbol/inlining limits and are not removable wall time or causal synchronization savings.

|Case|Mode|Samples|Epoch %|Scheduler %|
|---|---|---:|---:|---:|
|1x2-w32|accepted|8978|9.88|4.18|
|1x2-w32|off|9490|10.33|3.91|
|1x2-w32|on|9469|8.59|4.20|
|merge-54x1-w32|off|14639|1.84|1.46|
|merge-54x1-w32|on|14961|1.43|1.98|
|merge-54x1-w32|accepted|14568|2.21|1.96|

## Supplied M5 references

Historical supplied figures, not fresh matched-machine M5 measurements. Merge54 has no supplied reference.

|Shape|CPU reference ms|Metal reference ms|ON ms|CPU/ON|Metal/ON|
|---|---:|---:|---:|---:|---:|
|1x2|37|24|8.327132|4.443×|2.882×|
|2x2|59|30|11.392968|5.179×|2.633×|
|4x2|109|48|15.960166|6.830×|3.007×|
|8x2|189|74|23.918371|7.902×|3.094×|
|12x2|284|101|33.868743|8.385×|2.982×|
|16x2|335|124|39.723730|8.433×|3.122×|
|24x2|528|176|66.531005|7.936×|2.645×|
|merge-24x1|507|175|65.177690|7.779×|2.685×|

## Validation and limits

The candidate preserves the original advancement body byte-for-byte after removing inserted gate code. The [runtime lock audit](artifacts/lock-audit.json) confirms 218 of 219 package entries are identical; only epoch loses its registry source/checksum for the private path. Eight directly linked and four actual transitive scheduler/collector libraries are pinned. A [preparation repair](artifacts/library-discovery-repair.json) selects those four from Cargo JSON rather than assuming directory cardinality identifies runtime artifacts.

The first epoch suite passed 43 tests and failed one new test because its setup overlooked the first pin's automatic collection. The [test-only repair](artifacts/test-repair.json) acquires the test permit before pinning; that targeted test then passes, covering 44 tests across two invocations without repeating 43 successful checks. Added checks cover nonblocking contention/unwind release, skipped advancement, pinned-reader retention/release, and 8,192 concurrent deferred callbacks reclaimed exactly once. Alloc/no-std check, 119 native core, two raw and six cubic tests pass with mode ON. Field source is unchanged; inherited 33-field-test qualification is not a new run. These native tests do not prove the full memory model, fairness, bounded retention with stalled threads, mobile behavior or a universal progress bound.

All nine OFF/ON four-worker preflights complete 72 SDK calls and 18 exports. Latency/profiles add 19,182 calls and 276 exports: **19,254 SDK calls, 294 distinct independently Gnark-verified exports, 588 proof/public members**. Exports are 164 bytes, public-fixture-bound and fresh against 12,173 prior/new identities. Ledger **12,173→12,467**. No successful SDK capture or verifier is repeated for repairs. No portable whole-prover, concurrent SDK, pressure, 250/4999, phone, TEE, or new AWS qualification occurs. A passing speed screen would still require resource/compatibility review before promotion.

Primary ordinary CPU time falls from 190.374 to 187.823 ms/request, about 1.34% descriptively, without a demonstrated latency gain. Separate profiles show lower epoch share but more voluntary switches; merge54 profiled CPU time also increases. [Detailed interpretation](artifacts/review.md) retains positive estimates, control drift, mixed memory peaks and limits. These observations do not isolate a hardware cause. The [accepted CPU/Metal comparison](../2026-10-27-clean-fft-adoption/README.md) remains the production reference.

[Declaration](artifacts/design.md), [source audit](artifacts/source-audit.json), [raw rows](artifacts/performance/rows.json), [proof inventory](artifacts/proof-inventory.json), [build qualification](artifacts/native-qualified/result.json), [artifact hashes](artifact-index.json). Rawperf.data and binaries/compiler caches remain local with hashes. Production remains unchanged.
