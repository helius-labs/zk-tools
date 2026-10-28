# Victim-scan code in the disabled runtime

The rejected experimental runtime adds per-victim calls, closure-context loads, a runtime-option test and a second function frame. The accepted runtime keeps this scan in its caller. Saved IBS samples locate the added loads in the actual OFF path; the difference is more than a symbol-attribution change. No production optimization is adopted, and no fresh latency result is measured.

This closes the diagnostic fork following the [OFF IBS comparison](../2026-10-27-off-ibs/README.md) and [arithmetic audit](../2026-10-27-batch-off-code-audit/README.md). It does not establish what fraction of the whole-proof gap these instructions cause. Removing overhead from an already rejected experiment would not improve the accepted SDK.

## Source and assembly

The exact accepted SDK is `14da8231`, experimental OFF is `fde4038c`. Reversing the experimental flag field, initialization, transfer choice and private module declaration recovers the upstream registry source byte for byte. The source preserves victim order and retry semantics but tests the immutable runtime flag inside each victim callback.

The accepted `wait_until_cold` contains two scan loops calling the existing single-job Stealer directly. OFF contains two scan loops calling an outlined `filter_try_fold` closure, which loads closure context, reads Registry through WorkerThread, tests a byte at Registry+0x210, and on OFF branches to the same single-job operation. The closure saves six callee-saved registers and reserves 0x58 stack bytes even on OFF. The caller's frame reservation changes from 0x58 to 0xd8. These are reservations and instruction observations, not measured spills or a RAM-limit diagnosis.

| Function | Accepted bytes | OFF bytes | Accepted static instructions | OFF static instructions |
|---|---:|---:|---:|---:|
| wait_until_cold | 1661 | 2191 | 404 | 497 |
| outlined victim closure | — | 2711 | — | 626 |
| single-job Stealer | 356 | 356 | 84 | 84 |

Equal single-Stealer sizes/counts do not prove byte identity or equal execution costs; TLS offsets and call destinations can differ. Five selected function instances are inspected, not the whole program.

## Existing sample evidence

These are per-instruction demand-load counts from the six existing captures, not new captures or timings. Existing raw/script joins, operations, weights and data sources are rechecked while calculating this new selected histogram. All sampled selected addresses resolve to instruction starts.

| Process | Wait loads | Single-Stealer loads | Outlined-scan loads |
|---|---:|---:|---:|
| b0-control | 346 | 1077 | — |
| b0-off | 107 | 1015 | 1311 |
| b1-off | 111 | 976 | 1304 |
| b1-control | 331 | 1048 | — |
| b2-control | 349 | 1088 | — |
| b2-off | 84 | 977 | 1245 |

| OFF process | Flag pointer + byte loads | Five closure-context loads | Pop/return loads | All closure miss-weight sum |
|---|---:|---:|---:|---:|
| b0-off | 163 | 414 | 575 | 0 |
| b1-off | 149 | 398 | 586 | 0 |
| b2-off | 147 | 393 | 546 | 0 |

The extra closure's demand-load samples have zero positive cache-miss weight in all three OFF captures. The flag/context and return traffic is consistent with added instructions and cache-hit loads, not added RAM service. Counts cannot be converted into causal milliseconds, useful-work fractions or complete retired-instruction totals. Existing algorithm choices, contention and other compiler changes remain combined in the prior whole-proof result.

The inherited instrumented medians remain 8.462729 ms accepted and 9.170174 ms OFF; the accepted unprofiled checkpoint remains **8.324853 ms**, with 134.50 MiB peak RSS in its qualification run. Nothing here supplies a new memory measurement or minimum-RAM bound.

## Validation and provenance

Two new observer tests pass: offset-zero SDK mapping selection and instruction-range boundaries. The qualified IBS decoder is reused byte-identically; its nine successful tests are inherited and explicitly filtered out. PIE bias comes from the SDK's offset-zero mapping rather than an assumed file-offset/virtual-address equality. Function counts agree with the previously archived symbol-level counts.

The first audit's validation selector also included two samples in a nested sleep instantiation containing the same name. The failed source/log are preserved. Tightening only the expected symbol boundary fixes the validation, and no successful test or prover measurement is repeated. The final audit then completes all six saved-record analyses. This is exploratory static/sample analysis; the design was recorded after initial manual source/disassembly inspection, not described as a prospective timing protocol.

SDKs, actual libraries, 172 production source files, original observer and raw/script inputs are hash checked against the earlier pins. Pins are checked again at closure. No SDK requests, proof exports, independent re-verifications, prover builds or production edits occur. Proof ledger remains **12,524**. There are no live jobs or new AWS resources.

## Next direction

Do not recover the rejected fork through another runtime-switch, inlining or code-layout timing sweep. The accepted implementation already avoids this added wrapper. A substantial improvement needs a distinct reduction in useful arithmetic or dependencies. A separate exact mathematical cost/bounds study can test whether a compact quadratic representation of the BN254 base field offers a credible advantage before implementing or benchmarking it; the prior negative scalar-field Gaussian and six-coefficient PMNS results remain relevant. No speed claim follows from that proposal.

## Inherited CPU and Metal comparison

The [accepted all-eight report](../2026-10-27-clean-fft-adoption/README.md) remains authoritative, including its full-performance-gate failure and engineering exception. Supplied M5 CPU/Metal references are historical, not newly measured here.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 4.445× | 24 | 2.883× |
| 2x2 | 11.460675 | 59 | 5.148× | 30 | 2.618× |
| 4x2 | 15.854365 | 109 | 6.875× | 48 | 3.028× |
| 8x2 | 24.084626 | 189 | 7.847× | 74 | 3.072× |
| 12x2 | 33.693397 | 284 | 8.429× | 101 | 2.998× |
| 16x2 | 39.981636 | 335 | 8.379× | 124 | 3.101× |
| 24x2 | 66.010670 | 528 | 7.999× | 176 | 2.666× |
| merge24x1 | 65.086624 | 507 | 7.790× | 175 | 2.689× |

Accepted merge54 remains 161.341835 ms without a supplied M5 reference. The original all-eight CPU10× goal remains active and unmet, 0/8. No new Aeglos, AWS, phone, pressure, concurrency or giant-circuit comparison is claimed.

[Analysis](artifacts/analysis.json), [instruction classification](artifacts/review.json), [observer](artifacts/audit.rs), [design](artifacts/design.md), [source pins](artifacts/pins.json), [artifact hashes](artifact-index.json).
