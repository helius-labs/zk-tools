# Direct inverse finish in addition callers

The candidate **passed actual addition-caller screen; full SDK qualification required**. This measures complete G1/G2 batch-addition callers, not full proving latency. Production and the accepted SDK are unchanged.

The paired six-case median improvement is 0.081% versus frozen and 0.036% versus fresh OFF. The corresponding intervals are [0.998935197, 0.999593231] and [0.999263312, 0.999946518], with 29/31 and 23/31 aggregate wins. Every calibration and individual nonregression check passes. The gain is small and the fresh comparison is close to parity; this permits a full-SDK test without predicting its result.

The [parent arithmetic study](../2026-10-28-inverse-direct-finish/README.md) checks a signed 320-bit reduction that removes the inverse finish's scale-restoring field multiplication. Its primitive timing lacked an OFF control because Cargo's top-level library was overwritten; that schedule remains unqualified and was not rerun. This study tests a different boundary with audited libraries: G1 owned unsigned, G1 borrowed signed, and G2 prefix/norm additions, each at 200 and 352 entries. These sizes come from the real primary batch census, but points and destination patterns are synthetic ordinary generator fixtures. This does not measure collision queues, routing, a complete MSM, or a witness distribution.

The field arithmetic matches the parent source exactly. The frozen baseline retains accepted field source; fresh and selected share the candidate source, with the selection cfg present only for selected. No stream arithmetic body changes. Two original core files add diagnostic module/reexports; 170 original frozen files and 169 original fresh files remain exact. The borrowed adapter prepares a reference vector outside the timer and calls the original borrowed signed stream. This vector is diagnostic input storage, not a proposed new production buffer. Arithmetic scratch capacities are checked equal across variants.

Libraries are frozen immediately after each build into create-new content-addressed paths before a later build can overwrite Cargo outputs. All probe links search only frozen dependencies. Flag sets, field-library hashes, executable hashes and complete copies are checked before capture. The [identity audit](artifacts/control-identity-audit.json) verifies distinct OFF/ON fields and probes. A regression test overwrites a simulated mutable Cargo library and confirms the earlier frozen OFF bytes survive.

The two OFF executables are byte-identical, while selected ON differs. They provide separate process observations, not independent machine-code controls. The first probe link failed because the frozen set omitted proc-macro shared libraries. Those artifacts were added from the saved Cargo receipts, with separate hashes, before linking and timing. No successful build, test or measurement was repeated. Original failure logs and the supplement helper are retained.

Three adapter tests pass against frozen libraries. Fresh and selected each pass the complete 122-test native core suite, including the adapters and existing complete-curve tests, with serial test execution. Tests cover independent generator sums, owned and borrowed signs, empty/partial batches, 199/200/351/352 boundaries, all bucket outputs and repeated scratch reuse. The parent's 34-test OFF/ON field qualification and 68,672 independent signed-residue cases are inherited, not rerun or recounted as new cases.

All 558 latency processes complete in a fixed 31-block rotated/reversed schedule on CPU2. Each initializes its fixture and runs three checked eight-round primes, resets outside the timer, then performs 1024 complete batch additions. Every final bucket matches the independent generator-scalar oracle, output digests agree, and timed allocation counts are zero. Entries below are medians of 31 process means, in microseconds per batch.

| Caller | Entries | Frozen µs | Fresh OFF µs | Selected ON µs |
|---|---:|---:|---:|---:|
| g1-owned | 200 | 4.009 | 4.010 | 4.005 |
| g1-owned | 352 | 6.559 | 6.559 | 6.556 |
| g1-borrowed-signed | 200 | 4.216 | 4.216 | 4.206 |
| g1-borrowed-signed | 352 | 6.920 | 6.919 | 6.913 |
| g2-prefix | 200 | 7.579 | 7.570 | 7.576 |
| g2-prefix | 352 | 12.841 | 12.821 | 12.834 |

| Caller | Entries | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---:|---|---:|---:|---:|---|
| g1-owned | 200 | fresh/frozen | 1.000409976 | [0.999448279, 1.001912833] | 14/31 | pass |
| g1-owned | 200 | selected/frozen | 0.999343841 | [0.998805564, 1.000637990] | 18/31 | pass |
| g1-owned | 200 | selected/fresh | 0.999300888 | [0.997382058, 1.000288321] | 21/31 | pass |
| g1-owned | 352 | fresh/frozen | 0.999977685 | [0.999630413, 1.000708287] | 16/31 | pass |
| g1-owned | 352 | selected/frozen | 0.999552138 | [0.999248865, 1.000239983] | 18/31 | pass |
| g1-owned | 352 | selected/fresh | 0.999533258 | [0.999068209, 1.000116429] | 20/31 | pass |
| g1-borrowed-signed | 200 | fresh/frozen | 1.000060391 | [0.997998289, 1.002150005] | 15/31 | pass |
| g1-borrowed-signed | 200 | selected/frozen | 0.998125270 | [0.997091181, 0.999032324] | 25/31 | pass |
| g1-borrowed-signed | 200 | selected/fresh | 0.997901682 | [0.996502364, 0.999258131] | 23/31 | pass |
| g1-borrowed-signed | 352 | fresh/frozen | 1.000158187 | [0.999288415, 1.000982442] | 15/31 | pass |
| g1-borrowed-signed | 352 | selected/frozen | 0.998647756 | [0.997967520, 1.000085044] | 21/31 | pass |
| g1-borrowed-signed | 352 | selected/fresh | 0.998722703 | [0.998034908, 0.999597017] | 26/31 | pass |
| g2-prefix | 200 | fresh/frozen | 0.999077739 | [0.997933928, 0.999679527] | 25/31 | pass |
| g2-prefix | 200 | selected/frozen | 0.998889357 | [0.998056402, 1.000426417] | 19/31 | pass |
| g2-prefix | 200 | selected/fresh | 1.000727167 | [0.999569635, 1.001336206] | 13/31 | pass |
| g2-prefix | 352 | fresh/frozen | 0.998789924 | [0.998004481, 0.999009433] | 27/31 | pass |
| g2-prefix | 352 | selected/frozen | 0.999864324 | [0.999230827, 1.000674987] | 16/31 | pass |
| g2-prefix | 352 | selected/fresh | 1.001174097 | [1.000943631, 1.001724782] | 5/31 | pass |

The equal-weight six-case paired geometric aggregates are [recorded in analysis](artifacts/analysis.json): selected/frozen `0.999186458` and selected/fresh `0.999638985`. Aggregate upper bounds must be below one with at least 21/31 wins against both controls. Each individual upper ratio must be at most 1.02, and every fresh/frozen calibration interval must fit [1/1.03,1.03]. Ordered 10th–22nd ratios have nominal 97.055063% coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusions, extension or unchanged reroll is used. Equal case weights do not model the whole prover's work distribution.

Four separate warm 199 Hz cycles:u profiles follow all latency. Each performs 262,144 calls after three eight-call primes; capture excludes initialization, primes and oracle verification, and includes the loop and accounting. All captures precede decode, hash and archive work. No owned helper, build, hash, archive or other benchmark overlaps capture. Every profile has one sampled PID and zero lost records. Symbol shares are not removable proof wall-time fractions.

| Caller | Variant | Samples | Lost |
|---|---|---:|---:|
| g1-borrowed-signed | fresh | 352 | 0 |
| g1-borrowed-signed | selected | 361 | 0 |
| g2-prefix | fresh | 388 | 0 |
| g2-prefix | selected | 396 | 0 |

CPU usage, RSS, capacities, complete process means and symbols are retained. Component RSS includes fixtures, setup and thread initialization; it is not full-prover RAM or minimum memory. No new SDK/Gnark proof was produced; ledger remains 12,785. There is no new all-eight/54 proof timing, portable/mobile/TEE, pressure/concurrency, large-circuit or AWS/Aeglos result. A passing caller screen requires separate full-SDK and broader qualification before any adoption.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These accepted proof timings and supplied references are inherited.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted primary historical peak RSS is 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB historical peak with no supplied M5 reference. No current-source AWS/Aeglos comparison is added.
