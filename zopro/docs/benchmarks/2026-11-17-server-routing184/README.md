# Parallel routing for server shifted G2 tables

Parallel routing reduces the serial prototype's cost but the complete query
remains slower than current Rust G2. This exact implementation does not advance
to full-proof integration. The result narrows the problem: reduction cost also
needs to fall; improving routing alone cannot establish a local win here.

| Captured query, 32 workers | Complete ms | Route ms | Reduction ms |
|---|---:|---:|---:|
| 1x2 current | 3.279210 | — | — |
| 1x2 serial shared buckets | 9.340784 | 5.663464 | 3.597841 |
| 1x2 parallel shared buckets | 5.177909 | 1.686899 | 3.442258 |
| 2x2 current | 4.237256 | — | — |
| 2x2 serial shared buckets | 16.097889 | 10.151302 | 5.833194 |
| 2x2 parallel shared buckets | 9.132672 | 3.178200 | 5.771317 |

Entries are medians of nine measurements; separate column medians need not sum.
Parallel/current paired medians are 1.597333 and 2.148853, with zero wins in both
cases. Three primes per arm precede nine rotated/reversed blocks. Input copying,
recoding, allocation, count/scatter/concatenation, indexed reduction, signed bucket
merging and final normalization are included. Routing/reduction spans are coarse
instrumented component intervals, not an exact accepted full-proof decomposition.
Both shifted modes have identical timer reads; no calibrated phase-observer claim
is made. These large regressions do not support production integration.

The immutable 22-window, width12 table and indexed reducer are unchanged from
[the serial trial](../2026-11-17-server-shifted183/README.md). Each worker builds
its local histogram and order; disjoint destination bucket slices concatenate
these in parallel. This adds a second contribution-order allocation and per-worker
counts/starts. Table payload remains about 81 MiB/140 MiB for these two queries.
Peak whole-process RSS is 212024 KiB, including setup and both contexts. Process
CPU is 19.90 user + 0.28 system seconds; per-context CPU was not measured.

All timed outputs and primes match retained independent Arkworks full-query hashes.
Fixture hashes and current-query outputs are checked, and the parent identity,
inverse, duplicate/full-width and shared-table tests pass. The timing control's
extra scalar copy was found before any timing capture, removed in source-fixed,
and correctness rerun. Both source snapshots and build receipts are retained.
No successful timing capture was repeated. No new proofs or AWS resources;
canonical56585 and production remain unchanged. Commit is local; no push.

Two direct shared-bucket variants now fail locally. Close this family absent new
evidence; do not sweep width or chunk constants. The reduction machinery is the
next discriminating design constraint for a future server algorithm. This is not
proof that precomputation cannot help on C7a, and no target speedup is claimed.
A new server proposal should remove arithmetic/reduction work or change its
execution dependencies, with complete memory accounting and low-memory fallback.
Private root: `/home/ubuntu/src/zopro-perf-work/server-routing184`.
