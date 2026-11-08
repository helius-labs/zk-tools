# Global window-pair reuse screen

Searching every pair of signed GLV windows finds more overlap than the earlier bounded adjacent-window screen. However, the fixed disjoint-pair construction removes only **2.08% / 2.11% of nonzero digit arrivals after charging point formation**, beyond exact full-scalar grouping, on the retained primary/raw2×2 G2 inputs. These are work counts, not latency gains. No native implementation is selected and production is unchanged.

The [earlier bounded screen](../2026-10-21-ryzen-research/sre-bounded-window-opportunity/README.md) capped candidate lists and searched adjacent columns. This study instead checks all 231 pairs of the accepted 22 signed component/window positions, with no candidate cap, including simultaneous sign reversal. It is motivated by [SRE](https://arxiv.org/abs/2610.27452), which studies scalar-window redundancy. Its reported runtime is not transferred to this implementation. This is a broader local opportunity screen, not a reproduction or a new MSM algorithm.

## Counts and scope

Zero scalars and identity bases are excluded. Exact canonical scalar classes are separated first; this deliberately exposes partial reuse beyond whole-scalar equality. The production grouping sampler, admission policy, point sums and cancellation behavior are not executed. Primary production declines global grouping; raw2×2 production admits grouping, so its whole-scalar saving below is already available to that route and is **not a new gain**.

|Quantity|1×2 B2|Raw2×2 B2, diagnostic|
|---|---:|---:|
|Original rows|30,089|52,274|
|Nonzero, nonidentity rows|28,489|49,511|
|Exact nonzero scalar classes|26,142|28,852|
|Original nonzero digit arrivals|613,891|1,064,904|
|Arrivals after exact-scalar grouping|574,356|633,695|
|Point formations for exact-scalar grouping|2,347|20,659|
|Ideal whole-scalar arrival saving, formations charged|37,188|410,550|
|Global pair-ticket upper bound beyond exact classes|84,599|102,034|
|Fixed greedy additional point formations|11,836|13,190|
|Fixed greedy additional net arrivals saved|11,952|13,398|
|Greedy saving / post-group arrivals|2.08%|2.11%|

The fixed greedy rule processes column pairs lexicographically, groups their two nonzero signed digits, pairs consecutive unused row indices, and consumes every compatible common window for each selected pair. Each point participates in at most one new sum. One formation is charged against its shared-window arrival savings. There is no order, width, parameter or subset sweep. Most formations save only one net arrival; the largest match spans all 22 positions, which can include opposite-scalar classes left distinct by exact equality.

The additional G2 sums alone would contain 1,515,008 / 1,688,320 coordinate bytes if retained in ordinary 128-byte affine form. This is a hypothetical payload term, not an implemented allocation, required algorithmic minimum, peak RSS or minimum RAM. Original points, residual codes, sorting/classification, scratch, exceptions and tree geometry add costs. No key copy, point lookup table, retained execution plan or production pool is built.

## What the bound proves

For each signed two-column class with m members, count m−1 optimistic tickets. A one-generation group of m points sharing k nonzero windows saves at most `(m−1)(k−1)` arrivals after formation and supplies `(m−1)·k(k−1)/2` pair tickets. Disjoint groups cannot spend more than m−1 forest edges in one such class. Summing all classes and column pairs therefore bounds that generation. The exact proof and restrictions are in [design.md](artifacts/design.md).

The combined ideal whole-scalar saving plus ticket bound is 121,787 primary arrivals, **19.84% of the raw total**. The achieved fixed construction saves 49,140, including the hypothetical whole-scalar stage. Raw2×2's combined upper is 512,584 (48.13%), dominated by the grouping already present in production. The bounds are loose and are not achieved schedules.

This does not bound recursive reuse of newly created sums, a different scalar representation, arbitrary MSMs, actual generic-add counts or proof speed. Bucket first assignments, point partitions, weighted trees, SIMD occupancy, inversions and cancellation are not priced by arrival subtraction. It cannot establish that any native implementation is faster. The primary count case also supplies no route to halving arrivals within this fixed pipeline. The narrower earlier failure is preserved; it did not prove global reuse impossible.

No native candidate is selected from this result. A future reuse implementation would need an explicit cost/resource case for point formation and residual routing, and a prospective complete-query comparison. The prior failed grouping and coalescing implementations remain closed. The next search should target substantial main-MSM arithmetic or dependency removal rather than treat these optimistic tickets as a measured gain.

## Validation and provenance

Four new Rust1.98.1/xtask commands pass on first execution: compile the new test, run it, compile the observer and execute the two-fixture screen. The test checks all **531,441** four-point/three-window matrices over −1/0/1 against every signed two-point gain and disjoint two-edge matching optimum. It does not exhaustively test arbitrary larger groups; their bound relies on the stated forest argument.

The exact qualified split/Booth helper is reused. Every real scalar receives component integer reconstruction and Fr/eigenvalue reconstruction. Four independent weighted Fr token checks validate residual-plus-created-sum conservation for the greedy outputs. These are linear algebra checks using the accepted field arithmetic, not elliptic-curve or cryptographic proof verification.

A separate reader verifies all 208 source/input/tool pins, four command receipts and stream hashes, the unchanged 37,023-proof ledger, and all 462 pair rows. It uses ordered-map frequencies independently of the observer's sorted run lengths. Recoding is shared; the reader does not replay the greedy construction or independently implement field arithmetic. Its first compiler invocation misspelled the existing field-library path (`…3221fcef…` instead of `…3221f1cef…`) and exited1; correcting only that command argument compiled the unchanged reader successfully. The tool transcript retains that invocation. The largest individual pair-key vector capacity payload is 128KiB in each case; other observer allocations are excluded, so this is not a complete diagnostic or native memory bound. There are no new timings, profiles, SDK requests or proofs.

An earlier preparation mistake in this turn is retained: the already-completed τ⁹ census was initially mistaken for unfinished work. Its setup guard exited101 before rerunning a census. The overwritten private design note was restored to its original SHA256 `a9d127405d383d6424e6bde22e5f600e6cbd63ea31f3b4dee2d8288c9c0a8309`; prior results were unchanged. The intercepted source and an explicitly labeled tool-observation record are archived. The [completed τ⁹ rejection](../2026-11-08-tau9-ring-census176/README.md) remains authoritative. The new global-pair screen itself has no failed scientific command or replay.

## Inherited full-proof comparisons

Accepted production176/SDK6d83 remains unchanged. These are the latest saved local32 full-proof observations from the [tree-policy SDK comparison](../2026-11-08-tree-policy-sdk176/README.md), not measurements from this count screen. The same accepted binary previously measured 7.915ms on primary; private 7.860–7.869ms candidates were not adopted. Latest versus earlier observations alone do not establish a code regression.

|Shape|Local ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.040649|37 /4.602×|24 /2.985×|3.7|
|2×2|11.149097|59 /5.292×|30 /2.691×|5.9|
|4×2|16.069415|109 /6.783×|48 /2.987×|10.9|
|8×2|24.131178|189 /7.832×|74 /3.067×|18.9|
|12×2|33.940628|284 /8.368×|101 /2.976×|28.4|
|16×2|40.164083|335 /8.341×|124 /3.087×|33.5|
|24×2|66.187704|528 /7.977×|176 /2.659×|52.8|
|merge24×1|65.361571|507 /7.757×|175 /2.677×|50.7|

Merge54 remains160.890372ms without a supplied M5 reference. Verified single-proof CPU≥10×M5 CPU on all eight supplied shapes remains **active and unmet, 0/8**. No fresh AWS/Aeglos, huge-circuit, mobile, pressure/swap, minimum-RAM or concurrent-proof result is added.

[Counts](artifacts/analysis.json), [reader result](artifacts/review.json), [source](artifacts/study.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [exact evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) preserve the result. Every UTF-8 member hash and gzip roundtrip is verified. Private scalar/point/key/witness/proof arrays, binaries and unchanged full production source are excluded. Helper compilation/preparation/publication are recorded by the tool transcript rather than claimed as additional xtask receipts. Private root: `/home/ubuntu/src/zopro-perf-work/global-window-pair-bound176`.
