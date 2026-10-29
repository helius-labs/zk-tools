# G2 preparation on accepted175

The new observer separates the previously unexplained G2 preparation interval into serial scalar grouping and decomposition service. It qualifies at the declared overhead limits and adopts no optimization. Production source175 and its accepted SDK remain unchanged.

## Preparation service

Nine fixed main graphs cover rounds 0/15/30 in three ON processes. The figures below are independent medians, so they need not add.

|Measurement|Median|
|---|---:|
|G2 entry → input marker, ms|2.235615|
|Grouping elapsed, ms|1.424636|
|Grouping thread CPU, ms|1.307020|
|Decomposition elapsed, ms|0.522505|
|Sum of decomposition fold CPU, ms|0.437115|
|Fold CPU per input, ns|14.527402|
|Consumer folds|64.000000|
|Maximum concurrent fold spans per graph|1.000000|
|Stage entry → first fold, ms|0.000531|
|Stage return → input marker, ms|0.000120|

Each graph decomposes all 30,089 inputs exactly once, with no empty folds. No grouped-point stage occurs in the nine main graphs: grouping declines and the original inputs reach G2. Five graphs have 64 folds and only one live fold span at a time. The other four have 188–897 folds and 2–6 simultaneously live spans. Fold CPU includes observer/count/clock work; live span counts are not hardware CPU utilization.

Four graphs have a substantial interval after the final decomposition fold ends. Covered service on the physical decomposition owner belongs to other logical queries:

|Process/round|Last fold → stage return ms|Foreign serial-span coverage|Fully contained foreign CPU ms|
|---|---:|---:|---:|
|b1-on/0|0.879138|94.14%|0.310365|
|b1-on/15|0.701913|92.05%|0.076033|
|b1-on/30|1.064718|93.73%|0.993972|
|b2-on/15|0.769972|90.31%|0.138732|

The other five last-fold-to-return intervals are 80–111 ns. Span coverage includes descheduling within a leaf. CPU is summed only for fully contained leaves; partially clipped CPU remains unknown rather than being prorated. Uncovered gaps are not idle time, and these intervals are not removable full-proof savings.

The main newly covered CPU cost is grouping, rather than two milliseconds of scalar arithmetic. Earlier grouping bypass, parallel-map and exact early-rejection experiments already exist; their outcomes remain in force. The [B2-only bypass](../2026-10-28-b2-group-only174/README.md) lost 1.285% against OFF, and the [global bypass](../2026-10-28-dedup-off174/README.md) regressed primary and larger proofs. This measurement does not justify repeating either. The [parallel grouping](../2026-10-27-parallel-scalar-groups/README.md) and [prefix cutoff](../2026-10-27-grouping-prefix-cutoff/README.md) reports likewise retain their failed gates.

A distinct candidate to audit next is sequential in-place decomposition at the already bounded G2 caller: it could avoid this fork/join boundary while leaving grouping and query order intact. It trades away decomposition parallelism. No benefit is established here; an actual full-SDK comparison is required.

## Observer qualification

Ryzen 9950X, 32 Rayon workers on CPUs 0–31. Three fixed rotated/reversed accepted/OFF/ON blocks, 31 complete SDK requests after three primes. Solving, fresh randomized proving, SDK verification and serialization are timed. Loading, reset, snapshot and export are outside samples. All captures finish before main-export Gnark verification and decoding. No owned build, helper, hash, archive, independent verifier or other benchmark overlaps captures. No successful run is excluded, extended or repeated.

|Mode|Median of process medians ms|Maximum whole-process RSS MiB|
|---|---:|---:|
|accepted|7.915674|111.50|
|off|8.028948|120.25|
|on|8.079523|123.25|

OFF/accepted block ratios: `[1.0154159018058777,1.0059716618634729,1.0143100890713792]`. ON/OFF ratios: `[1.0129083229999305,1.0048341630225155,1.003037259675863]`; paired median **1.004834163**, or **0.483% overhead**. Calibration and overhead checks both pass. Every OFF/accepted block must lie within factor 1.05, its median within factor 1.03, and median ON/OFF must be ≤1.05. These diagnostic limits do not prove unbiased timing or establish a new accepted performance result. RSS includes load/primes/recorder and is not minimum RAM.

## Fidelity and validation

The diagnostic adds grouping-owner CPU, a possible grouped-point boundary, decomposition boundaries and one serial guard per actual Rayon consumer fold. ON uses map_init with no new grain/chunk/pool/width; OFF retains the original scalar loop. Pinned Rayon 1.12 adapter source delegates splitting, full checks and producer length limits. Controlled splitting/bounds and fold-lifetime/item tests pass. Dynamic stealing can still change under observation.

All 128 unfiltered native core tests and eight prover tests pass. Four recorder cases run on this binary: two inherited test cases and two newly introduced fold tests. Three additional new decoder/interval/ownership tests pass. The analysis.json field new_recorder_tests=4 names the total recorder cases; review.json records that two cases are new in this study. All 17 field files are byte-exact; the 33-field-test qualification is inherited, not rerun. Eight reversible original source edits plus the recorder module produce 176 diagnostic sources; inverse audit recovers all 175 originals. Arithmetic and scheduling parameters remain unchanged.

The initial compile failed E0277 because a closure captured a local !Sync guard. Copying only its numeric parent ID repairs the capture; no unsafe Send/Sync implementation is added. The failed source/pins/logs are retained. During the corrected build, a documentation-only design edit triggered the file-identity guard after SDK linking. Restoring the exact pinned bytes and using resume reused all successful core/prover/library/freeze/link receipts; no successful build/test was repeated. Only the remaining code audit ran.

All 39 capture/verification steps complete: **346 internally verified SDK requests, 19 fresh independently Gnark-verified 164-byte exports**, exact image/public binding, and freshness against 17,811 preceding proof identities. The ledger is **17,830**. All ten enabled graphs, including semantic preflight, are complete with zero drops; OFF graphs are empty. All 15,867 records reconstruct byte-for-byte, totaling 1,269,360 bytes. All 746 final pins close. Fixed 1,024×80-byte records per worker/caller bound recorder payload to 2.579 MiB at 32 workers, plus metadata and snapshots; diagnostic buffers are not production requirements.

Compressed JSON archives use data/encoding/hash entries for both source snapshots, workflow command/exit/stdout/stderr receipts and all 38 proof/public artifact members. Decoded records reconstruct original little-endian 80-byte events. Every member and gzip round trip is checked. Native ELF/library artifacts remain local with SHA identities. No new PMU, AWS/Aeglos, mobile/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput measurement is made.

Publication review retains one trailing blank line in the original test log; authored Rust has no whitespace findings.

## Inherited CPU and Metal comparison

Accepted local measurements against supplied historical M5 references; these are not new all-shape latency measurements. Primary accepted peak remains 134 MiB. The original full-performance-gate failure and physical16 calibration engineering exception remain.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet (0/8).

[Design](artifacts/design.md), [detailed preparation](artifacts/preparation-detail.json), [all observations](artifacts/rows.json), [analysis](artifacts/analysis.json), [review](artifacts/review.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json).
