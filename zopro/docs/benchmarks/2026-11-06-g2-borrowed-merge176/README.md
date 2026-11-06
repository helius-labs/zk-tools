# Borrowed G2 bucket merge experiment

**Rejected. No adoption or full SDK trial from this result.** This replaces owned ordinary right-operand records with references during existing 512-entry G2 bucket merges. Arithmetic, bucket geometry, grouping, pools and keys stay the same. Production and accepted full-proof timings are unchanged.

Ryzen 9950X, 16 physical cores / 32 logical CPUs. Fresh OFF/ON and inherited optimized FROZEN use identical complete-query drivers and verified O/native/fat-LTO/one-codegen-unit flags. Each caller primes three times, times 17 queries and checks once more. The table reports medians of 15 process medians; paired ratios need not equal ratios of those displayed times. All allocation/drop, reset copies, grouping, merges and normalization are timed; parsing and pool creation are outside.

|Query / workers|FROZEN ms|OFF ms|ON ms|ON/OFF [bounds]|Wins/15|OFF/ON max RSS MiB|
|---|---:|---:|---:|---|---:|---:|
|1x2-A-w32|1.178943|1.192337|1.205111|1.013227 [0.990839, 1.031657]|5|26.00/26.00|
|1x2-B-w1|37.446879|37.265368|37.112901|0.995764 [0.993682, 0.996707]|14|15.50/15.50|
|1x2-B-w16|3.648247|3.440486|3.452348|1.001307 [0.994732, 1.012848]|6|26.50/27.50|
|1x2-B-w32|3.258823|3.029250|3.054688|1.016210 [0.995795, 1.021801]|5|38.25/32.00|
|2x2-B-w1|40.050257|39.489731|39.444405|0.997507 [0.995679, 0.998523]|12|27.25/27.25|
|2x2-B-w16|4.168368|3.912094|3.937783|1.002067 [1.000123, 1.013616]|3|43.25/41.75|
|2x2-B-w32|3.803009|3.472266|3.495729|1.002210 [0.995554, 1.014248]|6|60.75/52.25|

Both B shapes at 32 workers aggregate to 1.008936 [0.996659, 1.017894], 5/15 wins. Fresh-OFF nonregression: false; primary/aggregate gain: false; diagnostic qualification: true. OFF/FROZEN 3% calibration: false. The frozen/fresh discrepancy was unresolved before this experiment, so calibration is descriptive and component gates use fresh OFF as declared in the protocol. No component or control result establishes an accepted full-proof gain.

Native layout checks confirm 128-byte G2 points, 136-byte owned records and 16-byte borrowed records: 61,440 fewer requested pair-vector bytes per full 512-entry job. This is a local temporary-payload reduction, not proof-wide peak-RAM savings. The existing forecast remains conservative for requested heap capacity; stack headers, allocator retention and process RSS are separate. Unsupported/disabled dispatch returns before mutation and retains the existing fallback.

Full unfiltered suites pass 125 OFF and 129 ON tests. Four new tests cover actual record layouts, independent generator-sum oracles for ordinary/identity/doubling/opposite points and SIMD tails, typed disabled fallback without mutation, repeated calls, immutable right inputs, and public merge chunk boundaries at 1/4/16/32 workers with 512 KiB stacks. Test-only dispatch counts prove the path executes; timing binaries have no dispatch counters and keep the inherited API metadata at zero. Existing field qualification is inherited, not rerun. Both libraries and all transitive native dependencies are frozen coherently.

The 177-file snapshot preserves 172 of 176 originals byte-exact; all originals recover by reversing narrow dispatch/visibility/export edits and suffixes. All original field and arithmetic bodies are unchanged. Initial source/compile and qualification receipts are retained. No successful recording is repeated. Seven exact frozen preflight receipts are reused, with 341 new caller processes: 348 qualified rows, 9,168 query invocations including 35 reused. These are component queries on fixed fixtures, not serialized proofs or distinct witnesses.

All 315 latency callers finish before six PMU panels and six separate cycles profiles. Diagnostics use FIFO-controlled 200-query intervals, fixed 16M cycles:u sampling, checked actual PID/worker TID/CPU/executable MMAP and zero loss. Profiles and generic cache misses do not identify a causal memory level or critical path. Independent review reconstructs performance statistics/gates, source recovery, receipts, final flags, RSS and exact profile ownership/period/CPU mapping. Raw PMU parsing and symbol qualification remain in the capture helper; review does not claim a second PMU parser.

The primary 32-worker PMU panel records 567.607 to 568.034 million instructions and 318.391 to 317.996 million cycles per query, with generic cache misses 1.0392 to 1.0366 million. These separate diagnostic runs show little change in total work. At one worker, paired latency improves by 0.42% for primary B and 0.25% for grouped 2x2 B; those gains do not carry through the declared wide-worker criteria. Six profiles contain 22,142 samples, with zero loss and less than 0.9% unknown samples. The dominant sampled symbols remain borrowed affine addition, layer reduction and field inversion. This result does not support treating merge operand copies as the main remaining CPU bottleneck.

Closeout review checks 897 pin occurrences and 380 successful workflow commands. Its first attempt omitted xtask's explicit timeout prefix while comparing command arguments; the original reviewer and failure log are preserved. Correcting that receipt comparison reuses all native tests, links and recordings. No measurement is repeated for this repair.

Accepted local full-proof results and supplied M5 references are inherited:

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37|24|3.7|
|2x2|11.151932|59|30|5.9|
|4x2|16.086386|109|48|10.9|
|8x2|24.179453|189|74|18.9|
|12x2|33.884455|284|101|28.4|
|16x2|40.077381|335|124|33.5|
|24x2|65.850777|528|176|52.8|
|merge24|65.060937|507|175|50.7|

Merge54 stays 161.883899 ms without a supplied M5 reference. SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`; proof ledger stays 30,708. All-eight CPU10x remains active and unmet, 0/8. No new SDK/Gnark, AWS/Aeglos, phone, pressure/swap, minimum-RAM or concurrent-throughput result is claimed.

[Analysis](artifacts/analysis.json), [review](artifacts/review.json), [protocol](artifacts/protocol.md), [source snapshot](artifacts/source-snapshot.hex.json.gz), [receipts and helpers](artifacts/receipts.json.gz), [raw profile inventory](artifacts/raw-profile-inventory.json), [checksums](SHA256SUMS). Source hex and UTF-8 receipt archives round-trip exact bytes, lengths and hashes. Runtime point/scalar fixtures, private keys/witnesses, executables, libraries and raw perf.data stay local with hashes. Reproduction requires pinned toolchain/dependencies and private fixtures. Private root: `/home/ubuntu/src/zopro-perf-work/g2-borrowed-merge176`.
