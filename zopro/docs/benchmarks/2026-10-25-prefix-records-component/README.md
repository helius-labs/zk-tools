# Prefix-only streaming records: mixed component result

The combined G1/G2 design fails its declared component gate and is not integrated into the prover. G1 slows by3.2–3.6%, while G2 improves by6.1% in every paired block. All negative results remain part of the decision; a possible G2-specific follow-up requires a separate prospective SDK experiment.

| Kernel/batch | Old ns/batch | New ns/batch | Paired new/old | Winning blocks | Old record capacity B | New record capacity B | Pass |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| g1-b200 | 4027.68 | 4156.63 | 1.031952 | 0/31 | 40,000 | 8,000 | False |
| g1-b352 | 6598.81 | 6839.62 | 1.036209 | 0/31 | 70,400 | 14,080 | False |
| g2-b200 | 7848.94 | 7367.96 | 0.938905 | 31/31 | 72,000 | 16,000 | True |

The geometric ratio across the three cases is1.001328. The original gate requires each case below1 with at least21/31 wins and an aggregate below1. Both G1 cases have zero wins. That combined gate remains failed; the G2 result does not waive it.

The accepted streaming kernel stores coordinates and scaled numerators for reverse evaluation. This private design instead stores one vector prefix for G1 and a prefix/norm pair for G2. It rereads original bucket and input coordinates on reverse. A batch contains distinct bucket indices, so a reverse write cannot alter another group's inputs. Scalar-tail blocks are reused verbatim and retain their original inversion handling. G1 uses the same number of products with additional coordinate packing; G2 combines prefix and inverse once before scaling both numerator components, removing one elementary Fq product per full point. This changes arithmetic order and memory access together, so the timings do not isolate either effect.

G1 record payload falls from five vectors to one (80%); G2 falls from nine to two (77.8%). Reported capacities cover only group-record vectors after priming. They exclude bucket arrays, scalar tails, allocator overhead, library state and all other memory. There is one additional empty vector header in each private scratch type, and old group vectors remain unallocated on the candidate path. This is not peak process RSS, minimum RAM or full-prover memory qualification.

Native validation passes all106 core tests, including two new tests against complete scalar curve formulas. New cases cover signed/unsigned G1, signed-point G2, zero length, short vectors, tails, several batch sizes, both canonical and inversion settings, and three scratch reuses. They perform396 full-buffer comparisons. Candidate checks assert that the original group vector stays unallocated. Existing field primitives are unchanged.

Thirty-one fixed rotated/reversed blocks compare direct calls to old and new kernels in one native fat-LTO executable pinned to CPU0. Representative batches are200 and352 for G1 and200 for G2, selected before compilation using the [operation census](../2026-10-25-operation-model/README.md). Every sample updates the same valid bucket geometry256 times after three prime calls; point generation, allocation setup and full output checksums are outside timing. All93 paired full-bucket checksums agree. The original G1 control uses the accepted reserve/write storage mode. No build or other owned CPU benchmark overlaps measurement.

No normal SDK dispatch was changed. This component generates zero SDK or Gnark proofs and establishes no full-request latency improvement, portable behavior, memory-pressure feasibility, concurrency, large-circuit result, AWS or phone performance. Accepted production remains the same167 files and historical8.689205ms primary. The all-eight10x M5 CPU objective remains unmet. No source is adopted or pushed.

The G1 result shows that reducing source record bytes alone does not guarantee faster execution. The G2 result motivates a distinct question: whether a G2-only path preserves a benefit in complete fresh proofs while G1 stays on the accepted implementation. That investigation would retain this failed combined result, declare fresh controls and guards before SDK measurement, and require memory and compatibility qualification before adoption.

[Protocol](protocol.json), [results](results.json), [raw observations](measurements/rows.json), [scalar-tail audit](tail-reuse-audit.json), [source](source/crates/zopro-core/src/msm/prefix_records.rs), [driver](component.rs), [build](build-complete.json).
