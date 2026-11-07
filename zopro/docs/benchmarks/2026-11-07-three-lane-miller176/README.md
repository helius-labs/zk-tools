# Packed execution of three verification Miller loops

**The Miller product at 32 workers improves 35.24% against the current parallel policy, with calibrated controls and 15/15 wins. The overall gate remains false: only the full-endpoint calibration checks at 16 and 32 workers fail.** Production remains unchanged. The Rust prototype keeps three independent Miller recurrences in SIMD lanes, then multiplies their results and uses the original final exponentiation. These are synthetic component measurements, not SDK proof or complete verification timings.

## Execution strategy

The current verification key already prepares its fixed G2 lines. Its native policy runs three one-pair scalar Miller loops with nested Rayon joins at three or more workers, or a shared three-pair loop with one worker. The candidate runs the common schedule once with three real coefficients in the low vector half and three imaginary coefficients in the high half. The fourth complex lane stays the identity. A fused eight-lane two-product operation performs the complex multiply. Six packed Fq2 coefficients hold Fq12 state throughout the recurrence; extraction and the final product of three scalar Fq12 values are timed.

This differs from the earlier subgroup-square-only, G2 coordinate-constructor and scalar Fq6 experiments. Their negative gates remain closed. Existing mixed-R256 field1200 and its qualified primitives are reused unchanged. There is no copied proving key, retained point table, new production pool or C/assembly implementation. Explicit function boundaries for packed square/line/general-product kernels were fixed before timing; there was no layout or annotation sweep.

## Correctness

The optimized prototype passes 1,536 extension output comparisons with a separately organized degree-five Fq2 polynomial convolution reduced by w^6=9+u. It also checks 128 complex batches and all 16 Miller triples against the retained shared loop, products of retained one-pair loops and an exactly copied fresh shared-loop body. Identities, signs, cancellation, valid/invalid pairing products and unchanged final exponentiation are covered. Tests execute on a 512 KiB stack. This reuses scalar base-field arithmetic; it is not a new independent field or Gnark proof qualification.

The original preparation/shared-loop text is recovered exactly after reversing its import and removing the copied constants/accessors. Initial qualification linking failed on missing helper-dependency LTO bitcode; selecting coherent frozen dependencies repairs only that build. Initial benchmark compilation failed on include! of a module inner doc comment; a path module repairs only that build. Failed notes/sources are retained. The successful qualification, field library and earlier source/native/oracle/capture stages were not rerun. Unused diagnostic-method warnings are retained without cleanup rebuilds.

## Complete caller measurements

Ryzen 9 9950X, native optimized fat-LTO/one-codegen-unit common executable. Six contexts, fifteen fixed rotating/reversed process blocks,31 timed calls after 3 checked primes. `native` uses the original field-library policy, `fresh` copies that scalar policy, `packed` runs the new SIMD strategy. All workers have 512 KiB stacks and observed CPU/TID pins (CPU2 solo,0–15/0–31 otherwise). Every output matches one of eight cached synthetic full-field oracles; no new fixture/solver/key/proof generation occurs. Timers include pool.install dispatch/join, in-kernel allocations, packing, unpacking and final scalar multiplication. They exclude loading, public-input MSM and dynamic proof-B preparation.

Per-process statistic is median31 samples. Displayed medians are not paired ratios. Bounds use ranks4..12 of15; no familywise guarantee. Calibration requires fresh/native within factor1.03; all candidate nonregression upper bounds<=1.03. Required gains are full32 and all-six equal-log aggregate against BOTH controls, upper<1 and>=10wins. No gain floor, exclusions, extensions, favorable-subset promotion or reroll.

| Endpoint | Workers | Native µs | Fresh µs | Packed µs | Packed/native [bounds],wins | Packed/fresh [bounds],wins |
|---|---:|---:|---:|---:|---|---|
| miller | 1 | 277.012 | 276.893 | 112.943 | 0.407127 [0.406455,0.408353],15/15 | 0.407584 [0.406528,0.408405],15/15 |
| miller | 16 | 173.697 | 173.508 | 111.029 | 0.641133 [0.636384,0.648686],15/15 | 0.638761 [0.636789,0.644317],15/15 |
| miller | 32 | 174.640 | 174.028 | 112.452 | 0.647586 [0.636497,0.679060],15/15 | 0.646057 [0.641610,0.682130],15/15 |
| full | 1 | 488.823 | 489.173 | 323.340 | 0.662354 [0.659858,0.664074],15/15 | 0.662364 [0.659683,0.663405],15/15 |
| full | 16 | 396.257 | 384.716 | 322.028 | 0.835417 [0.806621,0.855683],15/15 | 0.845639 [0.831737,0.849237],15/15 |
| full | 32 | 386.169 | 397.681 | 324.973 | 0.839332 [0.807913,0.863940],15/15 | 0.843390 [0.801979,0.857899],15/15 |

All-six aggregates: native `{"high":0.6614730673043447,"low":0.6541985551016809,"median":0.6558119604944307,"wins":15}`; fresh `{"high":0.659377753495238,"low":0.6556353065360215,"median":0.6575552873244651,"wins":15}`. Failed checks: `["calibration full w16","calibration full w32"]`. Full calibration intervals and serial-shared diagnostics are preserved in [analysis](artifacts/analysis.json); no observation is discarded. This tested execution policy is closed without an SDK trial or post-result lane/worker/layout/compiler/annotation rescue. It does not rule out all SIMD extension-field verification strategies.

## Profiles and memory scope

FIFO-gated PMU and cycles profiles follow all latency, full endpoint native/packed at1/32workers,8192calls each after3primes. Fixed4M cycles:u period,512pages, sampledCPU/timestamp/period. Setup, primes and final oracle checks are outside the gate; timing-loop instrumentation is inside it. Profile extraction has one simple parser, without an independent exact ELF/IP/TID/period reparser. Do not treat flat symbol shares as exclusive wall time or a cause of memory/CPU scaling. Raw files/decode streams are locally hash-indexed, with summary reports archived.

| Profile | Counted samples | Unknown | LOST records | ≥1000 samples |
|---|---:|---:|---:|---|
| profile-b0-full-w1-native | 5530 | 3 | 0 | true |
| profile-b0-full-w1-packed | 3650 | 15 | 0 | true |
| profile-b0-full-w32-native | 9087 | 38 | 0 | true |
| profile-b0-full-w32-packed | 3849 | 27 | 0 | true |

Packed Fq12 payload is1,920 bytes. Compiler frames/temporaries, prepared inputs, allocator overhead and worker stacks are additional. The benchmark also retains output arrays outside clocks for verification. Reported process RSS includes fixture parsing/preparation/primes and driver history; it is not minimum RAM or a production memory saving. No pressure/concurrency/mobile/huge-circuit or portable qualification is claimed.

## Accepted full-proof comparison, unchanged

No new proof timing, AWS/Aeglos/Gnark benchmark or fresh proof export is added. The [all-eight supplied M5 CPU and Metal comparison](../2026-11-07-nonbinary-radix176/README.md#accepted-proof-baseline-unchanged) remains unchanged: local accepted32-worker1×2=7.915095ms,merge54×1=161.718956ms. CPU10× requires1×2<=3.7ms. Production176/SDK6d83 and the canonical31,387-proof ledger stay unchanged; all eight goal requirements remain active and unmet0/8.

[Qualification](artifacts/qualification.json), [qualification design](artifacts/design.md), [timing protocol](artifacts/timing-design.md), [packed implementation](artifacts/packed.rs), [driver](artifacts/bench.rs), [all observations, gzip JSON](artifacts/observations.json.gz), [command receipts, gzip JSON](artifacts/receipts.json.gz), [local bulk hashes](artifacts/bulk-index.json) and [checksums](SHA256SUMS). The two large JSON files use lossless gzip compression; [decompressed hashes](artifacts/compression.txt) verify their exact roundtrip. Synthetic point/result payloads, binaries/libraries and original key/witness/proof payloads are not published.

## Additional review and next experiment

An independent reader reconstructs all 22 timing checks from raw process samples, all 356 output digests from the reused cached results, 377 command stream receipts, and every worker CPU/TID set. The only failures remain the two full-endpoint calibrations; all gain and nonregression requirements pass. All four PMU groups report 100% running. Additional sample-header checks bind the 4M period and every sampled TID/CPU to the observed worker set or coordinator. This is not an independent ELF/IP or LOST-record parser. The 16-triple qualification command, exit and output receipt is also preserved. [Review](artifacts/independent-review.json), [review source](artifacts/review.rs).

The calibrated raw-Miller gains are retained. Full32 packed/native is 0.839332 [0.807913, 0.863940], with 15/15 wins, but its control calibration does not meet the declared bound; it is not promoted to a qualified full-proof gain. The 32-worker packed profile attributes 36.48% of samples to scalar Fq6 multiplication and 15.25% to the scalar final exponentiation's fp4-square body. These are sampled CPU shares, not removable wall-time fractions.

A distinct next hypothesis keeps the three Miller values packed through the existing final-exponentiation schedule and multiplies the three target-group outputs afterward. For nonzero factors this follows the homomorphism FE(a*b*c)=FE(a)*FE(b)*FE(c); zero handling must preserve failure. It changes the executed computation, since three final exponentiations replace one, and has no speed claim or implementation yet. It does not rerun this policy, change its lane layout, relax its gate or adopt the current candidate. [Prospective design](artifacts/next-design.md) records the qualification and full-scope requirements before that work.
