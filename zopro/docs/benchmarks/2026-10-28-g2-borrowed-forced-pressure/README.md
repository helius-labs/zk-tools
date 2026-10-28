# Borrowed G2 under forced high-limit pressure

**The accepted baseline timed out under persistent high-limit throttling; candidate cases did not run.** Only its three priming requests completed. No new complete exports or independent proofs were available, so the ledger stays14,551. Owned services are cleaned up. No larger-limit retry occurs; production remains unchanged.

The [preceding qualification](../2026-10-28-g2-borrowed-qualification/README.md) passed its bounded-memory cases, but every high-event counter was0. This new declared scenario holds112MiB of touched anonymous ballast, lowers memory.high to96MiB, retains the same192MiB hard limit and disables swap. Ballast alone exceeds the high limit. It tests actual kernel throttling on the exact accepted/candidate warm SDKs, with default/retained allocator settings. No old latency block or failed limit is retried.

Each case primes3 times under512MiB, applies the declared limits, then makes2 fresh requests. The privately copied image is charged inside the owned service. Success requires positive high events, normal SDK completion, no OOM/kill/max events, expected key/public verification and cleanup. Intentional throttling gets a300second SDK-child deadline; these times are not pooled with latency measurements. Limits are validation conditions, not architecture caps or minimum RAM.

| Case | Executed | Passed | High events | Peak MiB | Max events | OOM/kill |
|---|---|---|---:|---:|---:|---|
| warm-accepted-default | true | false | 23166 | 177.59 | 0 | 0/0 |
| warm-accepted-retained | false | false | null | unobserved | null | null/null |
| warm-on-default | false | false | null | unobserved | null | null/null |
| warm-on-retained | false | false | null | unobserved | null | null/null |

Every available complete export is hashed,164bytes, unique against14551 prior identities, and independently checked against the historical primary image/public. If the controller stopped early, unexecuted cases stay unexecuted and any complete previously unverified export is checked afterward with a retained receipt; successful verification is not repeated. Known request counts do not invent unobserved partial work. No larger-limit restart or source/library/SDK change occurs. Service cleanup affects only the unique zopro-borrow-forced prefix; unrelated work and global kernel settings are unchanged. No owned build/hash/archive/second benchmark overlaps captures.

The primary32 performance gain remains the [preceding measured2.13% result](../2026-10-28-g2-borrowed-sdk/README.md), not a timing from this test. The two physical16 performance bounds remain failed. This pressure result informs an explicit adoption assessment and does not silently promote the old gate or satisfy the original10x objective, which stays active/unmet0/8. No new phone/TEE, AWS/Aeglos, swap, minimum-RAM or250/4999 result is claimed.

## Inherited CPU and Metal comparisons

All8 rows below come from the completed full SDK study, with supplied historical M5 references; no latency rerun occurs here.

| Shape | Accepted ms | Candidate ms | M5 CPU ms | M5 Metal ms | CPU/candidate | Metal/candidate |
|---|---:|---:|---:|---:|---:|---:|
| 12x2 | 34.317529 | 34.148377 | 284 | 101 | 8.317× | 2.958× |
| 16x2 | 40.360763 | 40.625234 | 335 | 124 | 8.246× | 3.052× |
| 1x2 | 8.379021 | 8.226135 | 37 | 24 | 4.498× | 2.918× |
| 24x2 | 66.617867 | 66.614080 | 528 | 176 | 7.926× | 2.642× |
| 2x2 | 11.502104 | 11.434172 | 59 | 30 | 5.160× | 2.624× |
| 4x2 | 16.233889 | 16.228639 | 109 | 48 | 6.717× | 2.958× |
| 8x2 | 24.535797 | 24.481497 | 189 | 74 | 7.720× | 3.023× |
| merge-24x1 | 66.145121 | 65.775218 | 507 | 175 | 7.708× | 2.661× |

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [case receipts](artifacts/runs), [cleanup](artifacts/cleanup-units.txt). Executables remain local with hashes.

## Outcome and scope

The accepted/default baseline warmed successfully through3 internally verified requests, then failed to finish either fresh request before the fixed300second child deadline. The controller stopped after that first case. The other accepted/retained and both candidate cases remain unexecuted, not passing or failing candidate observations. There are no new complete exports and no additional independent proofs; the ledger stays14551. All owned services are stopped/reset and their control directories cleaned.

Observed baseline peak is186212352bytes (177.586MiB), below the192MiB hard maximum. Events are23166high,0max,0OOM and0OOM-kill. The retained child failure says `owned child timeout`; this is not an OOM observation or evidence of a candidate regression. No larger limit or longer deadline is tried.

The scenario holds112MiB of touched anonymous memory while the high limit is96MiB and swapping is disabled. The ballast alone therefore exceeds the soft limit by16MiB even before SDK memory. That creates persistent, irreducible high-limit excess while the ballast lives. It establishes actual kernel throttling, but does not provide a useful accepted/candidate comparison or establish that either prover cannot eventually complete without the test deadline. It also does not establish minimum RAM, mobile behavior or swap performance.

This failed stress condition must not erase the completed prior evidence:750 full-SDK exports and all shape measurements, followed by89 additional portable/opt-out/stack/concurrent/bounded-memory exports. The primary32 paired2.13% gain and40 fewer allocations/1.404MiB less allocation traffic remain measured. The two physical16 bounds remain failed and their point estimates remain positive. Ordinary192MiB bounded cases passed, with no observed high events in those cases; none of these results is silently relabeled.

Next is a concrete clean-source adoption assessment using that verified scope, explicitly recording an engineering exception if the primary32 gain is adopted despite the two physical16 uncertainty bounds. This extreme baseline-only timeout remains a limitation, rather than a new hidden performance rejection rule or a reason to keep repeating synthetic quotas. Any final artifact must preserve the qualified algorithm, immutable-key/fallback behavior and source provenance. The original all-eight CPU10x goal remains active/unmet0/8.
