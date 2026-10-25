# Small-query core entry: production CLI comparison

The unchanged small-core-entry source passes the final matched native release-profile comparison against the exact accepted SDK executable. The candidate uses the existing standard CLI source, fat LTO and one codegen unit, matching the workspace release profile. Source adoption is recorded separately after all qualification receipts are checked.

Ryzen9950X. Seven fixed rotated/reversed fresh-process blocks compare exact accepted binary and identical candidate runtimeOFF/ON. Nine shapes use32 unpinned workers; primary also uses16 physical workers pinned by the existing CLI. Each process excludes loading and three primes from latency. Full verified SDK request timing includes fresh assignment, randomness, proof, verification and serialization. Peak RSS includes loading, preparation and worker receipt collection. No owned build/profile/CPU benchmark overlaps timing. All samples are retained.

| Case | Accepted ms | OFF ms | ON ms | Paired ON/accepted | Paired ON/OFF | Wins/7 versus accepted/OFF | Accepted peak MiB | ON peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2-w32 | 9.013 | 8.987 | 8.689 | 0.96537 | 0.97047 | 7/7 | 121.00 | 112.25 |
| 2x2-w32 | 11.819 | 11.878 | 11.781 | 0.99549 | 0.98850 | 6/6 | 275.00 | 269.25 |
| 4x2-w32 | 16.444 | 16.231 | 16.306 | 0.99298 | 1.00458 | 6/3 | 318.00 | 315.00 |
| 8x2-w32 | 24.560 | 24.760 | 24.520 | 0.99599 | 0.98844 | 4/6 | 497.75 | 474.50 |
| 12x2-w32 | 35.593 | 35.012 | 35.097 | 0.98625 | 1.00372 | 6/3 | 676.25 | 674.75 |
| 16x2-w32 | 41.296 | 41.230 | 41.320 | 1.00759 | 1.00106 | 3/2 | 800.00 | 789.75 |
| 24x2-w32 | 69.106 | 68.955 | 69.314 | 1.00111 | 1.00620 | 2/2 | 1134.94 | 1178.31 |
| merge-24x1-w32 | 68.548 | 68.223 | 68.231 | 0.99829 | 0.99586 | 4/4 | 1091.50 | 1088.01 |
| merge-54x1-w32 | 174.375 | 175.450 | 174.892 | 0.99331 | 0.98226 | 5/5 | 1668.18 | 1649.02 |
| 1x2-w16 | 10.494 | 10.456 | 10.190 | 0.97149 | 0.97391 | 7/7 | 87.75 | 81.00 |

Primary wins7/7 versus both controls at32 and16 workers. Full-nine geometric ratios are0.991568 versus accepted,7/7 winning blocks, and0.992475 versus OFF,6/7. All ten cases remain within the inherited2% paired nonregression band against both controls. The less favorable cases remain visible:16x2 is1.007592 versus accepted and24x2 is1.001114; this is not an improvement on every shape.

| Shape | Historical M5 CPU ms | Historical M5 Metal ms | ON CPU speedup | ON Metal speedup |
|---|---:|---:|---:|---:|
| 1x2 | 37 | 24 | 4.26× | 2.76× |
| 2x2 | 59 | 30 | 5.01× | 2.55× |
| 4x2 | 109 | 48 | 6.68× | 2.94× |
| 8x2 | 189 | 74 | 7.71× | 3.02× |
| 12x2 | 284 | 101 | 8.09× | 2.88× |
| 16x2 | 335 | 124 | 8.11× | 3.00× |
| 24x2 | 528 | 176 | 7.62× | 2.54× |
| merge-24x1 | 507 | 175 | 7.43× | 2.56× |

The M5 values are user-supplied historical references, not fresh Apple measurements. The original all-eight10× CPU objective remains unmet.

The210 timing processes complete4,914 full SDK requests and export210 distinct fresh proofs independently verified with Gnark. Public inputs match pinned fixtures and proof hashes differ from checked earlier exports. The exact original standard CLI source passes all nine shapes and pinned16 compatibility,40 SDK calls and10 independent exports, with the new runtime flag omitted. The compiled default enables entry on the eligible native small path; other tested paths emit no entry trace. The initial legacy export-depth verifier error is retained; the existing first proof was verified after correcting the directory, without re-proving that case.

A separate sustained test uses the exact release-profile CLI for51 measured requests after three primes, per variant, under192MiB hard limit, no soft limit and no swap. The input is copied inside the cgroup and32MiB controller ballast remains live. All162 SDK calls complete; three exports independently verify. Accepted/OFF/ON cgroup peaks are117.809/117.930/117.328MiB including ballast and controller. The initial missing monitor-output parent error occurred before any unit or proof ran and is retained with its repair receipt. These are finite feasibility checks, not minimum RAM or an architectural cap.

Earlier matched native/server-compatible screens, math suites, concurrency, memory transitions and actual250/4999 proofs remain in their linked reports. The new core branch adds no thread pool, per-key field, query buffer, lookup table or solver plan. All source/library/image/binary checks pass; existing public APIs, input format and standard CLI arguments are retained. No new AWS or phone-performance result is claimed.

[Results](results.json), [protocol](protocol.json), [compatibility](compatibility/complete.json), [sustained limit](sustained/complete.json), [native qualification](../2026-10-25-small-core-entry-native-qualification/README.md).
