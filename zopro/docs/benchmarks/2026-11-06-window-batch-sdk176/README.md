# Window batch contract: native SDK correctness

Both fresh SDK builds pass real-shape correctness checks. There are **22 new independently verified, unique proof exports** from **88 internally verified SDK requests**. No full-proof performance result or production adoption is claimed.

The [component study](../2026-11-06-window-batch-capture176/README.md) found faster G1 and slower G2 paths and failed28/76 checks. Its explicit exception to the component stop rule selects this first full-proof study of the same combined candidate across every shape. The old failed gate is unchanged. The [prospective full-proof protocol](artifacts/design.md) was written before any new SDK request; correctness-run clocks are excluded from that future timing comparison.

## Native bridge and correctness

The exact qualified OFF/ON core libraries and original accepted field1200 are reused. Original Prover and Zolana wrapper sources come from the same qualified snapshot. The standard cpu-sdk.rs driver and perf support match the accepted SDK's pinned source bytes. Both final SDK links use optimization, target-cpu=native, fat LTO and one codegen unit. No field/core rebuild or replay of their successful suites occurs.

All ten new native commands succeed once: two Prover libraries, two Zolana libraries, two final SDK links, and builds/runs of the ON Prover unit and cubic integration tests. The two raw-format and six cubic tests pass unfiltered. They reapply inherited cases to changed coherent libraries; they are not eight new mathematical cases. The qualified core's129 tests and original field tests are inherited.

Real proofs cover all eight supplied shapes plus merge54 at32 workers, and primary1×2 at1and16 workers. Both fresh arms run each context. Each process makes three internally verified primes and one internally verified request, exporting only its final proof/public pair. An existing pinned Gnark verifier independently checks each export against the exact image and public inputs. VK hashes match previously verified image-bound receipts. Export hashes are unique against each other and the prior30,846-export ledger. The successor ledger contains30,868 exports. Accepted SDK correctness is inherited; no accepted preflight is replayed for bookkeeping.

| Qualification | Result |
|---|---:|
| New native commands | 10 successful |
| Reapplied ON Prover tests | 2 raw-format + 6 cubic passed |
| New real-shape SDK processes | 22 |
| New internally verified SDK requests | 88, including66 primes |
| New independent Gnark verifications | 22 |
| Prior / successor unique export ledger | 30,846 / 30,868 |
| New performance observations used for decisions | 0 |

The native and preflight workflows complete on their first execution; no SDK proof, successful build, test or link is repeated. Preparation repairs are retained: the initial unexecuted link plan gained its missing libc extern; the first reviewer assumed Cargo variables appeared in runtime receipts; the second reviewer completed native review but assumed a nine-row historical case file, which also contains a primary16-worker row. The continuation reuses completed native review/identities and selects the declared nine32-worker rows plus two primary contexts. None of these repairs follows a measured performance result. Full sources and failed helper logs are archived.

Receipt checks cover exact argv, recorded managed environment, successful exit and stream hashes. Cargo manifest/package variables remain in the pinned invocation plan; the runner omits them from command.json, so independent receipt coverage of those variables is not claimed. The pinned current runner source explains that schema; it does not independently establish the build provenance of the existing runner executable.

## Scope and next measurement

Production is unchanged. The candidate adds no retained table, key copy, execution plan, scratch buffer or thread pool. Native correctness does not establish portability, low-memory behavior or minimum RAM; those remain required before adoption. This stage adds no AWS, Aeglos, phone, huge-circuit, swap, pressure or throughput result and creates no cloud resource.

The already-declared next study uses the exact accepted SDK and fresh OFF/ON builds:11 contexts,15 matched rotating/reversing blocks,31 timed requests after3 primes,495 latency processes, then12PMU panels and12profiles. All39 declared performance checks and every context remain; no favorable subset, parameter rescue or adoption follows from component results. Collection helpers and review/ledger handling must be frozen before launch. The full all-eight CPU10× goal remains active and unmet0/8.

## Accepted comparison, inherited

These32-worker local CPU times and supplied M5 references are unchanged. They are not new timings from this qualification.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU10× target ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 24 | 3.7 |
| 2×2 | 11.151932 | 59 | 30 | 5.9 |
| 4×2 | 16.086386 | 109 | 48 | 10.9 |
| 8×2 | 24.179453 | 189 | 74 | 18.9 |
| 12×2 | 33.884455 | 284 | 101 | 28.4 |
| 16×2 | 40.077381 | 335 | 124 | 33.5 |
| 24×2 | 65.850777 | 528 | 176 | 52.8 |
| merge24 | 65.060937 | 507 | 175 | 50.7 |

Merge54 remains161.883899ms, without a supplied M5 reference. Inherited1×2RSS132.75MiB is neither a new measured peak nor minimum RAM. The accepted SDK hash is6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661.

[Native review](artifacts/native-review.json), [proof review and hashes](artifacts/preflight-review.json), [identities](artifacts/identities.json), [correctness protocol](artifacts/preflight-protocol.md), [preparation repairs](artifacts/prepare-repair.md), [archive index](artifacts/archive-index.json). The archive contains exact UTF-8 source/helper/protocol/receipt bytes and hashes, with extraction checks. Executables, libraries, key images, private witnesses and full ledger payloads are excluded. Local root: `/home/ubuntu/src/zopro-perf-work/window-batch-sdk176`.
