# G2 reverse-loop stack-demand diagnostic

**No native change is selected.** The rejected factor-lookahead implementation has less peak loop-internal explicit stack demand, but its measured complete queries did not improve. This offline diagnostic reuses the exact terminal comparison's six G2 profiles and OFF/ON binaries; it adds no recording, native build, timing, SDK call or proof. Accepted local 1×2 remains **7.829286 ms**, with **7.803404 ms** observed earlier from the same executable.

A source count of forty coordinate vectors does not prove forty simultaneous hardware-register lifetimes. The new analysis follows actual control flow and direct constant-RSP memory demand. It finds the reverse full-group strongly connected component rather than treating cold backward branches as independent loops. All six selected profile populations map to exact instruction starts.

|Actual linked consumer|OFF|Rejected lookahead ON|
|---|---:|---:|
|Whole-symbol instruction sites|8591|8997|
|Whole-symbol IFMA sites|1450|1650|
|Reverse-SCC instruction sites|3590|3637|
|Reverse-SCC IFMA sites|1200|1200|
|Peak internal direct-stack byte demand|4200|3624|
|Same-basic-block direct store/read pairs|51|51|
|Longest paired instruction distance|775|780|

The ON whole-symbol IFMA count includes a peeled initial two-product factor preparation outside its reverse SCC. This is static duplication, not 200 additional IFMAs per group. Peak direct demand falls **13.71%**, while the inherited complete-query panels report about **0.5% more G2 instructions** and no accepted gain. The two byte-demand maxima need not occur at corresponding arithmetic points. They are not whole-stack peaks, register counts, memory peaks or minimum RAM.

|Retained cycle profile|Reverse samples|Direct-stack operand samples / share|Stack move samples|Stack-operand IFMA samples|
|---|---:|---:|---:|---:|
|cycles-1x2-B-w16-OFF|2106|210 / 9.97%|142|0|
|cycles-1x2-B-w32-OFF|2720|301 / 11.07%|200|0|
|cycles-2x2-B-w32-OFF|2852|302 / 10.59%|184|0|
|cycles-1x2-B-w16-ON|2079|260 / 12.51%|173|0|
|cycles-1x2-B-w32-ON|2683|298 / 11.11%|188|0|
|cycles-2x2-B-w32-ON|2831|348 / 12.29%|230|0|

Samples landing on direct stack operands cover **9.97–12.51%** of these reverse-loop populations. There are no sampled reverse-loop IFMAs with direct stack operands. This does not mean stack loads cannot delay arithmetic: sampling skid, operand dependencies and out-of-order execution prevent causal attribution. These shares are not retired instruction frequencies, exclusive wall time or removable latency fractions. Existing ordinary-memory operands and all unselected samples remain accounted for.

The liveness model takes byte uses/kills for constant direct `[rsp+offset]` operands and solves a least fixed point over each selected SCC. Demands outside that SCC and indirect/alias effects are excluded. Masked stores do not kill all bytes. LEA/NOP are address calculations. The selected SCCs contain no calls or direct RSP address-taking. Named direct noreturn-call contracts are part of the CFG. RBP is not assumed to be a frame pointer. Constants, deliberate local data and compiler spills are all included; they are not separated by this model.

The long store/read pairs have no intervening control-flow boundary or overlapping direct write. Their debug markers mostly name field/SIMD intrinsic definitions. Those markers do **not** establish that a value is Ax, Ay, Bx, By or a specific source variable. No coordinate provenance, complete alias analysis or physical-register allocation proof is claimed. A blanket spill/packing/inline rewrite is therefore not selected. Prior layout, lazy-Y, cached-X/denominator, staged and lookahead negatives stay closed.

The separate same-agent reader checks **17,588 instruction rows**, **18,326 selected-symbol samples**, **15,271 reverse-SCC samples**, all 102 straight-line pairs and **7,227 byte-demand fixed-point rows**. It identifies SCCs independently by forward/reverse reachability and recomputes liveness with a predecessor worklist from empty. It checks current production's **176 hashes**, the eleven recorded source-file hashes and 140 analysis pins. The publisher additionally checks six ELF LOAD/MMAP2 biases, the reader's own terminal receipts and 153 review pins. Limited debug-line markers are retained exactly; third-party source text is not embedded.

There are **16 serialized xtask steps: 14 successes and two retained analysis failures**. Initial lexical loop selection was ambiguous; the first CFG repair missed noreturn `slice_index_fail`. The successful repair adds that contract and broadcast-width handling, then passes four parser tests and analysis. A first direct reviewer attempt also stops on its incorrect equal whole-symbol IFMA-count assumption; the corrected final reader passes. Preparation/display/direct-review attempts are separate from the 16 steps. Tooling notes retain the additional failed preparation and absent-plan invocation. The initial overwritten audit source is hash-identified but not separately snapshotted. Saved decode streams, failures, final source and available pre-repair snapshots are included. No successful analysis or performance capture is replayed.

The inherited factor-lookahead primary32 ON/OFF ratio is **0.999361 [0.983690, 1.010566]**, 8/15 wins; ON/REF is **1.002719 [0.992802, 1.019546]**, 6/15. Its original complete-query decision remains rejected. Smaller explicit stack demand alone does not explain or reverse it. This diagnostic supplies no new twofold gain mechanism.

Accepted full-proof observations and supplied historical M5 references follow. They are unchanged local 32-worker measurements, not new AWS results.

|Shape|Accepted CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1x2|7.829286|37 / 4.726×|24 / 3.065×|3.7|
|2x2|10.979335|59 / 5.374×|30 / 2.732×|5.9|
|4x2|15.803969|109 / 6.897×|48 / 3.037×|10.9|
|8x2|23.944017|189 / 7.893×|74 / 3.091×|18.9|
|12x2|33.703579|284 / 8.426×|101 / 2.997×|28.4|
|16x2|39.858478|335 / 8.405×|124 / 3.111×|33.5|
|24x2|66.186709|528 / 7.977×|176 / 2.659×|52.8|
|merge-24x1|65.489902|507 / 7.742×|175 / 2.672×|50.7|

Primary peak RSS remains **133.50 MiB**; 16 workers **9.192237 ms**. Merge54 remains **162.541995 ms**, with no supplied M5 reference. Accepted SDK/source176 and canonical **43,730** verified proofs remain unchanged. CPU AVX 10× for all eight stays **active and unmet, 0/8**. No new AWS/Aeglos, mobile/ARM, huge-circuit, pressure/swap, minimum-RAM or concurrent-throughput result follows.

[Review](artifacts/review.json), [method](artifacts/design.md), [repair record](artifacts/repair.md), [tooling notes](artifacts/tooling-notes.md), [decision](artifacts/decision.json), [continuation and autoresearch loop](artifacts/next.md), [full instruction/CFG/liveness/sample data, sources and receipts](sources-receipts.tar.gz), [archive index](artifacts/archive-index.json), [checksums](SHA256SUMS). Private root: /home/ubuntu/src/zopro-perf-work/g2-stack-lifetimes176.

[Publication finalization](artifacts/publication-repair.md), [finisher source](artifacts/finish-publication.rs).
