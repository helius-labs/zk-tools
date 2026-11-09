# First-48 Montgomery cancellation screen

**The exact arithmetic passes; this fixed recipe is closed before native implementation.** Moving the 48-bit cancellation ahead of four 52-bit cancellations retains the same Montgomery quotient and output, but adds multiplier repacking. With that cost included, both source dependency scores worsen. No proof speedup or measured slowdown follows from this model. Production176 and the accepted SDK remain unchanged.

The candidate keeps radix52 input/output limbs and logical Montgomery factor R=2^256. Only the multiplier is recoded into widths [48,52,52,52,52]; fused products recode both multipliers. The first row uses the accepted canceled-low carry identity and mixed48 coefficient repacking, followed by four normal rows. The accepted order is [52,52,52,52,48]. Both orders still need one mixed48 row. Moving it does not eliminate its work, and the replacement multiplier codec costs sixteen source vector operations per operand.

This differs from earlier split product/reduction planes, two-digit quotient lookahead, inverse-radix residue folding, final48 packing and late-centered output trials. Their outcomes remain closed. No order/width/layout sweep, favorable-operation promotion or unchanged rerun is selected. The proposed primitive adds no table, retained point/key/scalar copy, solver plan, heap buffer or pool; the mathematical helper's BigUint allocations are offline diagnostic storage.

## Exact arithmetic and source model

Both BN254 fields pass **115,712 output evaluations and 578,560 row-state checks** in total. Counts include both orders, all three operation modes and repeated corner values; they are not unique field values or proofs. Per field, the fixed cohorts contain 4,096 inclusive [0,2p] corner tuples, 768 reduced power/neighbor tuples, 4,096 deterministic full-width tuples and 256 eight-step dependent chains. Ordinary, sum2 and positive-complement difference2 outputs agree between orders, with normalized digits and the original strict below-2p result bound after fused correction.

Each row is checked independently from the original operands: after k consumed bits, the reconstructed coefficient state equals `(a*(b mod 2^k) + c*(d mod 2^k) + p*q_prefix)/2^k`, omitting the second product for ordinary multiplication. Divisibility and codec reconstruction are checked. Final quotient/output agree with direct full-R BigUint REDC and logical Arkworks field arithmetic. No invalid raw canonical Fp object is constructed. Observed post-row coefficient maxima are 58,695,267,012,817,701 for Fq and 62,760,934,483,373,531 for Fr. They are observed maxima, not general bounds or CPU register values.

For a conservative range argument, at most two product and one reduction stream contribute two radix52 halves per coefficient per row. Five rows contribute at most 30 radix units before canceled-low carries. Mixed48 repacking bounds the low four outputs by one radix unit plus a small carry; its unmasked high output admits at most 48 units. Adding remaining row contributions and carries stays below the deliberately loose 256-unit (2^60) envelope used by the interpreter. Products are evaluated in u128; intended low-word quotient multiplication is wrapping. This is a mathematical/source bound, not native code qualification.

|Operation|Accepted IFMA sites|First48 IFMA sites|Accepted / First48 ALU sites|Accepted / First48 output-ready score|
|---|---:|---:|---:|---:|
|Ordinary|100|100|50 / 66|102 / 104|
|Fused sum/difference body|150|150|50 / 82|136 / 140|

The source SSA model assigns IFMA weight4 and vector ALU/shift weight2, informed by the earlier [local instruction calibration](../2026-11-06-instruction-calibration176/README.md). Weight2 for all logical operations is an explicit model assumption, not a separately measured latency for every opcode. Constants/inputs start ready. Both multiplier codecs, cancellation rows, mixed48 repacking and final carries are charged. Common fused correction, difference-operand complement, external packing/get, scheduling, throughput, register allocation/spills and compiler simplification are excluded. These scores are neither hardware cycles nor proving-time estimates. The necessary gate required strictly better scores for both operations with no added IFMAs; it fails both score checks. No native experiment is selected.

A separately written whole-integer reduced-radix reader checks 178,839 ordinary pairs across odd moduli3..63 and 67,667 fused tuples across moduli3/5/7, each in both orders, against a direct full-R quotient. It uses no producer limb arrays or canceled-low identity. It also checks cohort conservation, source operation counts, all189 pinned files, successful command exits and stream hashes. The dependency graph itself is shared with the producer, not independently reproduced. This is same-agent review, not an external audit.

The first compiler invocation fails on two ambiguous BigUint shift types. Explicit usize annotations fix compilation before the first execution; original source, failure and pins remain. One earlier source draft was corrected before compilation to avoid closure borrowing and to check the difference sign independently. The repaired model builds and executes once, then the review builds and executes once: four successful xtask commands and one retained failed compiler command. A manual preliminary review compile was followed by stronger exit/stream validation before the recorded review build. Bootstrap/publication compiler invocations have tool-transcript evidence. No successful model, native suite, proof capture or verifier is replayed. An unused-import warning is retained.

## Accepted proof comparisons, inherited

These are the previous campaign's **fixed-worker-placement accepted-library observations**, not timings of this candidate or an adopted pinning default. Supplied M5 CPU/Metal references come from different hardware. The same accepted standard build previously measured **7.915095 ms** at1×2; its latest separate unpinned reference was8.075143 ms. Fixed1×2 was7.869302 ms. Sub-8 ms remains a valid observation; no code regression or new latency record is established here.

|Shape|Accepted fixed CPU ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.869302|37 / 4.702×|24 / 3.050×|3.7|
|2×2|10.984868|59 / 5.371×|30 / 2.731×|5.9|
|4×2|15.734200|109 / 6.928×|48 / 3.051×|10.9|
|8×2|23.929023|189 / 7.898×|74 / 3.092×|18.9|
|12×2|33.722646|284 / 8.422×|101 / 2.995×|28.4|
|16×2|39.776845|335 / 8.422×|124 / 3.117×|33.5|
|24×2|66.183856|528 / 7.978×|176 / 2.659×|52.8|
|Merge24×1|65.448733|507 / 7.747×|175 / 2.674×|50.7|

The [full preceding report](../2026-11-09-weighted-prefix-fixed-sdk176/README.md) contains all controls and resources; its rejected weighted-tree candidate remains rejected. Merge54 fixed accepted remains162.489760 ms without a supplied M5 reference. This screen generates zero SDK requests or Gnark exports; the canonical inventory remains38,668. No CPU profile, native caller, AWS/Aeglos, mobile, huge-circuit, pressure/swap, RAM peak/minimum or concurrent-throughput result is added. The verified all-eight CPU10× objective stays active and unmet, **0/8**.

[Fixed design](artifacts/design.md), [mathematical/source results](artifacts/result.json), [reader result](artifacts/review.json), [producer](artifacts/model.rs), [reader](artifacts/review.rs), [pins](artifacts/pins.json), [sources and command evidence](artifacts/evidence.json.gz), [publication checks](artifacts/publication.json), [checksums](SHA256SUMS). The archive retains UTF8 member bytes/hashes and checked gzip roundtrip. Executables/libraries, private proof inventory, keys, witnesses and papers are excluded. Reproduction requires pinned dependencies and adjusted absolute paths.
