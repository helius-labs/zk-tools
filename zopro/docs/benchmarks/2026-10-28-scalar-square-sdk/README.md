# Scalar symmetric square in full SDK proofs

The candidate **does not pass the primary SDK screen and is not adopted**. At 32 workers, paired median ON/OFF latency is **0.997867676** with marginal interval [0.990994106, 1.001811947] and 9/15 wins. ON/accepted is **1.009859033**, interval [1.002443175, 1.012866903], 1/15 wins. The full gate retains every calibration and physical-16 nonregression check. Production and the accepted SDK remain unchanged.

The [component screen](../2026-10-28-scalar-symmetric-square/README.md) found a 6.68% Fr aggregate improvement and roughly 10.6% lower fifth-power update latency. The unchanged Fq dependent control also improved, so no direct transfer of the entire component gain was assumed. This study uses exactly that native Fr-only safe Rust square kernel and dispatch, with no new adapters or proof-path instrumentation. Fq and IFMA dispatch remain unchanged. The wide square uses ten distinct limb products before the existing mathematical Montgomery reduction. No heap buffer, key lookup table, cached plan, assembly rewrite, or runtime option is added. Attribution and license are retained in the parent and private source.

## Complete verified requests

Fifteen fixed rotated/reversed blocks compare accepted SDK14da, fresh OFF and ON at 32 logical and 16 physical workers. Each latency process has three primes followed by 151 complete requests. Three four-worker preflights precede latency. Loading is outside timing; request witness construction, constraint solving, fresh randomness, proving, internal verification and output serialization are included. An unrelated ceremony job was present in the recorded host snapshot; no exclusive-host claim is made.

| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---|---:|---:|---:|---:|
| 32 | control | 8.500478 | 191.114305 | 129.75 | 134.75 |
| 32 | off | 8.612909 | 191.639318 | 129.25 | 136.00 |
| 32 | on | 8.573445 | 191.693662 | 130.75 | 136.00 |
| 16 | control | 10.873244 | 125.356106 | 88.00 | 90.75 |
| 16 | off | 11.122164 | 124.261344 | 87.50 | 91.25 |
| 16 | on | 10.864779 | 123.939934 | 88.50 | 92.75 |

| Workers | Paired ratio | Median | Marginal interval | Wins | Check |
|---|---|---:|---:|---:|---|
| 32 | off/control | 1.018070748 | [1.005160575, 1.020535499] | 1/15 | pass |
| 32 | on/control | 1.009859033 | [1.002443175, 1.012866903] | 1/15 | fail |
| 32 | on/off | 0.997867676 | [0.990994106, 1.001811947] | 9/15 | fail |
| 16 | off/control | 1.013292347 | [0.989223445, 1.050654767] | 6/15 | fail |
| 16 | on/control | 0.988296428 | [0.980146955, 1.046053459] | 10/15 | fail |
| 16 | on/off | 0.994525262 | [0.956131738, 1.015460299] | 11/15 | pass |

Ratios use paired process medians; bounds are ordered ranks 4 through 12 of 15, nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise guarantees. OFF/accepted calibration must fit [1/1.03, 1.03] at both worker counts. Primary ON must have upper ratio <1 and at least 11/15 wins against both controls; physical-16 upper bounds must be ≤1.02. No gain floor, exclusions, schedule extensions, successful-capture repeats or unchanged rerolls were used. Failed checks: `["w32/on/control","w32/on/off","w16/off/control","w16/on/control"]`.

## Correctness and profiles

Both native core suites passed 119 tests, including required solver-prefix, refined-component and invalid-witness cases. Both configurations passed two raw-image tests and six cubic prover tests. The exact source/flags parent field suites (35 OFF and 35 ON) are inherited, not rerun. Some optional dump/Gnark-fixture core tests return without external fixtures; this report does not claim those external cases were exercised. Standard SDK preflights and independently verified complete 1x2 proofs exercise the actual solver/prover path.

Pure libraries were built in separate target roots and immediately frozen, including transitive rlibs and proc-macro shared objects. Hashed rmeta companions were used for any primary Cargo rlib instead of mutable root outputs. The standard cpu-sdk driver/profile helper links through Rust xtask with eight required libraries, including memmap2. Exact flags, source pins, distinct OFF/ON field/core/SDK hashes and all receipts are checked. No diagnostic caller module is present.

All 90 latency processes precede six FIFO-gated warm 199 Hz cycles:u profiles, with 251 requests each after three primes. All 99 captures precede all 99 independent Gnark verifications, then decoding, hashes and archive work. No owned helper, build, hash, archive or second CPU benchmark overlaps capture. Every profile has one sampled SDK PID and zero lost records. Solver symbol shares include broad solver work and inline code; they do not isolate the changed square or represent removable proof wall time.

| Profile | Samples | Lost | Solver symbol period share |
|---|---:|---:|---:|
| profile-w16-control | 5460 | 0 | 1.902% |
| profile-w16-off | 5637 | 0 | 1.941% |
| profile-w16-on | 5512 | 0 | 2.130% |
| profile-w32-control | 9006 | 0 | 1.702% |
| profile-w32-off | 9128 | 0 | 1.990% |
| profile-w32-on | 9130 | 0 | 1.949% |

Total profile samples: 43873. Raw perf.data stays local with recorded hashes; decoded observations and symbol weights are archived.

All **15,396** SDK calls verify internally. All **99** exported fresh 164-byte proofs independently pass Gnark, image/public binding and disjointness against 12,884 prior identities, advancing the ledger to **12,983**. Proof and public members are archived. RSS includes loading and primes; it is not minimum RAM or pressure qualification.

## Accepted comparison (inherited)

These are accepted 172-source observations, not this candidate's all-shape results. The original all-eight 10× CPU objective remains active and unmet (0/8); primary target 3.7 ms. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

Accepted historical primary peak RSS: 134.50 MiB. Merge54: 161.341835 ms / 1910.1875 MiB historical peak, with no supplied M5 reference. No new all-eight/54, portable/mobile/TEE, concurrent whole-proof, pressure, giant-circuit or AWS/Aeglos measurement is claimed. The latest AWS comparison predates the accepted FFT source. No cloud resource was created.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [native qualification](artifacts/native/result.json), and raw workflow/verification logs are archived. The component's preparation/analysis repairs are documented in its report; they were not repeated here.

This exact candidate is closed at the full-SDK gate. Retain its component gains and failed full-proof checks; do not rerun unchanged arithmetic, fields, flags or layout to search for a pass.
