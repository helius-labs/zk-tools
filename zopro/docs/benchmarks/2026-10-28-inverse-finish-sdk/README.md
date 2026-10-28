# Direct inverse finish in full SDK proofs

The candidate **failed the primary SDK screen**. The complete paired results, including uncertainty and failed checks, are retained. No production adoption or global-goal achievement is claimed.

At 32 workers, ON takes 8.526416 ms versus 8.502570 ms accepted and 8.535152 ms fresh OFF. The paired ON/OFF median ratio is 0.998796488 (11/15 wins), but its interval [0.993408098, 1.001401291] crosses parity. ON/accepted is 1.000753237 with only 7/15 wins. The physical-16 run fails calibration and ON nonregression; its paired ON/OFF median ratio is 1.043004820. All observations remain. The tiny caller gain did not establish a full-proof gain, so the candidate is not adopted and this unchanged implementation is retired from further timing trials.

The [addition-caller study](../2026-10-28-inverse-finish-callers/README.md) passed its six-case screen with small aggregate gains, 0.081% against frozen and 0.036% against fresh OFF. This experiment tests whether the unchanged arithmetic helps complete verified requests. The earlier [primitive schedule](../2026-10-28-inverse-direct-finish/README.md) remains unqualified by its duplicate ON control and was not rerun.

The inverse's final signed linear form is reduced directly with a constant reciprocal instead of a Montgomery cancellation followed by a field multiplication restoring that scale. The Euclidean loop, 31/62-bit matrices, field representation, curve formulas, query layout and scheduler remain. No key table, heap buffer, retained plan, assembly rewrite or new runtime option is added. Exact input assumptions and independent arithmetic checks are in the parent report.

The SDKs link the standard driver/profile helper through Rust xtask against the parent's immutable, content-addressed OFF/ON libraries. All hashes, flags and distinct field/core/SDK identities are checked before capture. Both library sets contain the same diagnostic-only caller module/reexports, which the normal SDK does not call; there is no observer hook or runtime instrumentation in the proof path. This source scope is retained: any positive result still needs a production build without adapters and broader qualification before adoption. The parent 122-test core suites, three adapter cases, 34-test field suites and 68,672 independent signed-residue cases are inherited, not rerun. New raw-image two and cubic six prover tests pass in each configuration with explicit required names.

Two preparation failures are preserved. The first link mapping omitted `memmap2`; it was added from the parent's already-frozen libraries, and the passed OFF suites were reused. Later, the optional provenance-extension helper checked all four suite receipts, then stopped while hashing `native-first/result.json`; a failed workflow writes `failure.json` instead. Capture proceeded using the original 749-pin manifest, which remained unchanged and matches at closeout. The pre-capture suite-check record agrees with the retained suite receipts. The extension was not represented as complete, no measurement was repeated, and [closeout](artifacts/final-qualification-audit.json) records the failure and original manifest. These preparation failures do not change the performance gates or proof checks.

Fifteen fixed rotated/reversed blocks compare accepted SDK14da, fresh OFF and ON at 32 logical and 16 physical workers. Each latency process runs three primes then 151 fresh SDK requests. Three four-worker preflights precede latency. Loading is excluded; witness solving, fresh randomness, proving, internal verification and serialization are included.

| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---|---:|---:|---:|---:|
| 32 | control | 8.502570 | 191.093682 | 130.25 | 133.75 |
| 32 | off | 8.535152 | 190.436636 | 131.50 | 134.00 |
| 32 | on | 8.526416 | 191.384616 | 130.00 | 134.75 |
| 16 | control | 11.012643 | 124.300146 | 89.00 | 92.00 |
| 16 | off | 10.763875 | 124.136623 | 87.75 | 92.00 |
| 16 | on | 11.263588 | 123.556848 | 88.25 | 92.50 |

| Workers | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---|---:|---:|---:|---|
| 32 | off/control | 1.001493721 | [0.993521418, 1.008470527] | 6/15 | pass |
| 32 | on/control | 1.000753237 | [0.994913631, 1.003815435] | 7/15 | fail |
| 32 | on/off | 0.998796488 | [0.993408098, 1.001401291] | 11/15 | fail |
| 16 | off/control | 0.975395372 | [0.947020892, 1.009834482] | 9/15 | fail |
| 16 | on/control | 1.013361705 | [0.968451388, 1.048407442] | 7/15 | fail |
| 16 | on/off | 1.043004820 | [1.003703499, 1.059184075] | 2/15 | fail |

Ratios pair the 15 process medians. The 4th–12th ordered bounds have nominal 96.484375% coverage under independent exchangeable blocks, without a familywise guarantee. OFF/accepted calibration must fit [1/1.03,1.03]. Primary ON requires upper ratio below one and at least 11 wins against both controls; physical-16 upper ratios must be at most 1.02. There is no gain floor, exclusion, extension or unchanged reroll.

All 90 latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles, with 251 requests after three primes. All 99 SDK captures precede independent Gnark verification, profile decoding, hashes and archive work. No owned helper, build or other CPU benchmark overlaps capture. Each profile has one sampled SDK PID and zero lost records; symbol periods and raw profile hashes are retained. Inverse-symbol share includes the entire scalar inverse, not just its changed finish, and is not a removable wall-time fraction.

| Workers | Variant | Samples | Lost | Scalar-inverse symbol period share |
|---|---|---:|---:|---:|
| 32 | control | 8999 | 0 | 7.050% |
| 32 | off | 8789 | 0 | 6.766% |
| 32 | on | 8945 | 0 | 6.854% |
| 16 | control | 5490 | 0 | 7.812% |
| 16 | off | 5570 | 0 | 8.759% |
| 16 | on | 5495 | 0 | 8.390% |

All 15,396 SDK calls verify internally. The 99 exported fresh 164-byte proofs independently pass Gnark, public binding and disjointness against 12,785 prior identities; ledger 12,884. Full pins and every exported member are checked. RSS includes loading and primes and is not minimum RAM or memory-pressure qualification. Production and accepted SDK remain unchanged. No new all-eight/54, portable/mobile/TEE, concurrent whole-proof, pressure, large-circuit or AWS result is claimed.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These accepted local comparisons are inherited, not measurements from this primary-only experiment.

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

Accepted primary historical peak RSS is 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB historical peak with no supplied M5 reference. The latest AWS/Aeglos comparison predates the current accepted FFT source; no new cloud claim follows from this experiment.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [qualification/linking](artifacts/native/result.json).
