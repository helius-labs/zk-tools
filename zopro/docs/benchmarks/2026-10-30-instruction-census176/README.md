# Current accepted CPU instruction attribution

Offline analysis of four existing qualified primary captures completes with **42,495 samples**, weighted period **679,920,000,000**, and exact sample conservation. These are the 16/32-worker captures for the accepted 176-file SDK and its previous accepted control. No new proof, test suite, native prover build, timing or profile recording occurs. Production remains unchanged; no new optimization is adopted. The distinct verified-export ledger remains 27,353.

The existing Rust xtask reader binds disassembly bytes to each executable, translates recorded MMAP2 addresses through ELF load segments and requires exact sampled instruction boundaries. A separate audit independently reconciles every raw sampled IP, PID, actual TID set, family/memory/scope/function bin and total. There are 41,709 executable samples and 786 outside the executable; external samples remain explicit. Each decoded sample retains the fixed 16M period. All 15 extraction commands and the attribution command complete without a native capture or successful-test rerun.

The exact executables and original `perf.data` files remain local at the hash-pinned configuration paths. Extracted streams, disassembly, symbol ranges, command receipts and attribution outputs are archived here. The first archive review reached the link check before `SHA256SUMS` had been created; its log is retained. Creating that file completed the report, without repeating extraction, attribution, tests or measurements.

|Capture|All samples|Executable samples|IFMA % of all|Scalar move % of all|RSP-operand % of all|
|---|---:|---:|---:|---:|---:|
|1x2-w16-prior|8896|8744|22.36|14.66|10.17|
|1x2-w16-current|8746|8572|22.49|13.88|8.63|
|1x2-w32-prior|12534|12301|18.80|20.38|9.89|
|1x2-w32-current|12319|12092|18.82|19.00|9.29|

## Current 32-worker hot contexts

|Context|Samples|% of all|IFMA % within context|Scalar move % within|RSP-operand % within|
|---|---:|---:|---:|---:|---:|
|G1 stream|2476|20.10|40.47|1.37|4.56|
|G2 borrowed add|1894|15.37|42.29|1.74|10.45|
|G1 tree reduce|643|5.22|30.95|5.75|13.69|
|G2 tree reduce|731|5.93|14.64|21.75|27.91|
|Epoch pin context|1006|8.17|0.00|82.80|0.00|

Instruction families and memory tags are overlapping views, not additive costs. Sampling can skid; inlining and aliases affect context labels. RSP operands do not establish spills, actual traffic or removable latency. Scalar-move samples can include register moves and memory accesses. The lower scalar-move share in current borrowed G2 is consistent with the adopted removal of scalar Y-sign work, but these single profile observations do not isolate that effect or replace the measured full-proof result.

The generic G2 tree reducer has more RSP-tagged samples within its own context than borrowed G2 addition. Source inspection finds output returned through canonical component arrays, interleaved into point arrays and scattered into the tree. This motivates inspecting a direct output-store path while keeping input packing, arithmetic, pair scheduling and topology unchanged. That path is not implemented or timed here. The G2 reducer is only 5.93% of this profile, so its stack-tag percentage is not a credible 2× whole-proof opportunity. A native candidate needs an actual distinct mechanism and complete threaded-caller evidence before any SDK promotion.

## History and papers

The [history audit](artifacts/history-audit.md) keeps prior failures closed: direct linear-wire/query-rank reductions, bounded/global/partial scalar-window reuse, queue replacement, partition/arena changes, joint G1 coordinate I/O, staged kernels and tested R260 layouts. [SRE](https://arxiv.org/abs/2610.27452) and the [Waterloo thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403) were already considered in recorded local experiments. Their results do not establish a gain for this native prover. The rejected joint G1 input/output transpose is distinct from an output-only G2-tree hypothesis, but remains a required negative reference. No unchanged candidate is rerun.

## Accepted proof results, unchanged

These are the prior measured accepted 32-worker full-proof results and supplied historical M5 references, not timings from this diagnostic. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) records the native flags, actual SDK, correctness and resource scope.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2x2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4x2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8x2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12x2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16x2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24x2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge-24x1|65.060937|507 / 7.793×|175 / 2.690×|50.7|
|merge-54x1|161.883899|—|—|—|

The original all-eight 10× CPU objective remains active and unmet, **0/8**. Primary needs 3.7 ms versus accepted 7.928660 ms. This diagnostic adds no AWS/Aeglos, phone/ARM/TEE, huge-circuit, swap, minimum-RAM or concurrent-throughput result. It introduces no lookup table, retained point copy, execution plan or prover allocation.

[Design](artifacts/design.md), [findings](artifacts/findings.json), [independent review](artifacts/independent-review.json), [archive review](artifacts/archive-review.json), [extraction plan](artifacts/extraction-plan.json), [attribution configuration](artifacts/analysis-config.json), [extraction receipts](artifacts/extraction.tar.gz), [attribution results and per-IP bins](artifacts/analysis.tar.gz), [attribution workflow](artifacts/analysis-workflow.tar.gz), [archive file hashes](artifacts/archive-file-hashes.json), [checksums](SHA256SUMS).
