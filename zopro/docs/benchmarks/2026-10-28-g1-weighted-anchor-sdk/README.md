# G1 weighted Y anchor in full SDK proofs

The candidate **does not pass the primary SDK screen and is not adopted**. At 32 workers, ON/OFF paired median is **1.003325660**, marginal bounds [0.996369104, 1.008873901], 7/15 wins. ON/accepted is **1.002472039**, bounds [0.996434554, 1.009267682], 5/15 wins. All calibration and physical-16 nonregression checks remain in the decision. Production and the accepted executable are unchanged.

The [caller screen](../2026-10-28-g1-weighted-anchor/README.md) measured about 1.24% aggregate improvement, with larger borrowed-signed than owned-unsigned gains. This study carries the exact runtime formula into pure libraries without diagnostic adapters. With S=Ax+Bx, T=2Ax+Bx and U=lambda², store T in the existing Ax record slot and compute X=U−S, Y=lambda(T−U)−Ay. One extra general vector add per eight pairs shortens the Y dependency chain. No field arithmetic, heap buffer, key table, assembly, new unsafe operation or public runtime option is added.

## Complete verified requests

Fifteen fixed rotated/reversed blocks compare accepted SDK14da, fresh ordinary OFF and weighted ON at 32 logical and 16 physical workers. Every latency process has three primes and 151 timed complete requests. Three four-worker preflights precede latency. Image loading is outside clocks; witness construction, constraint solving, fresh randomness, proving, internal verification and output serialization are included. A fresh host snapshot is recorded. The ceremony job present in the parent component study had ended before the SDK capture; no exclusive-host claim is made.

| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---|---:|---:|---:|---:|
| 32 | control | 8.365439 | 191.123351 | 129.75 | 134.00 |
| 32 | off | 8.361170 | 191.289517 | 129.00 | 135.50 |
| 32 | on | 8.378203 | 191.179662 | 131.50 | 135.00 |
| 16 | control | 10.122553 | 124.808152 | 86.25 | 88.50 |
| 16 | off | 10.031662 | 124.894060 | 85.50 | 88.75 |
| 16 | on | 10.097576 | 124.317576 | 85.50 | 90.25 |

| Workers | Paired ratio | Median | Marginal bounds | Wins | Check |
|---|---|---:|---|---:|---|
| 32 | off/control | 1.000065689 | [0.994116510, 1.003588046] | 7/15 | pass |
| 32 | on/control | 1.002472039 | [0.996434554, 1.009267682] | 5/15 | fail |
| 32 | on/off | 1.003325660 | [0.996369104, 1.008873901] | 7/15 | fail |
| 16 | off/control | 0.998218461 | [0.964531399, 1.005767252] | 8/15 | fail |
| 16 | on/control | 1.000417259 | [0.965888255, 1.030347316] | 7/15 | fail |
| 16 | on/off | 1.006047599 | [0.997964994, 1.036017197] | 5/15 | fail |

Bounds are ranks 4–12 of 15 paired process medians, nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. Fresh OFF/accepted calibration must fit [1/1.03,1.03] for both worker counts. Primary ON upper bounds must be below parity with at least 11/15 wins against both controls; physical-16 ON upper bounds must be at most 1.02 against both. There is no gain floor, exclusion, schedule extension, successful-capture retry or unchanged reroll. Failed checks: `["w32/on/control","w32/on/off","w16/off/control","w16/on/control","w16/on/off"]`.

At 32 workers, accepted/fresh OFF/ON medians are 8.365439/8.361170/8.378203 ms. The paired ON/OFF estimate is 0.333% slower, with bounds crossing parity; this is no verified proof gain. At 16 workers the raw ON median is 24.977 µs below accepted, but its paired median is slightly slower and the calibration/nonregression bounds fail. That small positive raw-median observation is retained without treating it as a qualified gain. Median CPU work falls from 191.289517 to 191.179662 ms/request at 32 workers, and from 124.894060 to 124.317576 at 16, descriptive reductions of about 0.057% and 0.462%. Median peak RSS at 32 is 129.00 MiB OFF versus 131.50 ON; maxima are 135.50/135.00 MiB. No RAM reduction is established.

The parent component used one active caller on one logical CPU; full proofs use parallel work and a different batch mix. Shortening a dependency chain at the cost of more instructions may behave differently when execution resources are shared or saturated. This is a hypothesis, not a demonstrated SMT, port-pressure or scheduler cause. The full-proof data justify closing this exact candidate; the component improvement remains valid within its measured scope.

## Correctness and profiles

Both native core suites pass 119 tests, including required solver-prefix, refined-component and invalid-witness cases. Both configurations pass two raw-image tests and six cubic prover tests. The parent native 124-test suite covers the new vector formula and alternating reused scratch; field source is unchanged and its previous tests are inherited. Some optional external fixture tests return when fixtures are absent; those external cases are not claimed. The standard SDK and independent Gnark checks exercise the actual 1x2 solver/prover.

The private source has 171 exact accepted originals and only msm.rs runtime edits. Diagnostic adapters/reexports are absent. Reversing the edits recovers all 172 accepted bytes. OFF and ON share the exact same candidate source; only the private native compile cfg differs. Separate target roots, immediate immutable copies of all transitive rlibs/proc-macro objects, native flags/features and the standard eight-library xtask link including memmap2 are checked. Distinct field hashes reflect compile flags/metadata and are not a field-arithmetic change.

All 90 latency processes precede six FIFO-gated warm 199 Hz cycles:u profiles, each 251 requests after three primes. All 99 captures finish before all 99 independent Gnark verifications, followed by profile decoding, hashing and archive work. No owned build/helper/hash/archive/second benchmark overlaps capture. Profiles have one sampled SDK PID each and zero lost records. The point-caller matcher covers stream_g1 and BatchAdd symbols; it does not include all inlined point work and is not an exclusive opcode cost or removable proof wall fraction.

| Workers | Variant | Samples | Lost | Matched point-caller period |
|---|---|---:|---:|---:|
| 32 | control | 9075 | 0 | 18.469% |
| 32 | off | 9106 | 0 | 18.928% |
| 32 | on | 9143 | 0 | 19.351% |
| 16 | control | 5466 | 0 | 22.434% |
| 16 | off | 5837 | 0 | 22.880% |
| 16 | on | 5637 | 0 | 20.261% |

Total samples: 44264. Raw perf.data stays local with hashes; decoded observations and full symbol weights are archived.

All **15,396** SDK calls verify internally. All **99** fresh exported 164-byte proofs independently pass Gnark and image/public binding and are disjoint from 12,983 prior proof identities, advancing the ledger to **13,082**. Proof/public members are archived. Process peak RSS includes loading and primes; it is not minimum RAM or memory-pressure qualification.

## Accepted comparisons (inherited)

These are accepted 172-source measurements, not this candidate's all-shape timings. The all-eight 10× CPU objective remains active and unmet, 0/8; primary target is 3.7 ms. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.

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

Historical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB, with no supplied M5 reference. No new all-eight/54, portable/mobile/TEE, whole-proof concurrency, pressure, giant-circuit or AWS/Aeglos result is claimed. Latest AWS measurements predate the accepted FFT source. No cloud resource was created.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [native qualification](artifacts/native/result.json) and raw logs are archived.

The exact candidate is closed at the SDK gate. Retain the component gains, full-proof positive estimates and every failed check. Do not rerun unchanged arithmetic, flags or layout to search for a pass.
